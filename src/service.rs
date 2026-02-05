use ratatui::style::Color;
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Clone, Deserialize)]
pub struct SystemdService {
    pub unit: String,
    pub load: String,
    pub active: String,
    pub sub: String,
    pub description: String,
}

impl SystemdService {
    pub fn status_display(&self) -> &str {
        &self.sub
    }

    pub fn status_color(&self) -> Color {
        match self.sub.as_str() {
            "running" => Color::Green,
            "exited" => Color::Yellow,
            "dead" | "stopped" => Color::DarkGray,
            "failed" => Color::Red,
            _ => Color::White,
        }
    }
}

pub fn fetch_services() -> Result<Vec<SystemdService>, String> {
    let output = Command::new("systemctl")
        .args(["list-units", "--type=service", "--all", "--no-pager", "--output=json"])
        .output()
        .map_err(|e| format!("Failed to execute systemctl: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "systemctl failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let services: Vec<SystemdService> = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(services)
}
