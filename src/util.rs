//! Iw model — wireless configuration utility.

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

impl Iw {
    pub fn new() -> Self { Self::default() }

    pub fn get_station(&self, _iface: &str) -> StationInfo {
        StationInfo { ssid: "MyNetwork".into(), bssid: "00:11:22:33:44:55".into(), signal: -50, freq: 2.437, tx_bitrate: 54.0, rx_bitrate: 54.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_station() {
        let i = Iw::new();
        assert_eq!(i.get_station("wlan0").ssid, "MyNetwork");
    }
}
