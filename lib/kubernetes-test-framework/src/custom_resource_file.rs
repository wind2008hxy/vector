use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct CustomResourceFile {
    path: PathBuf,
}

impl CustomResourceFile {
    pub fn new(data: &str) -> std::io::Result<Self> {
        let mut path = std::env::temp_dir();
        path.push("custom.yaml");
        std::fs::write(&path, data)?;
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

impl Drop for CustomResourceFile {
    fn drop(&mut self) {
        std::fs::remove_file(&self.path).expect("unable to clean up custom resource file");
    }
}
