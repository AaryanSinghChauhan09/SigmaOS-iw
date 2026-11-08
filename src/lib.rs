//! SigmaOS-iw — wireless configuration utility for SigmaOS.
//!
//! Models iw's wireless operations: device info, network scanning,
//! connection management, signal stats, and regulatory info.

pub mod device;
pub mod network;
pub mod connection;
pub mod stats;
pub mod manager;
pub mod options;
pub mod util;

pub use device::{WirelessDevice, DeviceState, DeviceInfo, DeviceCapabilities};
pub use network::{WirelessNetwork, NetworkState, NetworkSecurity, SignalInfo};
pub use connection::{Connection, ConnectionState, ConnectionConfig};
pub use stats::{LinkStats, StationStats};
pub use manager::{IwManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};
pub use util::Iw;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_device_state() {
        let states = vec![DeviceState::Down, DeviceState::Up, DeviceState::Scanning, DeviceState::Connecting, DeviceState::Connected, DeviceState::Error];
        assert_eq!(states.len(), 6);
    }
    
    #[test]
    fn test_network_security() {
        let securities = vec![NetworkSecurity::Open, NetworkSecurity::Wep, NetworkSecurity::Wpa, NetworkSecurity::Wpa2, NetworkSecurity::Wpa3];
        assert_eq!(securities.len(), 5);
    }
    
    #[test]
    fn test_network_state() {
        let states = vec![NetworkState::Visible, NetworkState::Hidden, NetworkState::Connected, NetworkState::Saved];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_connection_state() {
        let states = vec![ConnectionState::Disconnected, ConnectionState::Connecting, ConnectionState::Connected, ConnectionState::Failed];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_device_creation() {
        let device = WirelessDevice::new("wlan0".into());
        assert_eq!(device.name, "wlan0".to_string());
        assert_eq!(device.state, DeviceState::Down);
    }
    
    #[test]
    fn test_device_info() {
        let mut device = WirelessDevice::new("wlan0".into());
        device.set_mac("aa:bb:cc:dd:ee:ff".into());
        device.set_caps(DeviceCapabilities::new(true, true, vec!["2.4GHz".into(), "5GHz".into()]));
        
        assert_eq!(device.info.mac, "aa:bb:cc:dd:ee:ff".to_string());
        assert!(device.info.capabilities.bands.contains(&"5GHz".to_string()));
    }
    
    #[test]
    fn test_device_lifecycle() {
        let mut device = WirelessDevice::new("wlan0".into());
        device.up();
        assert_eq!(device.state, DeviceState::Up);
        device.scan();
        assert_eq!(device.state, DeviceState::Scanning);
        device.down();
        assert_eq!(device.state, DeviceState::Down);
    }
    
    #[test]
    fn test_network_creation() {
        let network = WirelessNetwork::new("MyWiFi".into(), "aa:bb:cc:dd:ee:ff".into());
        assert_eq!(network.ssid, "MyWiFi".to_string());
        assert_eq!(network.bssid, "aa:bb:cc:dd:ee:ff".to_string());
        assert_eq!(network.state, NetworkState::Visible);
    }
    
    #[test]
    fn test_network_signal() {
        let mut network = WirelessNetwork::new("WiFi".into(), "bssid".into());
        network.set_signal(SignalInfo::new(-50, 5745, 866));
        
        assert_eq!(network.signal.rssi_dbm, -50);
        assert_eq!(network.signal.frequency_mhz, 5745);
        assert_eq!(network.signal.bitrate_mbps, 866);
    }
    
    #[test]
    fn test_network_security_set() {
        let mut network = WirelessNetwork::new("WiFi".into(), "bssid".into());
        network.set_security(NetworkSecurity::Wpa2);
        assert_eq!(network.security, NetworkSecurity::Wpa2);
    }
    
    #[test]
    fn test_connection_creation() {
        let conn = Connection::new("MyWiFi".into());
        assert_eq!(conn.ssid, "MyWiFi".to_string());
        assert_eq!(conn.state, ConnectionState::Disconnected);
    }
    
    #[test]
    fn test_connection_lifecycle() {
        let mut conn = Connection::new("WiFi".into());
        conn.connect();
        assert_eq!(conn.state, ConnectionState::Connecting);
        conn.finish_connecting();
        assert_eq!(conn.state, ConnectionState::Connected);
        conn.disconnect();
        assert_eq!(conn.state, ConnectionState::Disconnected);
    }
    
    #[test]
    fn test_connection_config() {
        let mut config = ConnectionConfig::default();
        config.ssid = "MyWiFi".into();
        config.password = "secret".into();
        config.hidden = false;
        
        assert_eq!(config.ssid, "MyWiFi".to_string());
        assert!(!config.hidden);
    }
    
    #[test]
    fn test_link_stats() {
        let mut stats = LinkStats::new();
        stats.set_signal(-45);
        stats.set_bitrate(1200.0);
        stats.set_frequency(5180);
        
        assert_eq!(stats.signal_dbm, -45);
        assert_eq!(stats.bitrate_mbps, 1200.0);
    }
    
    #[test]
    fn test_station_stats() {
        let mut stats = StationStats::new();
        stats.set_tx_bytes(1024 * 1024);
        stats.set_rx_bytes(2048 * 1024);
        stats.set_tx_packets(100);
        stats.set_rx_packets(200);
        
        assert_eq!(stats.tx_bytes, 1024 * 1024);
        assert_eq!(stats.rx_bytes, 2048 * 1024);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = IwManager::new();
        assert!(manager.list_devices().is_empty());
        assert!(manager.list_networks().is_empty());
    }
    
    #[test]
    fn test_add_device() {
        let mut manager = IwManager::new();
        manager.initialize();
        let id = manager.add_device("wlan0".into());
        assert!(manager.get_device(&id).is_some());
    }
    
    #[test]
    fn test_scan_networks() {
        let mut manager = IwManager::new();
        manager.initialize();
        let dev_id = manager.add_device("wlan0".into());
        
        let count = manager.scan_networks(&dev_id);
        assert!(count >= 0);
    }
    
    #[test]
    fn test_connect() {
        let mut manager = IwManager::new();
        manager.initialize();
        let dev_id = manager.add_device("wlan0".into());
        
        let conn_id = manager.connect(&dev_id, "MyWiFi".into(), Some("password".into()));
        assert!(manager.get_connection(&conn_id).is_some());
    }
    
    #[test]
    fn test_disconnect() {
        let mut manager = IwManager::new();
        manager.initialize();
        let dev_id = manager.add_device("wlan0".into());
        let conn_id = manager.connect(&dev_id, "WiFi".into(), None);
        
        assert!(manager.disconnect(&conn_id));
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = IwManager::new();
        manager.initialize();
        manager.add_device("wlan0".into());
        
        let export = manager.export_data();
        let mut manager2 = IwManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_devices().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = IwManager::new();
        manager.initialize();
        manager.add_device("wlan0".into());
        manager.add_device("wlan1".into());
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_devices, 2);
    }
}
