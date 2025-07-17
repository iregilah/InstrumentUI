// examples/dmm_dcv.rs
use rigol_cli::aggregator::Aggregator;
use rigol_cli::lxi::query_scpi;
use rusb;
use std::net::SocketAddr;

const USB_VID: u16 = 0x1AB1;
const USB_PID: u16 = 0x04CE;

fn send_scpi_via_usb(vid: u16, pid: u16, scpi: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let devices = rusb::devices()?;
    for device in devices.iter() {
        let dd = device.device_descriptor()?;
        if dd.vendor_id() == vid && dd.product_id() == pid {
            let mut handle = device.open()?;
            if let Err(_) = handle.claim_interface(0) {
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
            let mut data = scpi.as_bytes().to_vec();
            if !data.ends_with(&[b'\n']) {
                data.push(b'\n');
            }
            handle.write_bulk(bulk_out, &data, std::time::Duration::from_secs(1))?;
            if scpi.trim_end().ends_with('?') {
                let mut buf = [0u8; 1024];
                let len = handle.read_bulk(bulk_in, &mut buf, std::time::Duration::from_secs(1))?;
                let response = String::from_utf8_lossy(&buf[..len]).to_string();
                return Ok(Some(response));
            } else {
                return Ok(None);
            }
        }
    }
    Err("USB device not found or not accessible".into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ip_addr = None;
    let mut use_usb = false;
    if let Some(arg) = std::env::args().nth(1) {
        if !arg.starts_with('-') {
            ip_addr = Some(arg);
        }
    }
    if ip_addr.is_none() {
        let mut agg = Aggregator::new()?;
        let devices = agg.discover_all();
        if let Some((_, info)) = devices.iter().find(|(_, info)| info.instrument_type.as_deref() == Some("Multimeter") || info.model.as_deref().map(|m| m.starts_with("DM")) == Some(true)) {
            if info.interface.starts_with("USB") {
                use_usb = true;
            } else if info.interface.to_ascii_uppercase().contains("LXI") {
                let ident = &info.identifier;
                ip_addr = Some(if ident.contains(':') { ident.clone() } else { format!("{}:5555", ident) });
            }
        }
        if !use_usb && ip_addr.is_none() {
            ip_addr = Some("169.254.50.23:5555".to_string());
        }
    }

    if use_usb {
        if let Some(resp) = send_scpi_via_usb(USB_VID, USB_PID, ":MEAS:VOLT:DC?")? {
            println!("DC Voltage: {}", resp.trim_end());
        } else {
            println!("DC Voltage: <nincs válasz>");
        }
        return Ok(());
    }

    let addr: SocketAddr = ip_addr.unwrap().parse()?;
    let v = query_scpi(&addr, ":MEAS:VOLT:DC?").await?;
    println!("DC Voltage: {}", v.trim_end());
    Ok(())
}
