//! Network definitions for iw.

/// Network security
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkSecurity {
    /// Open network
    Open,
    /// WEP security
    Wep,
    /// WPA security
    Wpa,
    /// WPA2 security
    Wpa2,
    /// WPA3 security
    Wpa3,
}

/// Network state
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkState {
    /// Visible network
    Visible,
    /// Hidden network
    Hidden,
    /// Connected network
    Connected,
    /// Saved network
    Saved,
}

/// Signal info
#[derive(Debug, Clone)]
pub struct SignalInfo {
    /// RSSI in dBm
    pub rssi_dbm: i32,
    /// Frequency in MHz
    pub frequency_mhz: u32,
    /// Bitrate in Mbps
    pub bitrate_mbps: u32,
    /// Noise in dBm
    pub noise_dbm: Option<i32>,
}

impl SignalInfo {
    /// Create new signal info
    pub fn new(rssi_dbm: i32, frequency_mhz: u32, bitrate_mbps: u32) -> Self {
        Self {
            rssi_dbm,
            frequency_mhz,
            bitrate_mbps,
            noise_dbm: None,
        }
    }
    
    /// Set noise
    pub fn set_noise(&mut self, noise_dbm: i32) {
        self.noise_dbm = Some(noise_dbm);
    }
    
    /// Get signal quality (0-100)
    pub fn get_quality(&self) -> u8 {
        // Convert dBm to quality (approximate)
        let quality = ((self.rssi_dbm + 100) * 2).clamp(0, 100) as u8;
        quality
    }
}

/// Wireless network
#[derive(Debug, Clone)]
pub struct WirelessNetwork {
    /// Network SSID
    pub ssid: String,
    /// Network BSSID
    pub bssid: String,
    /// Network state
    pub state: NetworkState,
    /// Network security
    pub security: NetworkSecurity,
    /// Signal info
    pub signal: SignalInfo,
    /// Channel
    pub channel: u32,
    /// Band (2.4GHz/5GHz/6GHz)
    pub band: String,
    /// Created timestamp
    pub created_at: u64,
    /// Last seen timestamp
    pub last_seen: u64,
}

impl WirelessNetwork {
    /// Create new wireless network
    pub fn new(ssid: String, bssid: String) -> Self {
        Self {
            ssid,
            bssid,
            state: NetworkState::Visible,
            security: NetworkSecurity::Open,
            signal: SignalInfo::new(-100, 0, 0),
            channel: 0,
            band: "2.4GHz".into(),
            created_at: Self::now(),
            last_seen: Self::now(),
        }
    }
    
    /// Set signal
    pub fn set_signal(&mut self, signal: SignalInfo) {
        self.signal = signal;
        self.last_seen = Self::now();
    }
    
    /// Set security
    pub fn set_security(&mut self, security: NetworkSecurity) {
        self.security = security;
        self.last_seen = Self::now();
    }
    
    /// Set channel
    pub fn set_channel(&mut self, channel: u32) {
        self.channel = channel;
        self.last_seen = Self::now();
    }
    
    /// Set band
    pub fn set_band(&mut self, band: String) {
        self.band = band;
        self.last_seen = Self::now();
    }
    
    /// Mark as connected
    pub fn mark_connected(&mut self) {
        self.state = NetworkState::Connected;
        self.last_seen = Self::now();
    }
    
    /// Mark as hidden
    pub fn mark_hidden(&mut self) {
        self.state = NetworkState::Hidden;
        self.last_seen = Self::now();
    }
    
    /// Mark as saved
    pub fn mark_saved(&mut self) {
        self.state = NetworkState::Saved;
        self.last_seen = Self::now();
    }
    
    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.state == NetworkState::Visible
    }
    
    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.state == NetworkState::Connected
    }
    
    /// Check if hidden
    pub fn is_hidden(&self) -> bool {
        self.state == NetworkState::Hidden
    }
    
    /// Check if secured
    pub fn is_secured(&self) -> bool {
        self.security != NetworkSecurity::Open
    }
    
    /// Get signal quality
    pub fn get_signal_quality(&self) -> u8 {
        self.signal.get_quality()
    }
    
    /// Get network age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Get time since last seen
    pub fn get_idle_seconds(&self) -> u64 {
        Self::now() - self.last_seen
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
