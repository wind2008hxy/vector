//! The test framework main entry point.

use super::{test_pod, vector, Interface, Result};

pub struct Framework {
    interface: Interface,
}

impl Framework {
    /// Create a new [`Framework`].
    pub fn new(interface: Interface) -> Self {
        Self { interface }
    }

    pub fn deploy_vector(&self, namespace: &str, custom_resource: &str) -> Result<vector::Manager> {
        let manager = vector::Manager::new(
            self.interface.deploy_vector_command.as_str(),
            namespace,
            custom_resource,
        )?;
        manager.up()?;
        Ok(manager)
    }

    pub fn deploy_test_pod(
        &self,
        config: test_pod::Config,
        namespace: Option<String>,
    ) -> Result<test_pod::Manager> {
        let manager = test_pod::Manager::new("kubectl", config, namespace)?;
        manager.up()?;
        Ok(manager)
    }

    pub fn collect_test_logs(&self) -> Vec<String> {
        todo!();
    }
}
