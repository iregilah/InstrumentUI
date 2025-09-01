// src/aggregator.rs

use std::collections::{HashMap, HashSet};
use std::fs;
use std::error::Error;
use serde_json::{Value, json};
use if_addrs::get_if_addrs;
use serialport::SerialPortType;
use std::net::IpAddr;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::instrument::Instrument;

pub fn start_capture_thread(instr: Arc<Mutex<Instrument>>) {
    println!("Starting capture thread for scope image...");
    thread::spawn(move || {
        println!("[AGR] Capture thread started");
        loop {
            println!("[AGR] Capture loop iteration begin");
            {
                let mut instrument = instr.lock().unwrap();
                println!("[AGR] Sending DISPlay:DATA? command");
                if let Err(e) = instrument.write(":DISP:DATA?") {
                    println!("[AGR] Error sending screenshot command: {}", e);
                    break;
                }
                println!("[AGR] Command sent, reading image data");
                match instrument.read_block() {
                    Ok(data) => {
                        println!("[AGR] {} bytes image data read", data.len());
                        if let Err(e) = std::fs::write("screenshot.png", &data) {
                            println!("[AGR] Failed to save image to file: {}", e);
                        } else {
                            println!("[AGR] Screenshot saved to screenshot.png");
                        }
                    }
                    Err(e) => {
                        println!("[AGR] Error reading image data: {}", e);
                        break;
                    }
                }
            }
            println!("[AGR] Capture thread sleeping");
            thread::sleep(Duration::from_secs(1));
        }
        println!("[AGR] Capture thread terminating due to error");
    });
}
// Communication layer trait for low-level interfaces
pub trait CommLayer {
    fn name(&self) -> &str;
    fn lsports(&self) -> Result<Vec<String>, Box<dyn Error>>;
    fn configure_port(&mut self, port: &str, settings: &Value) -> Result<(), Box<dyn Error>>;
    fn scan(&mut self, port: &str, id_range: Option<&Value>) -> Result<Vec<(String, String, Option<String>, Option<String>, Option<String>)>, Box<dyn Error>>;
    fn send(&mut self, port: &str, identifier: &str, message: &str) -> Result<Option<String>, Box<dyn Error>>;
}

// Record for a connected instrument in the database
#[derive(Debug, Clone)]
pub struct InstrumentInfo {
    pub interface: String,
    pub port: String,
    pub identifier: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub instrument_type: Option<String>,
}

// Aggregation layer struct managing connected instruments and interfaces
// Due to tests everything is pub here
pub struct Aggregator {
    pub connected_instruments: HashMap<u32, InstrumentInfo>,
    pub next_uuid: u32,
    pub comm_layers: Vec<Box<dyn CommLayer>>,
    pub config: Value,
}

