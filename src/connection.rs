//! Connection definitions for iw.

/// Connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    /// Disconnected
    Disconnected,
    /// Connecting
    Connecting,
    /// Connected
    Connected,
    /// Failed
    Failed,
}

/// Connection config
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// SSID
    pub ssid: String,
    /// Password
    pub password: String,
    /// Hidden network
    pub hidden: bool,
    /// Band preference
    pub band: String,
    /// Auto-connect
    pub auto_connect: bool,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            ssid: String::new(),
            password: String::new(),
            hidden: false,
            band: "any".into(),
            auto_connect: true,
        }
    }
}

/// Connection
#[derive(Debug, Clone)]
pub struct Connection {
    /// Connection ID
    pub id: String,
    /// Connection SSID
    pub ssid: String,
    /// Connection state
    pub state: ConnectionState,
    /// Connection config
    pub config: ConnectionConfig,
    /// Connected device name
    pub device_name: String,
    /// Error message
    pub error: Option<String>,
    /// Created timestamp
    pub created_at: u64,
    /// Connected timestamp
    pub connected_at: Option<u64>,
}

impl Connection {
    /// Create new connection
    pub fn new(ssid: String) -> Self {
        Self {
            id: format!("conn-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            ssid,
            state: ConnectionState::Disconnected,
            config: ConnectionConfig::default(),
            device_name: String::new(),
            error: None,
            created_at: Self::now(),
            connected_at: None,
        }
    }
    
    /// Set config
    pub fn set_config(&mut self, config: ConnectionConfig) {
        self.config = config;
        self.ssid = self.config.ssid.clone();
    }
    
    /// Set device
    pub fn set_device(&mut self, device_name: String) {
        self.device_name = device_name;
    }
    
    /// Connect
    pub fn connect(&mut self) {
        self.state = ConnectionState::Connecting;
        self.error = None;
    }
    
    /// Finish connecting
    pub fn finish_connecting(&mut self) {
        self.state = ConnectionState::Connected;
        self.connected_at = Some(Self::now());
    }
    
    /// Fail connecting
    pub fn fail(&mut self, error: String) {
        self.state = ConnectionState::Failed;
        self.error = Some(error);
    }
    
    /// Disconnect
    pub fn disconnect(&mut self) {
        self.state = ConnectionState::Disconnected;
        self.connected_at = None;
    }
    
    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.state == ConnectionState::Connected
    }
    
    /// Check if connecting
    pub fn is_connecting(&self) -> bool {
        self.state == ConnectionState::Connecting
    }
    
    /// Get connection duration in seconds
    pub fn get_duration_seconds(&self) -> u64 {
        if let Some(connected) = self.connected_at {
            Self::now() - connected
        } else {
            0
        }
    }
    
    /// Get connection age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
