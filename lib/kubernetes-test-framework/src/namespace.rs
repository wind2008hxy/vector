use super::{custom_resource_file::CustomResourceFile, Result};
use k8s_openapi::api::core::v1::Pod;
use std::process::{Command, Stdio};

pub struct Manager {
    kubectl_command: String,
    namespace: String,
}

impl Manager {
    pub fn new(kubectl_command: &str, namespace: &str) -> Result<Self> {
        Ok(Self {
            kubectl_command: kubectl_command.to_owned(),
            namespace: namespace.to_owned(),
        })
    }

    pub fn up(&self) -> Result<()> {
        self.exec("create")
    }

    pub fn down(self) -> Result<()> {
        self.exec("delete")
    }

    fn exec(&self, subcommand: &str) -> Result<()> {
        Command::new(&self.kubectl_command)
            .arg(subcommand)
            .arg("namespace")
            .arg(&self.namespace)
            .stdin(Stdio::null())
            .spawn()?;
        Ok(())
    }
}