impl Aggregator {
    /// Initializes an empty connected instrument database, sets nextUUID to 0,
    /// reads the config file and starts every enabled communication layer instance.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Initialize empty database and UUID counter
        let connected_instruments = HashMap::new();
        let next_uuid = 0;
        // Read configuration file (if missing or invalid, use default)
        let config_path = "config.json";
        let config_text = fs::read_to_string(config_path);
        let config: Value = match config_text {
            Ok(text) => serde_json::from_str(&text).unwrap_or_else(|e| {
                eprintln!("Config parse error: {}, using default configuration", e);
                json!({ "LXI": { "enabled": true } })
            }),
            Err(_) => {
                // No config file found
                json!({ "LXI": { "enabled": true } })
            }
        };
        // Prepare enabled communication layer instances
        let mut comm_layers: Vec<Box<dyn CommLayer>> = Vec::new();
        let is_enabled = |name: &str| {
            config.get(name)
                .and_then(|sect| sect.get("enabled"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        };
        // LXI interface (Ethernet/TCP)
        if is_enabled("LXI") {
            let allowed_nics: Option<HashSet<String>> = config.get("LXI")
                .and_then(|sect| sect.get("ports"))
                .and_then(|p| p.as_object())
                .map(|obj| obj.keys().cloned().collect());
            comm_layers.push(Box::new(LxiComm::new(allowed_nics)));
        }
        // USB interface
        if is_enabled("USB") {
            comm_layers.push(Box::new(UsbComm::new()));
        }
        // Serial (RS-485) interface
        if is_enabled("RS485") || is_enabled("Serial") {
            // Accept either "RS485" or "Serial" as config key for serial
            let serial_key = if config.get("RS485").is_some() { "RS485" } else { "Serial" };
            let allowed_ports: Option<HashSet<String>> = config.get(serial_key)
                .and_then(|sect| sect.get("ports"))
                .and_then(|p| p.as_object())
                .map(|obj| obj.keys().cloned().collect());
            comm_layers.push(Box::new(SerialComm::new(allowed_ports)));
        }
        // GPIB interface
        if is_enabled("GPIB") {
            comm_layers.push(Box::new(GpibComm::new()));
        }
        Ok(Aggregator {
            connected_instruments,
            next_uuid,
            comm_layers,
            config,
        })
    }

    /// Calls lsports() on all running communication layer instances and returns
    /// the combined interface list as a semicolon-separated string.
    pub fn lsif(&self) -> String {
        let mut entries: Vec<String> = Vec::new();
        for iface in &self.comm_layers {
            match iface.lsports() {
                Ok(ports) => {
                    for port in ports {
                        entries.push(format!("{} on {}", iface.name(), port));
                    }
                }
                Err(e) => {
                    eprintln!("Error listing ports for {}: {}", iface.name(), e);
                }
            }
        }
        entries.join("; ")
    }

    /// Configures a specific interface/port with the given settings.
    /// If interface/port/settings are None, configures all interfaces as per config file.
    pub fn configif(&mut self, interface: Option<&str>, port: Option<&str>, settings: Option<&Value>) -> Result<(), Box<dyn Error>> {
        // Helper to find a communication layer by interface name (case-insensitive)
        if let Some(if_name) = interface {
            // Configure specific interface
            let name_low = if_name.to_ascii_lowercase();
            let iface = self.comm_layers.iter_mut()
                .find(|iface| iface.name().to_ascii_lowercase().starts_with(&name_low))
                .map(|b| b.as_mut())
                .ok_or_else(|| format!("Interface {} not found or not running", if_name))?;
            if let Some(p_name) = port {
                // Configure one specific port of that interface
                let cfg_values = if let Some(cfg) = settings {
                    cfg.clone()
                } else {
                    // Use config file settings if available
                    self.config.get(if_name)
                        .and_then(|sect| sect.get("ports"))
                        .and_then(|ports| ports.get(p_name))
                        .cloned()
                        .unwrap_or_else(|| json!({}))
                };
                iface.configure_port(p_name, &cfg_values)?;
            } else {
                // Configure all ports of the specified interface from config
                if let Some(port_map) = self.config.get(if_name)
                    .and_then(|sect| sect.get("ports"))
                    .and_then(|p| p.as_object()) {
                    for (p_name, cfg) in port_map {
                        iface.configure_port(p_name, cfg)?;
                    }
                } else {
                    // If no ports specified in config, configure all detected ports with default settings
                    if let Ok(ports) = iface.lsports() {
                        for p in ports {
                            iface.configure_port(&p, &json!({}))?;
                        }
                    }
                }
            }
        } else {
            // No interface specified: configure all interfaces according to config
            for iface_box in &mut self.comm_layers {
                let if_name = iface_box.name();
                if let Some(port_map) = self.config.get(if_name)
                    .and_then(|sect| sect.get("ports"))
                    .and_then(|p| p.as_object()) {
                    for (p_name, cfg) in port_map {
                        iface_box.configure_port(p_name, cfg)?;
                    }
                } else {
                    if let Ok(ports) = iface_box.lsports() {
                        for p in ports {
                            iface_box.configure_port(&p, &json!({}))?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Discover all instruments on all interfaces and build the connected instrument database.
    pub fn discover_all(&mut self) -> &HashMap<u32, InstrumentInfo> {
        // Ensure all interfaces are configured (open ports if needed)
        let _ = self.configif(None, None, None);
        for iface in &mut self.comm_layers {
            if let Ok(ports) = iface.lsports() {
                for port in ports {
                    let if_name = iface.name().to_string();
                    // Determine config-specified identifier range for this port, if any
                    let port_key = if if_name.contains("LXI") && port.to_lowercase().starts_with("adapter ") {
                        port["adapter ".len()..].to_string()
                    } else {
                        port.clone()
                    };
                    let config_section = if if_name.to_uppercase().contains("RS-485") {
                        if self.config.get("RS485").is_some() { "RS485" } else { "Serial" }
                    } else {
                        &if_name
                    };
                    let id_range_val = self.config.get(config_section)
                        .and_then(|sect| sect.get("ports"))
                        .and_then(|ports| ports.get(&port_key))
                        .and_then(|p| p.get("id_range"));
                    match iface.scan(&port, id_range_val) {
                        Ok(found_list) => {
                            for (prt, identifier, vendor, model, instr_type) in found_list {
                                // Avoid duplicates if already present
                                if self.connected_instruments.values().any(|info|
                                info.interface == if_name && info.port == prt && info.identifier == identifier) {
                                    continue;
                                }
                                let uuid = self.next_uuid;
                                self.next_uuid += 1;
                                self.connected_instruments.insert(uuid, InstrumentInfo {
                                    interface: if_name.to_string(),
                                    port: prt,
                                    identifier,
                                    vendor,
                                    model,
                                    instrument_type: instr_type,
                                });
                            }
                        }
                        Err(e) => {
                            eprintln!("Error scanning {} on {}: {}", if_name, port, e);
                        }
                    }
                }
            } else {
                eprintln!("Error listing ports for {}", iface.name());
            }
        }
        &self.connected_instruments
    }

    /// Manually connect to an instrument on a given interface/port/identifier.
    /// Returns a reference to the database if successful, or None if not found.
    pub fn connect(&mut self, interface: &str, port: &str, identifier: &str) -> Option<&HashMap<u32, InstrumentInfo>> {
        // Configure the specified interface/port (open port if needed)
        if let Err(e) = self.configif(Some(interface), Some(port), None) {
            eprintln!("Error configuring {} {}: {}", interface, port, e);
            return None;
        }
        // Find the matching communication layer
        let iface = self.comm_layers.iter_mut()
            .find(|iface| iface.name().to_ascii_lowercase().starts_with(&interface.to_ascii_lowercase()))?;
        let if_name = iface.name().to_string();
        // Prepare identifier range for scanning (single identifier if given)
        let id_val = if identifier.is_empty() { None } else { Some(json!([identifier])) };
        let scan_result = iface.scan(port, id_val.as_ref());
        let devices = match scan_result {
            Ok(list) => list,
            Err(e) => {
                eprintln!("Error scanning {} {}: {}", interface, port, e);
                return None;
            }
        };
        if devices.is_empty() {
            return None;
        }
        // Use the first found instrument (there should be at most one)
        let (prt, ident, vendor, model, instr_type) = devices[0].clone();
        // Avoid adding duplicate if already connected
        if self.connected_instruments.values().any(|info|
        info.interface == if_name && info.port == prt && info.identifier == ident) {
            return Some(&self.connected_instruments);
        }
        let uuid = self.next_uuid;
        self.next_uuid += 1;
        self.connected_instruments.insert(uuid, InstrumentInfo {
            interface: if_name.to_string(),
            port: prt,
            identifier: ident,
            vendor,
            model,
            instrument_type: instr_type,
        });
        Some(&self.connected_instruments)
    }

    /// Manually disconnect an instrument by its UUID and remove it from the database.
    pub fn disconnect(&mut self, uuid: u32) -> bool {
        if let Some(info) = self.connected_instruments.remove(&uuid) {
            // If this was the last instrument on a serial port, close that port
            if info.interface.contains("RS-485") || info.interface.contains("Serial") {
                if let Some(serial_iface) = self.comm_layers.iter_mut().find(|iface| iface.name().to_uppercase().contains("RS-485")) {
                    let still_used = self.connected_instruments.values().any(|i|
                    i.interface == info.interface && i.port == info.port);
                    if !still_used {
                        // Close the serial port (ignore errors)
                        let _ = serial_iface.configure_port(&info.port, &json!({ "close": true }));
                    }
                }
            }
            true
        } else {
            false
        }
    }

    /// Disconnects an entire port or interface. If `port` is provided, disconnects all instruments on that interface and port.
    /// If `port` is None, disconnects all instruments on the specified interface.
    pub fn disconnect_interface(&mut self, interface: &str, port: Option<&str>) -> bool {
        let mut any_removed = false;
        // Gather all instrument UUIDs matching the criteria
        let to_remove: Vec<u32> = self.connected_instruments.iter()
            .filter_map(|(&uuid, info)| {
                if info.interface.to_ascii_lowercase().starts_with(&interface.to_ascii_lowercase()) &&
                    (port.is_none() || info.port == port.unwrap()) {
                    Some(uuid)
                } else {
                    None
                }
            })
            .collect();
        for uuid in to_remove {
            if self.disconnect(uuid) {
                any_removed = true;
            }
        }
        any_removed
    }

    /// Disconnects from all instruments in the database and removes all records.
    /// Note: nextUUID is not reset to 0.
    pub fn disconnect_all(&mut self) -> bool {
        let to_remove: Vec<u32> = self.connected_instruments.keys().cloned().collect();
        let mut any_removed = false;
        for uuid in to_remove {
            if self.disconnect(uuid) {
                any_removed = true;
            }
        }
        any_removed
    }

    /// Sends a message to one or multiple instruments specified by UUID(s).
    pub fn send_to(&mut self, uuids: &[u32], message: &str) -> Vec<(u32, Result<String, Box<dyn Error>>)> {
        let mut results = Vec::new();
        for &uuid in uuids {
            let info = match self.connected_instruments.get(&uuid) {
                Some(info) => info.clone(),
                None => {
                    results.push((uuid, Err(format!("Instrument UUID {} not found", uuid).into())));
                    continue;
                }
            };
            let iface = match self.comm_layers.iter_mut().find(|iface| iface.name().to_ascii_lowercase().starts_with(&info.interface.to_ascii_lowercase())) {
                Some(layer) => layer,
                None => {
                    results.push((uuid, Err(format!("Interface {} not running", info.interface).into())));
                    continue;
                }
            };
            match iface.send(&info.port, &info.identifier, message) {
                Ok(Some(resp)) => results.push((uuid, Ok(resp))),
                Ok(None) => results.push((uuid, Ok(String::new()))),
                Err(e) => results.push((uuid, Err(e))),
            }
        }
        results
    }

    /// Sends the message to every connected instrument.
    pub fn broadcast(&mut self, message: &str) -> Vec<(u32, Result<String, Box<dyn Error>>)> {
        let ids: Vec<u32> = self.connected_instruments.keys().cloned().collect();
        self.send_to(&ids, message)
    }

    /// Callback for incoming unsolicited data from any instrument.
    pub fn recvd(&mut self, uuid: u32, data: &[u8]) {
        if !self.connected_instruments.contains_key(&uuid) {
            eprintln!("Received data from unknown instrument UUID {}", uuid);
            return;
        }
        if let Ok(text) = std::str::from_utf8(data) {
            println!("(Unsolicited) [{}]: {}", uuid, text.trim());
        } else {
            println!("(Unsolicited) [{}]: {:?}", uuid, data);
        }
        // In a full implementation, this would propagate the data to higher layers.
    }
}

impl Drop for Aggregator {
    fn drop(&mut self) {
        // Destructor: ensure all instruments are disconnected and resources are freed
        let _ = self.disconnect_all();
    }
}

/* Communication layer implementations */

// LXI (TCP/LAN) communication layer
struct LxiComm {
    allowed_nics: Option<HashSet<String>>,
}

impl LxiComm {
    fn new(allowed: Option<HashSet<String>>) -> Self {
        LxiComm { allowed_nics: allowed }
    }
}

impl CommLayer for LxiComm {
    fn name(&self) -> &str {
        "LXI/TCP/MODBUS-TCP"
    }

    fn lsports(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let ifaces = get_if_addrs()?;
        let mut nic_names = HashSet::new();
        for iface in ifaces {
            // Filter out loopback addresses
            let ip_addr: IpAddr = match iface.addr {
                if_addrs::IfAddr::V4(v4) => v4.ip.into(),
                if_addrs::IfAddr::V6(v6) => v6.ip.into(),
            };
            if ip_addr.is_loopback() {
                continue;
            }
            nic_names.insert(iface.name);
        }
        let mut ports = Vec::new();
        for name in nic_names {
            if let Some(ref allowed) = self.allowed_nics {
                if !allowed.contains(&name) {
                    continue;
                }
            }
            ports.push(format!("adapter {}", name));
        }
        ports.sort();
        Ok(ports)
    }

    fn configure_port(&mut self, port: &str, settings: &Value) -> Result<(), Box<dyn Error>> {
        // Configure network adapter (e.g., IP settings) if supported - not implemented
        if settings.as_object().map_or(false, |m| !m.is_empty()) {
            eprintln!("LXI: ignoring unsupported config for adapter {}", port);
        }
        Ok(())
    }

    fn scan(&mut self, port: &str, id_range: Option<&Value>) -> Result<Vec<(String, String, Option<String>, Option<String>, Option<String>)>, Box<dyn Error>> {
        let mut found = Vec::new();
        // Collect addresses to scan from id_range
        let mut addresses: Vec<String> = Vec::new();
        if let Some(range) = id_range {
            if range.is_array() {
                for val in range.as_array().unwrap() {
                    if let Some(s) = val.as_str() {
                        addresses.push(s.to_string());
                    }
                }
            } else if let Some(s) = range.as_str() {
                addresses.push(s.to_string());
            }
        }
        if addresses.is_empty() {
            return Ok(found);
        }
        for addr in addresses {
            let target = if addr.contains(':') { addr.clone() } else { format!("{}:5555", addr) };
            if let Ok(mut stream) = std::net::TcpStream::connect_timeout(&target.parse()?, std::time::Duration::from_millis(500)) {
                stream.set_read_timeout(Some(std::time::Duration::from_millis(500)))?;
                let _ = stream.write_all(b"*IDN?\n");
                let mut buf = [0u8; 512];
                if let Ok(n) = stream.read(&mut buf) {
                    if n > 0 {
                        let resp = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                        if !resp.is_empty() {
                            let parts: Vec<&str> = resp.split(',').collect();
                            let vendor = parts.get(0).map(|s| s.trim().to_string());
                            let model = parts.get(1).map(|s| s.trim().to_string());
                            let instr_type = if let Some(m) = model.as_ref() {
                                if m.starts_with("DS") || m.starts_with("MSO") {
                                    Some("Oscilloscope".to_string())
                                } else if m.starts_with("DM") {
                                    Some("Multimeter".to_string())
                                } else if m.starts_with("DP") {
                                    Some("Power Supply".to_string())
                                } else if m.starts_with("DG") {
                                    Some("Signal Generator".to_string())
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            found.push((port.to_string(), addr.clone(), vendor, model, instr_type));
                        }
                    }
                }
            }
        }
        Ok(found)
    }

    fn send(&mut self, _port: &str, identifier: &str, message: &str) -> Result<Option<String>, Box<dyn Error>> {
        let target = if identifier.contains(':') {
            identifier.to_string()
        } else {
            format!("{}:5555", identifier)
        };
        let addr = target.parse()?;
        let mut stream = std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_secs(1))?;
        stream.set_read_timeout(Some(std::time::Duration::from_secs(1)))?;
        // Ensure message is terminated with newline
        let mut cmd = message.as_bytes().to_vec();
        if !cmd.ends_with(&[b'\n']) {
            cmd.push(b'\n');
        }
        stream.write_all(&cmd)?;
        stream.flush()?;
        // Determine if the message is a query (expects a response)
        let is_query = message.trim_end().ends_with('?');
        if !is_query {
            // If not a query, no response is expected. Return success with empty string.
            return Ok(Some(String::new()));
        }
        let mut buffer = [0u8; 1024];
        let n = stream.read(&mut buffer)?;
        if n > 0 {
            Ok(Some(String::from_utf8_lossy(&buffer[..n]).trim_end().to_string()))
        } else {
            Ok(None)
        }
    }
}

// USB communication layer
struct UsbComm;

impl UsbComm {
    fn new() -> Self { UsbComm }
}

impl CommLayer for UsbComm {
    fn name(&self) -> &str {
        "USB"
    }

    fn lsports(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let devices = rusb::devices()?;
        let mut buses = HashSet::new();
        for device in devices.iter() {
            buses.insert(device.bus_number());
        }
        let mut bus_list: Vec<_> = buses.into_iter().collect();
        bus_list.sort_unstable();
        let ports: Vec<String> = bus_list.iter()
            .map(|bus| format!("Bus {:03} Root Hub", bus))
            .collect();
        Ok(ports)
    }

    fn configure_port(&mut self, _port: &str, _settings: &Value) -> Result<(), Box<dyn Error>> {
        // USB ports do not require configuration
        Ok(())
    }

    fn scan(&mut self, _port: &str, id_range: Option<&Value>) -> Result<Vec<(String, String, Option<String>, Option<String>, Option<String>)>, Box<dyn Error>> {
        let mut found = Vec::new();
        let devices = rusb::devices()?;
        // Optional vendor/product filtering if id_range provided
        let mut filter_vendor = None;
        let mut filter_product = None;
        if let Some(range) = id_range {
            if range.is_array() && !range.as_array().unwrap().is_empty() {
                if let Some(val) = range.as_array().unwrap().get(0) {
                    if let Some(s) = val.as_str() {
                        if s.contains(':') {
                            let parts: Vec<&str> = s.split(':').collect();
                            if parts.len() == 2 {
                                if let Ok(v) = u16::from_str_radix(parts[0].trim_start_matches("0x"), 16).or_else(|_| parts[0].parse()) {
                                    if let Ok(p) = u16::from_str_radix(parts[1].trim_start_matches("0x"), 16).or_else(|_| parts[1].parse()) {
                                        filter_vendor = Some(v);
                                        filter_product = Some(p);
                                    }
                                }
                            }
                        } else if let Ok(v) = u16::from_str_radix(s.trim_start_matches("0x"), 16).or_else(|_| s.parse()) {
                            filter_vendor = Some(v);
                        }
                    }
                }
            }
        }
        for device in devices.iter() {
            let dd = device.device_descriptor()?;
            if dd.class_code() == 0x09 {
                continue; // skip hubs
            }
            if let Some(v) = filter_vendor {
                if dd.vendor_id() != v {
                    continue;
                }
                if let Some(p) = filter_product {
                    if dd.product_id() != p {
                        continue;
                    }
                }
            }
            if let Ok(handle) = device.open() {
                let mut vendor_str = String::new();
                let mut product_str = String::new();
                if dd.vendor_id() != 0 {
                    vendor_str = handle.read_manufacturer_string_ascii(&dd).unwrap_or_default();
                }
                if dd.product_id() != 0 {
                    product_str = handle.read_product_string_ascii(&dd).unwrap_or_default();
                }
                // Check for USBTMC class interface
                let mut is_instrument = false;
                if let Ok(config) = device.active_config_descriptor() {
                    for interface in config.interfaces() {
                        for setting in interface.descriptors() {
                            if setting.class_code() == 0xFE && setting.sub_class_code() == 0x03 {
                                is_instrument = true;
                                break;
                            }
                        }
                        if is_instrument { break; }
                    }
                }
                if !is_instrument {
                    let manu_up = vendor_str.to_ascii_uppercase();
                    if manu_up.contains("RIGOL") || manu_up.contains("KEYSIGHT") || manu_up.contains("TEKTRONIX") {
                        is_instrument = true;
                    }
                }
                if is_instrument {
                    let vendor = if !vendor_str.is_empty() { Some(vendor_str.trim().to_string()) } else { None };
                    let model = if !product_str.is_empty() { Some(product_str.trim().to_string()) } else { None };
                    let instr_type = if let Some(m) = model.as_ref() {
                        if m.starts_with("DS") || m.starts_with("MSO") {
                            Some("Oscilloscope".to_string())
                        } else if m.starts_with("DM") || m.contains("Multimeter") {
                            Some("Multimeter".to_string())
                        } else if m.starts_with("DP") || m.contains("Power") {
                            Some("Power Supply".to_string())
                        } else if m.starts_with("DG") {
                            Some("Signal Generator".to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let identifier = format!("Bus {:03} Device {:03}", device.bus_number(), device.address());
                    let port_str = format!("Bus {:03}", device.bus_number());
                    found.push((port_str, identifier, vendor, model, instr_type));
                }
            }
        }
        Ok(found)
    }

    fn send(&mut self, _port: &str, _identifier: &str, _message: &str) -> Result<Option<String>, Box<dyn Error>> {
        // USB communication not implemented
        Err("USB communication not implemented".into())
    }
}

// Serial (MODBUS-RTU/RS-485 over COM port) communication layer
struct SerialComm {
    allowed_ports: Option<HashSet<String>>,
    open_ports: HashMap<String, Box<dyn serialport::SerialPort>>,
}

impl SerialComm {
    fn new(allowed: Option<HashSet<String>>) -> Self {
        SerialComm {
            allowed_ports: allowed,
            open_ports: HashMap::new(),
        }
    }
}

impl CommLayer for SerialComm {
    fn name(&self) -> &str {
        "MODBUS-RTU/RS-485"
    }

    fn lsports(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let ports_info = serialport::available_ports()?;
        let mut ports = Vec::new();
        for info in ports_info {
            if let Some(ref allowed) = self.allowed_ports {
                if !allowed.contains(&info.port_name) {
                    continue;
                }
            }
            if matches!(info.port_type, SerialPortType::BluetoothPort) {
                // Skip Bluetooth virtual COM ports
                continue;
            }
            ports.push(info.port_name);
        }
        ports.sort();
        Ok(ports)
    }

    fn configure_port(&mut self, port: &str, settings: &Value) -> Result<(), Box<dyn Error>> {
        // If instructed to close the port, close it and return
        if settings.get("close").and_then(|v| v.as_bool()).unwrap_or(false) {
            if self.open_ports.contains_key(port) {
                self.open_ports.remove(port);
            }
            return Ok(());
        }
        // Parse serial settings or use defaults
        let baud = settings.get("baud").and_then(|v| v.as_u64()).unwrap_or(9600) as u32;
        let data_bits = settings.get("data_bits").and_then(|v| v.as_u64()).unwrap_or(8) as u8;
        let parity_str = settings.get("parity").and_then(|v| v.as_str()).unwrap_or("N");
        let stop_bits = settings.get("stop_bits").and_then(|v| v.as_u64()).unwrap_or(1) as u8;
        // Close existing port if already open
        if self.open_ports.contains_key(port) {
            self.open_ports.remove(port);
        }
        // Open serial port with specified settings
        let mut builder = serialport::new(port, baud);
        builder = builder.data_bits(match data_bits {
            5 => serialport::DataBits::Five,
            6 => serialport::DataBits::Six,
            7 => serialport::DataBits::Seven,
            _ => serialport::DataBits::Eight
        });
        builder = builder.parity(match parity_str.to_ascii_uppercase().as_str() {
            "E" | "EVEN" => serialport::Parity::Even,
            "O" | "ODD" => serialport::Parity::Odd,
            _ => serialport::Parity::None
        });
        builder = builder.stop_bits(if stop_bits == 2 {
            serialport::StopBits::Two
        } else {
            serialport::StopBits::One
        });
        builder = builder.timeout(std::time::Duration::from_millis(100));
        let port_handle = builder.open().map_err(|e| format!("Failed to open {}: {}", port, e))?;
        self.open_ports.insert(port.to_string(), port_handle);
        Ok(())
    }

    fn scan(&mut self, port: &str, id_range: Option<&Value>) -> Result<Vec<(String, String, Option<String>, Option<String>, Option<String>)>, Box<dyn Error>> {
        let mut found = Vec::new();
        // Open port if not already open
        if !self.open_ports.contains_key(port) {
            let _ = self.configure_port(port, &json!({}));
        }
        let port_handle = match self.open_ports.get_mut(port) {
            Some(p) => p,
            None => return Ok(found),
        };
        // Prepare identifier list (e.g., address IDs)
        let mut ids: Vec<String> = Vec::new();
        if let Some(range) = id_range {
            if range.is_array() {
                for val in range.as_array().unwrap() {
                    if let Some(n) = val.as_u64() {
                        ids.push(n.to_string());
                    } else if let Some(s) = val.as_str() {
                        ids.push(s.to_string());
                    }
                }
            } else if let Some(n) = range.as_u64() {
                ids.push(n.to_string());
            } else if let Some(s) = range.as_str() {
                ids.push(s.to_string());
            }
        }
        if ids.is_empty() {
            ids.push(String::new()); // no address specified, assume single device
        }
        for id in ids {
            let _ = port_handle.clear(serialport::ClearBuffer::Input);
            let query = b"*IDN?\r\n";
            port_handle.write_all(query)?;
            port_handle.flush()?;
            let mut response = Vec::new();
            let mut buf = [0u8; 256];
            let start = std::time::Instant::now();
            loop {
                match port_handle.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        response.extend_from_slice(&buf[..n]);
                        if response.contains(&b'\n') || response.contains(&b'\r') {
                            break;
                        }
                    }
                    Ok(_) => break,
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                        if start.elapsed() > std::time::Duration::from_millis(500) {
                            break;
                        }
                        continue;
                    }
                    Err(e) => return Err(Box::new(e)),
                }
            }
            if !response.is_empty() {
                let resp_str = String::from_utf8_lossy(&response).trim().to_string();
                if !resp_str.is_empty() {
                    let parts: Vec<&str> = resp_str.split(',').collect();
                    let vendor = parts.get(0).map(|s| s.trim().to_string());
                    let model = parts.get(1).map(|s| s.trim().to_string());
                    let instr_type = if let Some(m) = model.as_ref() {
                        if m.starts_with("DS") || m.starts_with("MSO") {
                            Some("Oscilloscope".to_string())
                        } else if m.starts_with("DM") {
                            Some("Multimeter".to_string())
                        } else if m.starts_with("DP") {
                            Some("Power Supply".to_string())
                        } else if m.starts_with("DG") {
                            Some("Signal Generator".to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let identifier = if id.is_empty() { "".to_string() } else { id.clone() };
                    found.push((port.to_string(), identifier, vendor, model, instr_type));
                    break;
                }
            }
        }
        Ok(found)
    }

    fn send(&mut self, port: &str, _identifier: &str, message: &str) -> Result<Option<String>, Box<dyn Error>> {
        if !self.open_ports.contains_key(port) {
            let _ = self.configure_port(port, &json!({}));
        }
        let port_handle = self.open_ports.get_mut(port).ok_or_else(|| format!("Port {} not available", port))?;
        // Clear any pending input
        let _ = port_handle.clear(serialport::ClearBuffer::Input);
        // Write the message with CR+LF termination
        let line = message.trim_end_matches(|c| c == '\r' || c == '\n');
        let cmd_line = format!("{}\r\n", line);
        port_handle.write_all(cmd_line.as_bytes())?;
        port_handle.flush()?;
        // Determine if the message is a query (expects a response)
        let is_query = message.trim_end_matches(|c| c == '\r' || c == '\n').ends_with('?');
        if !is_query {
            // If not a query, no response is expected. Return success with empty string.
            return Ok(Some(String::new()));
        }
        // Read response until newline or timeout
        let mut response = Vec::new();
        let start = std::time::Instant::now();
        let mut buf = [0u8; 256];
        loop {
            match port_handle.read(&mut buf) {
                Ok(n) if n > 0 => {
                    response.extend_from_slice(&buf[..n]);
                    if response.contains(&b'\n') || response.contains(&b'\r') {
                        break;
                    }
                }
                Ok(_) => break,
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    if start.elapsed() > std::time::Duration::from_millis(500) {
                        break;
                    }
                    continue;
                }
                Err(e) => return Err(Box::new(e)),
            }
        }
        if !response.is_empty() {
            Ok(Some(String::from_utf8_lossy(&response).trim_end().to_string()))
        } else {
            Ok(None)
        }
    }
}

// GPIB communication layer
struct GpibComm;

impl GpibComm {
    fn new() -> Self { GpibComm }
}

impl CommLayer for GpibComm {
    fn name(&self) -> &str {
        "GPIB"
    }

    fn lsports(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let devices = rusb::devices()?;
        let mut ports = Vec::new();
        for device in devices.iter() {
            let dd = device.device_descriptor()?;
            if dd.class_code() == 0x09 {
                // Skip USB hubs
                continue;
            }
            if let Ok(handle) = device.open() {
                let product = if dd.product_id() != 0 {
                    handle.read_product_string_ascii(&dd).unwrap_or_default()
                } else {
                    String::new()
                };
                let manufacturer = if dd.vendor_id() != 0 {
                    handle.read_manufacturer_string_ascii(&dd).unwrap_or_default()
                } else {
                    String::new()
                };
                let mut adapter_name = None;
                if product.to_ascii_uppercase().contains("GPIB") {
                    adapter_name = Some(product.trim().to_string());
                } else if manufacturer.to_ascii_uppercase().contains("GPIB") {
                    adapter_name = Some(format!("{} GPIB Adapter", manufacturer.trim()));
                }
                if let Some(name) = adapter_name {
                    ports.push(format!("{}, Bus {:03} Device {:03}",
                                       name, device.bus_number(), device.address()));
                }
            }
        }
        ports.sort();
        Ok(ports)
    }

    fn configure_port(&mut self, _port: &str, _settings: &Value) -> Result<(), Box<dyn Error>> {
        // No special configuration needed for GPIB interfaces
        Ok(())
    }

    fn scan(&mut self, _port: &str, _id_range: Option<&Value>) -> Result<Vec<(String, String, Option<String>, Option<String>, Option<String>)>, Box<dyn Error>> {
        // Scanning GPIB devices requires specific controller commands (not implemented)
        Ok(Vec::new())
    }

    fn send(&mut self, _port: &str, _identifier: &str, _message: &str) -> Result<Option<String>, Box<dyn Error>> {
        // GPIB communication not implemented
        Err("GPIB communication not implemented".into())
    }
}
