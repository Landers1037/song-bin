use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

use anyhow::Result;

use crate::state::settings::AppSettings;

pub struct SingboxProcess {
    child: Arc<Mutex<Option<Child>>>,
    config_path: PathBuf,
}

impl SingboxProcess {
    pub fn new() -> Self {
        let config_path = AppSettings::data_dir().join("config.json");
        Self {
            child: Arc::new(Mutex::new(None)),
            config_path,
        }
    }

    pub fn core_path() -> PathBuf {
        AppSettings::data_dir().join("core").join("sing-box.exe")
    }

    pub fn is_core_installed() -> bool {
        Self::core_path().exists()
    }

    pub fn is_running(&self) -> bool {
        let guard = self.child.lock().unwrap();
        guard.is_some()
    }

    pub fn start(&self, config: &serde_json::Value) -> Result<()> {
        if self.is_running() {
            self.stop()?;
        }

        let core_path = Self::core_path();
        if !core_path.exists() {
            anyhow::bail!("sing-box core not found at {:?}", core_path);
        }

        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.config_path, serde_json::to_string_pretty(config)?)?;

        let mut cmd = Command::new(&core_path);
        cmd.arg("run")
            .arg("-c")
            .arg(&self.config_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }

        let child = cmd.spawn()?;

        log::info!("sing-box started with PID {}", child.id());
        *self.child.lock().unwrap() = Some(child);

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        let mut guard = self.child.lock().unwrap();
        if let Some(mut child) = guard.take() {
            log::info!("Stopping sing-box...");
            let _ = child.kill();
            let _ = child.wait();
            log::info!("sing-box stopped.");
        }
        Ok(())
    }

    pub fn restart(&self, config: &serde_json::Value) -> Result<()> {
        self.stop()?;
        self.start(config)
    }
}

impl Drop for SingboxProcess {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
