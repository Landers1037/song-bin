use std::fs;
use std::path::{Path, PathBuf};

/// 返回程序安装目录（即当前 exe 所在目录）。
///
/// 注意：开发态运行时，这通常是 `target/{debug|release}`。
pub fn app_install_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// 在资源管理器中打开目录；若目录不存在则先创建。
pub fn open_dir_in_explorer(dir: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(dir)?;
    let dir = fs::canonicalize(dir)?;

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;

        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let path = dir.to_string_lossy();

        // explorer 对不存在的路径会静默失败；用 `start` 更可靠。
        std::process::Command::new("cmd")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["/C", "start", "", &path])
            .spawn()
            .map_err(|e| anyhow::anyhow!("无法打开目录：{e}"))?;
    }

    #[cfg(not(windows))]
    {
        let _ = dir;
        anyhow::bail!("open_dir_in_explorer 仅支持 Windows");
    }

    Ok(())
}

