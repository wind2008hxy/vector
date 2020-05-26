//! The test framework main entry point.

use super::{namespace, test_pod, vector, Interface, Result};

pub struct Framework {
    interface: Interface,
}

impl Framework {
    /// Create a new [`Framework`].
    pub fn new(interface: Interface) -> Self {
        Self { interface }
    }

    pub fn vector(&self, namespace: &str, custom_resource: &str) -> Result<vector::Manager> {
        let manager = vector::Manager::new(
            self.interface.deploy_vector_command.as_str(),
            namespace,
            custom_resource,
        )?;
        manager.up()?;
        Ok(manager)
    }

    pub fn namespace(&self, namespace: &str) -> Result<namespace::Manager> {
        let manager = namespace::Manager::new(&self.interface.kubectl_command, namespace)?;
        manager.up()?;
        Ok(manager)
    }

    pub fn test_pod(&self, config: test_pod::Config) -> Result<test_pod::Manager> {
        let manager = test_pod::Manager::new(&self.interface.kubectl_command, config)?;
        manager.up()?;
        Ok(manager)
    }

    pub fn logs(&self, namespace: &str, name: &str) -> Vec<String> {
        todo!();
    }
}
