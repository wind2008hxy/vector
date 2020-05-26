use std::env;

#[derive(Debug)]
pub struct Interface {
    pub deploy_vector_command: String,
    pub deploy_test_pod_command: String,
    pub collect_test_logs_command: String,
    pub kubectl_command: String,
}

impl Interface {
    pub fn from_env() -> Option<Self> {
        Some(Self {
            deploy_vector_command: env::var("KUBE_TEST_DEPLOY_VECTOR_COMMAND").ok()?,
            deploy_test_pod_command: env::var("KUBE_TEST_DEPLOY_TEST_POD_COMMAND").ok()?,
            collect_test_logs_command: env::var("KUBE_TEST_COLLECT_TEST_LOGS_COMMAND").ok()?,
            kubectl_command: env::var("VECTOR_TEST_KUBECTL").ok()?,
        })
    }
}
