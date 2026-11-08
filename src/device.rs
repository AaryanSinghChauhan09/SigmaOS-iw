//! Wireless device definitions for iw.

/// Device state
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceState {
    /// Device down
    Down,
    /// Device up
    Up,
    /// Device scanning
    Scanning,
    /// Device connecting
    Connecting,
    /// Device connected
    Connected,
    /// Device error
    Error,
}

/// Device info
#[derive(Debug, Clone, Default)]
pub struct DeviceInfo {
    /// MAC address
    pub mac: String,
    /// IP address
    pub ip_address: String,
    /// Interface type
    pub interface_type: String,
    /// Capabilities
    pub capabilities: DeviceCapabilities,
}

/// Device capabilities
#[derive(Debug, Clone)]
pub struct DeviceCapabilities {
    /// Supports monitor mode
    pub supports_monitor: bool,
    /// Supports AP mode
    pub supports_ap: bool,
    /// Supported bands
    pub bands: Vec<String>,
    /// Max bitrate
    pub max_bitrate: f64,
}

impl DeviceCapabilities {
    /// Create new capabilities
    pub fn new(supports_monitor: bool, supports_ap: bool, bands: Vec<String>) -> Self {
        Self {
            supports_monitor,
            supports_ap,
            bands,
            max_bitrate: 0.0,
        }
    }
    
    /// Set max bitrate
    pub fn set_max_bitrate(&mut self, bitrate: f64) {
        self.max_bitrate = bitrate;
    }
}

impl Default for DeviceCapabilities {
    fn default() -> Self {
        Self::new(false, false, vec![])
    }
}

/// Wireless device
#[derive(Debug, Clone)]
pub struct WirelessDevice {
    /// Device name (interface)
    pub name: String,
    /// Device state
    pub state: DeviceState,
    /// Device info
    pub info: DeviceInfo,
    /// Created timestamp
    pub created_at: u64,
    /// Last modified timestamp
    pub last_modified: u64,
}

impl WirelessDevice {
    /// Create new wireless device
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: DeviceState::Down,
            info: DeviceInfo::default(),
            created_at: Self::now(),
            last_modified: Self::now(),
        }
    }
    
    /// Set MAC address
    pub fn set_mac(&mut self, mac: String) {
        self.info.mac = mac;
        self.last_modified = Self::now();
    }
    
    /// Set IP address
    pub fn set_ip(&mut self, ip: String) {
        self.info.ip_address = ip;
        self.last_modified = Self::now();
    }
    
    /// Set capabilities
    pub fn set_caps(&mut self, caps: DeviceCapabilities) {
        self.info.capabilities = caps;
        self.last_modified = Self::now();
    }
    
    /// Bring device up
    pub fn up(&mut self) {
        self.state = DeviceState::Up;
        self.last_modified = Self::now();
    }
    
    /// Bring device down
    pub fn down(&mut self) {
        self.state = DeviceState::Down;
        self.last_modified = Self::now();
    }
    
    /// Start scan
    pub fn scan(&mut self) {
        if self.state == DeviceState::Up {
            self.state = DeviceState::Scanning;
            self.last_modified = Self::now();
        }
    }
    
    /// Finish scan
    pub fn finish_scan(&mut self) {
        if self.state == DeviceState::Scanning {
            self.state = DeviceState::Up;
            self.last_modified = Self::now();
        }
    }
    
    /// Set connecting state
    pub fn set_connecting(&mut self) {
        self.state = DeviceState::Connecting;
        self.last_modified = Self::now();
    }
    
    /// Set connected state
    pub fn set_connected(&mut self) {
        self.state = DeviceState::Connected;
        self.last_modified = Self::now();
    }
    
    /// Set error state
    pub fn set_error(&mut self) {
        self.state = DeviceState::Error;
        self.last_modified = Self::now();
    }
    
    /// Check if up
    pub fn is_up(&self) -> bool {
        matches!(self.state, DeviceState::Up | DeviceState::Scanning | DeviceState::Connecting | DeviceState::Connected)
    }
    
    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.state == DeviceState::Connected
    }
    
    /// Get device age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Get time since modification
    pub fn get_idle_seconds(&self) -> u64 {
        Self::now() - self.last_modified
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
