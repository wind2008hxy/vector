use super::{custom_resource_file::CustomResourceFile, Result};
use k8s_openapi::api::core::v1::Pod;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Config {
    custom_resource_file: CustomResourceFile,
}

impl Config {
    pub fn from_pod(pod: &Pod) -> Result<Self> {
        Self::from_resource_string(serde_json::to_string(pod)?.as_str())
    }

    pub fn from_resource_string(resource: &str) -> Result<Self> {
        let custom_resource_file = CustomResourceFile::new(resource)?;
        Ok(Self {
            custom_resource_file,
        })
    }
}

pub struct Manager {
    kubectl_command: String,
    config: Config,
    namespace: Option<String>,
}

impl Manager {
    pub fn new(kubectl_command: &str, config: Config, namespace: Option<String>) -> Result<Self> {
        Ok(Self {
            kubectl_command: kubectl_command.to_owned(),
            config,
            namespace,
        })
    }

    pub fn up(&self) -> Result<()> {
        let mut command = self.prepare_command();

        command.arg("create");
        if let Some(ref namespace) = self.namespace {
            command.arg("-n").arg(namespace);
        }
        command
            .arg("-f")
            .arg(self.config.custom_resource_file.path());
        Self::run_command(command)?;

        Ok(())
    }

    pub fn down(self) -> Result<()> {
        let mut command = self.prepare_command();

        command.arg("delete");
        if let Some(ref namespace) = self.namespace {
            command.arg("-n").arg(namespace);
        }
        command
            .arg("-f")
            .arg(self.config.custom_resource_file.path());
        Self::run_command(command)?;

        Ok(())
    }

    fn prepare_command(&self) -> Command {
        let mut command = Command::new(&self.kubectl_command);
        command.stdin(Stdio::null());
        command
    }

    fn run_command(mut command: Command) -> Result<()> {
        command.spawn()?;
        Ok(())
    }
}
