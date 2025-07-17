
// examples/measure_demo.rs
use rigol_cli::aggregator::Aggregator;
use rigol_cli::lxi::{send_scpi, query_scpi};
use std::time::Duration;
use std::thread;
use tokio::time::sleep;
use rusb;

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
            handle.write_bulk(bulk_out, &data, Duration::from_secs(1))?;
            if scpi.trim_end().ends_with('?') {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ip_addr = None;
    let mut use_usb = false;
    let mut agg = Aggregator::new()?;
    let devices = agg.discover_all();
    if let Some((_, info)) = devices.iter().find(|(_, info)| info.instrument_type.as_deref() == Some("Oscilloscope")) {
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

    println!("=== MEASURE DEMO ===");
    let freq = 1000.0;
    let vpp = 2.0;
    if use_usb {
        send_scpi_via_usb(USB_VID, USB_PID, &format!(":SOUR1:APPL:SIN {freq},{vpp},0,0"))?;
        send_scpi_via_usb(USB_VID, USB_PID, ":OUTPUT1 ON")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":CHAN1:DISP ON")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":CHAN1:SCAL 0.5")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":CHAN1:OFFS 0")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":TIM:SCAL 0.0002")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":TRIG:MODE EDGE")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":TRIG:EDGE:SOUR CHAN1")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":TRIG:EDGE:SLOP POS")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":TRIG:EDGE:LEV 0")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":SINGLE")?;
        thread::sleep(Duration::from_secs(1));
        if let Some(freq_meas) = send_scpi_via_usb(USB_VID, USB_PID, ":MEAS:ITEM? FREQ,CHAN1")? {
            println!(">> Mért frekvencia = {} Hz", freq_meas.trim_end());
        }
        if let Some(vpp_meas) = send_scpi_via_usb(USB_VID, USB_PID, ":MEAS:ITEM? PKPK,CHAN1")? {
            println!(">> Mért csúcs‑csúcs feszültség = {} V", vpp_meas.trim_end());
        }
        send_scpi_via_usb(USB_VID, USB_PID, ":OUTPUT1 OFF")?;
        println!("=== DEMO END ===");
        return Ok(());
    }

    let addr = ip_addr.unwrap().parse()?;
    send_scpi(&addr, &format!(":SOUR1:APPL:SIN {freq},{vpp},0,0")).await?;
    send_scpi(&addr, ":OUTPUT1 ON").await?;
    send_scpi(&addr, ":CHAN1:DISP ON").await?;
    send_scpi(&addr, ":CHAN1:SCAL 0.5").await?;
    send_scpi(&addr, ":CHAN1:OFFS 0").await?;
    send_scpi(&addr, ":TIM:SCAL 0.0002").await?;
    send_scpi(&addr, ":TRIG:MODE EDGE").await?;
    send_scpi(&addr, ":TRIG:EDGE:SOUR CHAN1").await?;
    send_scpi(&addr, ":TRIG:EDGE:SLOP POS").await?;
    send_scpi(&addr, ":TRIG:EDGE:LEV 0").await?;
    send_scpi(&addr, ":SINGLE").await?;
    sleep(Duration::from_secs(1)).await;
    let freq_meas = query_scpi(&addr, ":MEAS:ITEM? FREQ,CHAN1").await?;
    println!(">> Mért frekvencia = {} Hz", freq_meas.trim_end());
    let vpp_meas = query_scpi(&addr, ":MEAS:ITEM? PKPK,CHAN1").await?;
    println!(">> Mért csúcs‑csúcs feszültség = {} V", vpp_meas.trim_end());
    send_scpi(&addr, ":OUTPUT1 OFF").await?;
    println!("=== DEMO END ===");
    Ok(())
}