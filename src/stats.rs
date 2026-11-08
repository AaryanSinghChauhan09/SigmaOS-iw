//! Stats definitions for iw.

/// Link stats
#[derive(Debug, Clone, Default)]
pub struct LinkStats {
    /// Signal strength in dBm
    pub signal_dbm: i32,
    /// Bitrate in Mbps
    pub bitrate_mbps: f64,
    /// Frequency in MHz
    pub frequency_mhz: u32,
    /// TX power in dBm
    pub tx_power_dbm: i32,
    /// RX bitrate in Mbps
    pub rx_bitrate_mbps: f64,
    /// TX bitrate in Mbps
    pub tx_bitrate_mbps: f64,
}

impl LinkStats {
    /// Create new link stats
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set signal
    pub fn set_signal(&mut self, signal_dbm: i32) {
        self.signal_dbm = signal_dbm;
    }
    
    /// Set bitrate
    pub fn set_bitrate(&mut self, bitrate_mbps: f64) {
        self.bitrate_mbps = bitrate_mbps;
    }
    
    /// Set frequency
    pub fn set_frequency(&mut self, frequency_mhz: u32) {
        self.frequency_mhz = frequency_mhz;
    }
    
    /// Set TX power
    pub fn set_tx_power(&mut self, tx_power_dbm: i32) {
        self.tx_power_dbm = tx_power_dbm;
    }
    
    /// Set RX bitrate
    pub fn set_rx_bitrate(&mut self, bitrate_mbps: f64) {
        self.rx_bitrate_mbps = bitrate_mbps;
    }
    
    /// Set TX bitrate
    pub fn set_tx_bitrate(&mut self, bitrate_mbps: f64) {
        self.tx_bitrate_mbps = bitrate_mbps;
    }
}

/// Station stats
#[derive(Debug, Clone, Default)]
pub struct StationStats {
    /// TX bytes
    pub tx_bytes: u64,
    /// RX bytes
    pub rx_bytes: u64,
    /// TX packets
    pub tx_packets: u64,
    /// RX packets
    pub rx_packets: u64,
    /// TX retries
    pub tx_retries: u64,
    /// TX failed
    pub tx_failed: u64,
    /// RX dropped
    pub rx_dropped: u64,
}

impl StationStats {
    /// Create new station stats
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set TX bytes
    pub fn set_tx_bytes(&mut self, bytes: u64) {
        self.tx_bytes = bytes;
    }
    
    /// Set RX bytes
    pub fn set_rx_bytes(&mut self, bytes: u64) {
        self.rx_bytes = bytes;
    }
    
    /// Set TX packets
    pub fn set_tx_packets(&mut self, packets: u64) {
        self.tx_packets = packets;
    }
    
    /// Set RX packets
    pub fn set_rx_packets(&mut self, packets: u64) {
        self.rx_packets = packets;
    }
    
    /// Add TX retries
    pub fn add_tx_retries(&mut self, count: u64) {
        self.tx_retries += count;
    }
    
    /// Add TX failed
    pub fn add_tx_failed(&mut self, count: u64) {
        self.tx_failed += count;
    }
    
    /// Add RX dropped
    pub fn add_rx_dropped(&mut self, count: u64) {
        self.rx_dropped += count;
    }
    
    /// Get total bytes
    pub fn get_total_bytes(&self) -> u64 {
        self.tx_bytes + self.rx_bytes
    }
    
    /// Get total packets
    pub fn get_total_packets(&self) -> u64 {
        self.tx_packets + self.rx_packets
    }
}
