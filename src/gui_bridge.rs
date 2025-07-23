// src/gui_bridge.rs (CXX-Qt bridge module)
#[cxx_qt::bridge(cxx_file_stem = "gui_bridge")]
mod gui_bridge {
    // Import Rust backend types and modules
    use rigol_cli::aggregator::Aggregator;
    use std::net::SocketAddr;
    use std::thread;
    use std::time::Duration;
    use std::io::Write;
    use rusb;

    #[cxx_qt::qobject(qml_uri = "RigolApp", qml_version = "1.0")]
    pub struct Backend;

    impl Default for Backend {
        fn default() -> Self {
            Self {}
        }
    }

    impl qobject::Backend {
        /// Invokable method accessible from QML to run the basic demo.
        #[qinvokable]
        pub fn run_demo(&self) {
            if let Err(err) = run_basic_demo() {
                eprintln!("Demo error: {:?}", err);
            }
        }
    }

    /// Runs the BASIC DEMO sequence (turn CH1 off/on, set scale, autoscale).
    fn run_basic_demo() -> Result<(), Box<dyn std::error::Error>> {
        // Try to auto-detect a Rigol oscilloscope via USB or LXI
        let mut agg = Aggregator::new()?;
        let devices = agg.discover_all();
        let mut use_usb = false;
        let mut addr: Option<SocketAddr> = None;
        if let Some((_, info)) = devices.iter().find(|(_, info)| {
            info.instrument_type.as_deref() == Some("Oscilloscope")
        }) {
            if info.interface.starts_with("USB") {
                use_usb = true;
            } else if info.interface.to_ascii_uppercase().contains("LXI") {
                // Use the discovered network address (append port if missing)
                let ident = &info.identifier;
                let target = if ident.contains(':') {
                    ident.clone()
                } else {
                    format!("{}:5555", ident)
                };
                addr = Some(target.parse()?);
            }
        }
        if !use_usb && addr.is_none() {
            // Fallback to a default IP if no device was found
            addr = Some("169.254.50.23:5555".parse()?);
        }

        // Execute the SCPI commands sequence
        const USB_VID: u16 = 0x1AB1;
        const USB_PID: u16 = 0x04CE;
        if use_usb {
            // Send SCPI commands over USBTMC
            send_scpi_via_usb(USB_VID, USB_PID, ":CHAN1:DISP OFF")?;
            println!(">> CH1 OFF  – A nyomvonal el kell hogy tűnjön.");
            thread::sleep(Duration::from_secs(1));

            send_scpi_via_usb(USB_VID, USB_PID, ":CHAN1:DISP ON")?;
            println!(">> CH1 ON   – Nyomvonal visszatér.");
            thread::sleep(Duration::from_secs(1));

            send_scpi_via_usb(USB_VID, USB_PID, ":CHAN1:SCAL 1.5")?;
            println!(">> CH1 SCALE 1.5V/div");
            thread::sleep(Duration::from_secs(1));

            send_scpi_via_usb(USB_VID, USB_PID, ":AUTOSCALE")?;
            println!(">> AUTOSCALE – A scope újra­beállítja a skálákat.");
            thread::sleep(Duration::from_secs(2));

            println!("=== BASIC DEMO END ===");
        } else if let Some(addr) = addr {
            // Send SCPI commands over LXI (TCP/IP)
            use std::net::TcpStream;
            let mut stream = TcpStream::connect(addr)?;
            stream.set_write_timeout(Some(Duration::from_secs(1)))?;
            stream.set_read_timeout(Some(Duration::from_secs(1)))?;

            stream.write_all(b":CHAN1:DISP OFF\n")?;
            println!(">> CH1 OFF  – A nyomvonal el kell hogy tűnjön.");
            thread::sleep(Duration::from_secs(1));

            stream.write_all(b":CHAN1:DISP ON\n")?;
            println!(">> CH1 ON   – Nyomvonal visszatér.");
            thread::sleep(Duration::from_secs(1));

            stream.write_all(b":CHAN1:SCAL 1.5\n")?;
            println!(">> CH1 SCALE 1.5V/div");
            thread::sleep(Duration::from_secs(1));

            stream.write_all(b":AUTOSCALE\n")?;
            println!(">> AUTOSCALE – A scope újra­beállítja a skálákat.");
            thread::sleep(Duration::from_secs(2));

            println!("=== BASIC DEMO END ===");
        }
        Ok(())
    }

    /// Helper function to send a SCPI command via USBTMC using rusb.
    fn send_scpi_via_usb(vid: u16, pid: u16, scpi: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let devices = rusb::devices()?;
        for device in devices.iter() {
            let dd = device.device_descriptor()?;
            if dd.vendor_id() == vid && dd.product_id() == pid {
                let mut handle = device.open()?;
                if handle.claim_interface(0).is_err() {
                    handle.detach_kernel_driver(0).ok();
                    handle.claim_interface(0)?;
                }
                let config = device.active_config_descriptor()?;
                let interface = config.interfaces().next().ok_or("No USB interface")?;
                let descriptor = interface.descriptors().next().ok_or("No interface descriptor")?;
                let mut bulk_out = None;
                let mut bulk_in = None;
                for ep in descriptor.endpoint_descriptors() {
                    if ep.transfer_type() == rusb::TransferType::Bulk {
                        if ep.direction() == rusb::Direction::Out {
                            bulk_out = Some(ep.address());
                        } else if ep.direction() == rusb::Direction::In {
                            bulk_in = Some(ep.address());
                        }
                    }
                }
                let bulk_out = bulk_out.ok_or("No bulk OUT endpoint")?;
                let bulk_in = bulk_in.ok_or("No bulk IN endpoint")?;
                // Prepare command with newline terminator
                let mut data = scpi.as_bytes().to_vec();
                if !data.ends_with(&[b'\n']) {
                    data.push(b'\n');
                }
                handle.write_bulk(bulk_out, &data, Duration::from_secs(1))?;
                if scpi.trim_end().ends_with('?') {
                    // If it's a query, read the response
                    let mut buf = [0u8; 1024];
                    let len = handle.read_bulk(bulk_in, &mut buf, Duration::from_secs(1))?;
                    let response = String::from_utf8_lossy(&buf[..len]).to_string();
                    return Ok(Some(response));
                } else {
                    return Ok(None);
                }
            }
        }
        Err("USB device not found or not accessible".into())
    }
}