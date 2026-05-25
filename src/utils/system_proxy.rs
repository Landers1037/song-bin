use anyhow::Result;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
pub fn set_system_proxy(host: &str, port: u16) -> Result<()> {
    let proxy = format!("{}:{}", host, port);

    let output = std::process::Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v", "ProxyEnable",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f",
        ])
        .creation_flags(0x08000000)
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Failed to enable proxy in registry");
    }

    let output = std::process::Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v", "ProxyServer",
            "/t", "REG_SZ",
            "/d", &proxy,
            "/f",
        ])
        .creation_flags(0x08000000)
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Failed to set proxy server in registry");
    }

    log::info!("System proxy set to {}", proxy);
    Ok(())
}

#[cfg(windows)]
pub fn clear_system_proxy() -> Result<()> {
    let output = std::process::Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v", "ProxyEnable",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f",
        ])
        .creation_flags(0x08000000)
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Failed to disable proxy in registry");
    }

    log::info!("System proxy cleared");
    Ok(())
}

#[cfg(not(windows))]
pub fn set_system_proxy(_host: &str, _port: u16) -> Result<()> {
    anyhow::bail!("System proxy is only supported on Windows")
}

#[cfg(not(windows))]
pub fn clear_system_proxy() -> Result<()> {
    anyhow::bail!("System proxy is only supported on Windows")
}
