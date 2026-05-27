use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;

use crate::state::settings::AppSettings;
use crate::utils::paths;

const SINGBOX_RELEASE_API: &str =
    "https://api.github.com/repos/SagerNet/sing-box/releases/latest";

pub struct CoreDownloader;

impl CoreDownloader {
    pub fn core_dir() -> PathBuf {
        paths::app_install_dir().join("sing-box-core")
    }

    pub fn core_path() -> PathBuf {
        Self::core_dir().join("sing-box.exe")
    }

    pub fn is_installed() -> bool {
        Self::core_path().exists()
    }

    pub fn download_latest() -> Result<PathBuf> {
        log::info!("Fetching latest sing-box release info...");

        let body_str = ureq::get(SINGBOX_RELEASE_API)
            .header("User-Agent", "song-bin")
            .call()?
            .body_mut()
            .read_to_string()?;

        let response: serde_json::Value = serde_json::from_str(&body_str)?;

        let assets = response
            .get("assets")
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow::anyhow!("No assets in release"))?;

        let download_url = assets
            .iter()
            .find_map(|asset| {
                let name = asset.get("name")?.as_str()?;
                if name.ends_with("windows-arm64.zip") {
                    asset
                        .get("browser_download_url")
                        .and_then(|v| v.as_str())
                        .map(String::from)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("No windows-arm64 asset found"))?;

        log::info!("Downloading from: {}", download_url);

        let mut reader = ureq::get(&download_url)
            .header("User-Agent", "song-bin")
            .call()?
            .into_body()
            .into_reader();

        let temp_zip = AppSettings::data_dir().join("sing-box-download.zip");
        if let Some(parent) = temp_zip.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(&temp_zip)?;
        let mut buf = vec![0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => file.write_all(&buf[..n])?,
                Err(e) => return Err(e.into()),
            }
        }
        drop(file);

        let core_dir = Self::core_dir();
        fs::create_dir_all(&core_dir)?;

        extract_singbox_from_zip(&temp_zip, &core_dir)?;

        let _ = fs::remove_file(&temp_zip);

        let core_path = Self::core_path();
        if core_path.exists() {
            log::info!("sing-box core installed at {:?}", core_path);
            Ok(core_path)
        } else {
            anyhow::bail!("sing-box.exe not found after extraction")
        }
    }
}

fn extract_singbox_from_zip(zip_path: &PathBuf, target_dir: &PathBuf) -> Result<()> {
    let file = fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();

        if name.ends_with("sing-box.exe") {
            let target = target_dir.join("sing-box.exe");
            let mut outfile = fs::File::create(&target)?;
            std::io::copy(&mut entry, &mut outfile)?;
            log::info!("Extracted sing-box.exe to {:?}", target);
            return Ok(());
        }
    }

    anyhow::bail!("sing-box.exe not found in zip archive")
}
