use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TrafficStats {
    #[serde(default)]
    pub up: u64,
    #[serde(default)]
    pub down: u64,
}

pub struct SpeedMonitor {
    api_base: String,
    last_stats: TrafficStats,
}

impl SpeedMonitor {
    pub fn new(port: u16) -> Self {
        Self {
            api_base: format!("http://127.0.0.1:{}", port),
            last_stats: TrafficStats::default(),
        }
    }

    pub fn fetch_speed(&mut self) -> Result<(String, String)> {
        let url = format!("{}/traffic", self.api_base);
        let body = ureq::get(&url).call()?.body_mut().read_to_string()?;
        let stats: TrafficStats = serde_json::from_str(&body)?;

        let up_speed = stats.up.saturating_sub(self.last_stats.up);
        let down_speed = stats.down.saturating_sub(self.last_stats.down);
        self.last_stats = stats;

        Ok((format_speed(up_speed), format_speed(down_speed)))
    }
}

fn format_speed(bytes_per_sec: u64) -> String {
    if bytes_per_sec < 1024 {
        format!("{} B/s", bytes_per_sec)
    } else if bytes_per_sec < 1024 * 1024 {
        format!("{:.1} KB/s", bytes_per_sec as f64 / 1024.0)
    } else {
        format!("{:.2} MB/s", bytes_per_sec as f64 / (1024.0 * 1024.0))
    }
}
