//! Utility model for iw (legacy compat).

#[derive(Debug, Clone)]
pub struct StationInfo {
    pub ssid: String,
    pub bssid: String,
    pub signal: i32,
    pub freq: f64,
    pub tx_bitrate: f64,
    pub rx_bitrate: f64,
}

#[derive(Debug, Clone)]
pub struct Iw {
    pub json: bool,
    pub debug: bool,
}

impl Default for Iw {
    fn default() -> Self { Self { json: false, debug: false } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_iw() {
        let iw = Iw::default();
        assert!(!iw.json);
    }
}
