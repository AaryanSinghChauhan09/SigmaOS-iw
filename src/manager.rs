//! iw manager.

use std::collections::HashMap;
use crate::device::{WirelessDevice, DeviceState};
use crate::network::{WirelessNetwork, NetworkState};
use crate::connection::{Connection, ConnectionState, ConnectionConfig};
use crate::stats::{LinkStats, StationStats};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max devices
    pub max_devices: usize,
    /// Max networks per device
    pub max_networks_per_device: usize,
    /// Max connections
    pub max_connections: usize,
    /// Scan timeout in seconds
    pub scan_timeout_secs: u64,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_devices: 10,
            max_networks_per_device: 50,
            max_connections: 10,
            scan_timeout_secs: 10,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total devices
    pub total_devices: usize,
    /// Up devices
    pub up_devices: usize,
    /// Connected devices
    pub connected_devices: usize,
    /// Total networks
    pub total_networks: usize,
    /// Total connections
    pub total_connections: usize,
    /// Active connections
    pub active_connections: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// iw manager
#[derive(Debug, Clone)]
pub struct IwManager {
    /// Devices
    devices: HashMap<String, WirelessDevice>,
    /// Networks per device
    networks: HashMap<String, Vec<WirelessNetwork>>,
    /// Connections
    connections: HashMap<String, Connection>,
    /// Link stats per device
    link_stats: HashMap<String, LinkStats>,
    /// Station stats per device
    station_stats: HashMap<String, StationStats>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for IwManager {
    fn default() -> Self {
        Self {
            devices: HashMap::new(),
            networks: HashMap::new(),
            connections: HashMap::new(),
            link_stats: HashMap::new(),
            station_stats: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_devices: 0,
                up_devices: 0,
                connected_devices: 0,
                total_networks: 0,
                total_connections: 0,
                active_connections: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
}

impl IwManager {
    /// Create new iw manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            devices: HashMap::new(),
            networks: HashMap::new(),
            connections: HashMap::new(),
            link_stats: HashMap::new(),
            station_stats: HashMap::new(),
            options,
            statistics: ManagerStatistics {
                total_devices: 0,
                up_devices: 0,
                connected_devices: 0,
                total_networks: 0,
                total_connections: 0,
                active_connections: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        self.update_statistics();
    }
    
    // Device methods
    
    /// Add device
    pub fn add_device(&mut self, name: String) -> String {
        let device = WirelessDevice::new(name.clone());
        let id = name.clone();
        
        self.devices.insert(id.clone(), device);
        self.networks.insert(id.clone(), vec![]);
        self.update_statistics();
        id
    }
    
    /// Get device
    pub fn get_device(&self, device_id: &str) -> Option<&WirelessDevice> {
        self.devices.get(device_id)
    }
    
    /// Get device mutably
    pub fn get_device_mut(&mut self, device_id: &str) -> Option<&mut WirelessDevice> {
        self.devices.get_mut(device_id)
    }
    
    /// Get all devices
    pub fn list_devices(&self) -> Vec<&WirelessDevice> {
        self.devices.values().collect()
    }
    
    /// Remove device
    pub fn remove_device(&mut self, device_id: &str) -> bool {
        self.devices.remove(device_id);
        self.networks.remove(device_id);
        self.link_stats.remove(device_id);
        self.station_stats.remove(device_id);
        self.update_statistics();
        true
    }
    
    /// Bring device up
    pub fn device_up(&mut self, device_id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.up();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Bring device down
    pub fn device_down(&mut self, device_id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.down();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Scan device
    pub fn scan_device(&mut self, device_id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.scan();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Finish scan
    pub fn finish_scan(&mut self, device_id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.finish_scan();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get devices by state
    pub fn get_devices_by_state(&self, state: &DeviceState) -> Vec<&WirelessDevice> {
        self.devices.values()
            .filter(|d| d.state == *state)
            .collect()
    }
    
    /// Get up devices
    pub fn get_up_devices(&self) -> Vec<&WirelessDevice> {
        self.get_devices_by_state(&DeviceState::Up)
    }
    
    /// Get connected devices
    pub fn get_connected_devices(&self) -> Vec<&WirelessDevice> {
        self.get_devices_by_state(&DeviceState::Connected)
    }
    
    // Network methods
    
    /// Add network to device
    pub fn add_network(&mut self, device_id: &str, ssid: String, bssid: String) -> Option<String> {
        if let Some(networks) = self.networks.get_mut(device_id) {
            let network = WirelessNetwork::new(ssid.clone(), bssid);
            let id = format!("{}-{}", device_id, ssid);
            networks.push(network);
            self.update_statistics();
            return Some(id);
        }
        None
    }
    
    /// Scan networks
    pub fn scan_networks(&mut self, device_id: &str) -> usize {
        self.scan_device(device_id);
        self.networks.get(device_id)
            .map(|n| n.len())
            .unwrap_or(0)
    }
    
    /// Get networks for device
    pub fn get_networks(&self, device_id: &str) -> Vec<&WirelessNetwork> {
        self.networks.get(device_id)
            .map(|n| n.iter().collect())
            .unwrap_or_default()
    }
    
    /// Get all networks
    pub fn list_networks(&self) -> Vec<&WirelessNetwork> {
        self.networks.values()
            .flat_map(|n| n.iter())
            .collect()
    }
    
    /// Get visible networks
    pub fn get_visible_networks(&self) -> Vec<&WirelessNetwork> {
        self.list_networks()
            .into_iter()
            .filter(|n| n.is_visible())
            .collect()
    }
    
    /// Get connected networks
    pub fn get_connected_networks(&self) -> Vec<&WirelessNetwork> {
        self.list_networks()
            .into_iter()
            .filter(|n| n.is_connected())
            .collect()
    }
    
    /// Search networks
    pub fn search_networks(&self, query: &str) -> Vec<&WirelessNetwork> {
        let query_lower = query.to_lowercase();
        self.list_networks()
            .into_iter()
            .filter(|n| n.ssid.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Connection methods
    
    /// Connect to network
    pub fn connect(&mut self, device_id: &str, ssid: String, password: Option<String>) -> String {
        let mut conn = Connection::new(ssid.clone());
        conn.set_device(device_id.to_string());
        
        if let Some(pw) = password {
            conn.config.password = pw;
        }
        conn.config.ssid = ssid;
        
        conn.connect();
        conn.finish_connecting();
        
        let id = conn.id.clone();
        self.connections.insert(id.clone(), conn);
        self.update_statistics();
        id
    }
    
    /// Get connection
    pub fn get_connection(&self, conn_id: &str) -> Option<&Connection> {
        self.connections.get(conn_id)
    }
    
    /// Get connection mutably
    pub fn get_connection_mut(&mut self, conn_id: &str) -> Option<&mut Connection> {
        self.connections.get_mut(conn_id)
    }
    
    /// Get all connections
    pub fn list_connections(&self) -> Vec<&Connection> {
        self.connections.values().collect()
    }
    
    /// Remove connection
    pub fn remove_connection(&mut self, conn_id: &str) -> bool {
        if self.connections.remove(conn_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Disconnect
    pub fn disconnect(&mut self, conn_id: &str) -> bool {
        if let Some(conn) = self.connections.get_mut(conn_id) {
            conn.disconnect();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get connections by state
    pub fn get_connections_by_state(&self, state: &ConnectionState) -> Vec<&Connection> {
        self.connections.values()
            .filter(|c| c.state == *state)
            .collect()
    }
    
    /// Get active connections
    pub fn get_active_connections(&self) -> Vec<&Connection> {
        self.get_connections_by_state(&ConnectionState::Connected)
    }
    
    // Stats methods
    
    /// Set link stats
    pub fn set_link_stats(&mut self, device_id: &str, stats: LinkStats) -> bool {
        self.link_stats.insert(device_id.to_string(), stats);
        true
    }
    
    /// Get link stats
    pub fn get_link_stats(&self, device_id: &str) -> Option<&LinkStats> {
        self.link_stats.get(device_id)
    }
    
    /// Set station stats
    pub fn set_station_stats(&mut self, device_id: &str, stats: StationStats) -> bool {
        self.station_stats.insert(device_id.to_string(), stats);
        true
    }
    
    /// Get station stats
    pub fn get_station_stats(&self, device_id: &str) -> Option<&StationStats> {
        self.station_stats.get(device_id)
    }
    
    // Statistics methods
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_devices = self.devices.len();
        self.statistics.up_devices = self.devices.values()
            .filter(|d| d.is_up())
            .count();
        self.statistics.connected_devices = self.devices.values()
            .filter(|d| d.is_connected())
            .count();
        self.statistics.total_networks = self.networks.values()
            .map(|n| n.len())
            .sum();
        self.statistics.total_connections = self.connections.len();
        self.statistics.active_connections = self.connections.values()
            .filter(|c| c.is_connected())
            .count();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            devices: self.devices.values().cloned().collect(),
            connections: self.connections.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.devices.clear();
        for device in export.devices {
            self.devices.insert(device.name.clone(), device);
        }
        
        self.connections.clear();
        for conn in export.connections {
            self.connections.insert(conn.id.clone(), conn);
        }
        
        self.update_statistics();
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Devices
    pub devices: Vec<WirelessDevice>,
    /// Connections
    pub connections: Vec<Connection>,
}
