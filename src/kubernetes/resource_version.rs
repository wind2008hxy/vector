//! A resource version types to ensure proper usage protocol.

use k8s_openapi::apimachinery::pkg::apis::meta::v1::{ObjectMeta, WatchEvent};
use k8s_openapi::Metadata;
use std::ops::Deref;

/// Resource version state in the context of a chain of watch requests.
#[derive(Debug, Clone)]
pub struct State(Option<String>);

impl State {
    /// Create a new resource version [`State`].
    pub fn new() -> Self {
        Self(None)
    }

    /// Update the resource version from a candidate obtained earlier.
    ///
    /// Returns the previous state.
    pub fn update(&mut self, candidate: Candidate) -> Option<String> {
        self.0.replace(candidate.0)
    }

    /// Reset the resource version. Use in case of a desync.
    ///
    /// Returns the previous state.
    pub fn reset(&mut self) -> Option<String> {
        self.0.take()
    }

    /// Get the current resource version value.
    pub fn get(&self) -> Option<&str> {
        self.into()
    }
}

impl Deref for State {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<&'a State> for Option<&'a str> {
    fn from(val: &'a State) -> Self {
        match val.0 {
            Some(ref val) => Some(val.as_str()),
            None => None,
        }
    }
}

/// A resource version candidate, can be used to update the resource version.
pub struct Candidate(String);

impl Candidate {
    /// Obtain a resource version [`Candidate`] from a [`WatchEvent`].
    pub fn from_watch_event<T>(event: &WatchEvent<T>) -> Option<Self>
    where
        T: Metadata<Ty = ObjectMeta>,
    {
        let object = match event {
            WatchEvent::Added(object)
            | WatchEvent::Modified(object)
            | WatchEvent::Deleted(object)
            | WatchEvent::Bookmark(object) => object,
            WatchEvent::ErrorStatus(_) | WatchEvent::ErrorOther(_) => return None,
        };
        Self::from_object(object)
    }

    /// Obtain a resource version [`Candidate`] from a object of type `T`.
    pub fn from_object<T>(object: &T) -> Option<Self>
    where
        T: Metadata<Ty = ObjectMeta>,
    {
        let metadata = match object.metadata() {
            Some(val) => val,
            None => {
                warn!(message = "Got k8s object without metadata");
                return None;
            }
        };

        let new_resource_version = match metadata.resource_version {
            Some(ref val) => val,
            None => {
                warn!(message = "Got empty resource version at object metadata");
                return None;
            }
        };

        Some(Self(new_resource_version.clone()))
    }
}
