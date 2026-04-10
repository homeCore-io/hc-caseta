use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub homecore: HomecoreConfig,
    pub caseta: CasetaConfig,
    #[serde(default)]
    pub logging: crate::logging::LoggingConfig,
    #[serde(default)]
    pub devices: Vec<DeviceConfig>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Cannot read config {path}: {e}"))?;
        toml::from_str(&text).map_err(|e| anyhow::anyhow!("Config parse error in {path}: {e}"))
    }
}

// ---------------------------------------------------------------------------
// HomeCore broker connection
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct HomecoreConfig {
    #[serde(default = "default_broker_host")]
    pub broker_host: String,
    #[serde(default = "default_broker_port")]
    pub broker_port: u16,
    #[serde(default = "default_plugin_id")]
    pub plugin_id: String,
    #[serde(default)]
    pub password: String,
}

fn default_broker_host() -> String {
    "127.0.0.1".into()
}
fn default_broker_port() -> u16 {
    1883
}
fn default_plugin_id() -> String {
    "plugin.caseta".into()
}

// ---------------------------------------------------------------------------
// Caseta Pro bridge connection
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct CasetaConfig {
    pub host: String,
    #[serde(default = "default_lip_port")]
    pub port: u16,
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default = "default_password")]
    pub password: String,
    /// Default fade time for dimmers (seconds).
    #[serde(default = "default_fade_secs")]
    pub default_fade_secs: f64,
    /// Delay between reconnection attempts (seconds).
    #[serde(default = "default_reconnect_delay_secs")]
    pub reconnect_delay_secs: u64,
}

fn default_lip_port() -> u16 {
    23
}
fn default_username() -> String {
    "lutron".into()
}
fn default_password() -> String {
    "integration".into()
}
fn default_fade_secs() -> f64 {
    1.0
}
fn default_reconnect_delay_secs() -> u64 {
    5
}

// ---------------------------------------------------------------------------
// Device config
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceKind {
    /// Dimmable light — brightness 0-100 with optional fade.
    Dimmer,
    /// Non-dimmable load — on/off only.
    Switch,
    /// Motorized shade — position 0-100.
    Shade,
    /// Ceiling fan control — speed levels (off/low/medium/medium-high/high).
    FanControl,
    /// Pico wireless remote — publishes button events only (read-only).
    Pico,
    /// Occupancy sensor — publishes occupied/vacant (read-only).
    OccupancySensor,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceConfig {
    pub integration_id: u32,
    pub name: String,
    pub kind: DeviceKind,
    pub area: Option<String>,
    /// Per-device fade time override (seconds).  Falls back to caseta.default_fade_secs.
    pub fade_secs: Option<f64>,
    /// Invert shade position: false = Lutron native (0=open, 100=closed),
    /// true = inverted (0=closed, 100=open).
    #[serde(default)]
    pub invert_position: bool,
    /// Pico button component numbers (e.g. [2, 3, 4, 5, 6]).
    /// Reserved for future use (button name mapping).
    #[serde(default)]
    #[allow(dead_code)]
    pub buttons: Vec<u32>,
}
