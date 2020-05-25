//! A watcher based on the k8s API.

use super::{
    client::Client,
    stream as k8s_stream,
    watch_request_builder::WatchRequestBuilder,
    watcher::{self, Watcher},
};
use futures::{
    future::BoxFuture,
    stream::{BoxStream, Stream},
};
use http02::StatusCode;
use hyper13::Error as BodyError;
use k8s_openapi::{WatchOptional, WatchResponse};
use snafu::{ResultExt, Snafu};

/// A simple watcher atop of the Kubernetes API [`Client`].
pub struct ApiWatcher<B>
where
    B: 'static,
{
    client: Client,
    request_builder: B,
}

impl<B> ApiWatcher<B>
where
    B: 'static,
{
    /// Create a new [`ApiWatcher`].
    pub fn new(client: Client, request_builder: B) -> Self {
        Self {
            client,
            request_builder,
        }
    }
}

impl<B> ApiWatcher<B>
where
    B: 'static + WatchRequestBuilder,
    <B as WatchRequestBuilder>::Object: Send + Unpin,
{
    async fn invoke(
        &mut self,
        watch_optional: WatchOptional<'_>,
    ) -> Result<
        impl Stream<
                Item = Result<
                    WatchResponse<<B as WatchRequestBuilder>::Object>,
                    k8s_stream::Error<BodyError>,
                >,
            > + 'static,
        watcher::invocation::Error<invocation::Error>,
    > {
        // Prepare request.
        let request = self
            .request_builder
            .build(watch_optional)
            .context(invocation::RequestPreparation)?;
        trace!(message = "request prepared", ?request);

        // Send request, get response.
        let response = self
            .client
            .send(request)
            .await
            .context(invocation::Request)?;
        trace!(message = "got response", ?response);

        // Handle response status code.
        let status = response.status();
        if status != StatusCode::OK {
            let source = invocation::Error::BadStatus { status };
            let err = if status == StatusCode::GONE {
                watcher::invocation::Error::Desync { source }
            } else {
                watcher::invocation::Error::Other { source }
            };
            Err(err)?;
        }

        // Stream response body.
        let body = response.into_body();
        Ok(k8s_stream::body(body))
    }

    async fn invoke_boxed_stream(
        &mut self,
        watch_optional: WatchOptional<'_>,
    ) -> Result<
        BoxStream<
            'static,
            Result<WatchResponse<<B as WatchRequestBuilder>::Object>, k8s_stream::Error<BodyError>>,
        >,
        watcher::invocation::Error<invocation::Error>,
    > {
        let stream = self.invoke(watch_optional).await?;
        Ok(Box::pin(stream))
    }
}

impl<B> Watcher for ApiWatcher<B>
where
    B: 'static + WatchRequestBuilder + Send,
    <B as WatchRequestBuilder>::Object: Send + Unpin,
{
    type Object = <B as WatchRequestBuilder>::Object;

    type InvocationError = invocation::Error;

    type StreamError = k8s_stream::Error<BodyError>;
    type Stream = BoxStream<'static, Result<WatchResponse<Self::Object>, Self::StreamError>>;

    fn watch<'a>(
        &'a mut self,
        watch_optional: WatchOptional<'a>,
    ) -> BoxFuture<'a, Result<Self::Stream, watcher::invocation::Error<Self::InvocationError>>>
    {
        Box::pin(self.invoke_boxed_stream(watch_optional))
    }
}

pub mod invocation {
    //! Invocation error.
    use super::*;

    /// Errors that can occur while watching.
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub))]
    pub enum Error {
        /// Returned when the call-specific request builder fails.
        #[snafu(display("failed to prepare an HTTP request"))]
        RequestPreparation {
            /// The underlying error.
            source: k8s_openapi::RequestError,
        },

        /// Returned when the HTTP client fails to perform an HTTP request.
        #[snafu(display("error during the HTTP request"))]
        Request {
            /// The error that API client retunred.
            source: crate::Error,
        },

        /// Returned when the HTTP response has a bad status.
        #[snafu(display("HTTP response has a bad status: {}", status))]
        BadStatus {
            /// The status from the HTTP response.
            status: StatusCode,
        },
    }

    impl From<Error> for watcher::invocation::Error<Error> {
        fn from(source: Error) -> Self {
            watcher::invocation::Error::Other { source }
        }
    }
}
