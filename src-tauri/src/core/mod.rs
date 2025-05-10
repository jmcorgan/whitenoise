use anyhow::Context;
use std::path::{Path, PathBuf};
use crate::setup_logging;

#[derive(Clone, Debug)]
pub struct WhitenoiseConfig {
    /// Directory for application data
    pub data_dir: PathBuf,

    /// Directory for application logs
    pub logs_dir: PathBuf,
}

impl WhitenoiseConfig {
    pub fn new(data_dir: &Path, logs_dir: &Path) -> Self {
        let env_suffix = if cfg!(dev) { "dev" } else { "release" };
        let formatted_data_dir = data_dir.join(env_suffix);
        let formatted_logs_dir = logs_dir.join(env_suffix);

        Self {
            data_dir: formatted_data_dir,
            logs_dir: formatted_logs_dir,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WhitenoiseCore {
    config: WhitenoiseConfig,
}

impl WhitenoiseCore {
    pub fn new(config: WhitenoiseConfig) -> Self {
        let mut inst = Self { config };

        inst.setup_dirs().unwrap();
        inst.setup_logging().unwrap();

        // Return fully configured, ready-to-go instance
        inst
    }

    fn setup_dirs(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Note: tracing not yet available here
        let data_dir = &self.config.data_dir;
        let logs_dir = &self.config.logs_dir;

        std::fs::create_dir_all(data_dir)
            .with_context(|| format!("Failed to create data directory: {:?}", data_dir))?;
        std::fs::create_dir_all(logs_dir)
            .with_context(|| format!("Failed to create logs directory: {:?}", logs_dir))?;

        Ok(())
    }

    fn setup_logging(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let logs_dir = &self.config.logs_dir;

        setup_logging(logs_dir.clone())
            .map_err(|e| anyhow::anyhow!("Failed to setup logging: {}", e))?;

        tracing::debug!("Logging initialized in directory: {:?}", logs_dir);

        Ok(())
    }
}
