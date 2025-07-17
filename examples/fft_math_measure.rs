
// examples/fft_math_demo.rs
use rigol_cli::aggregator::Aggregator;
use rigol_cli::lxi::{query_scpi, send_scpi};
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
    let args: Vec<String> = std::env::args().collect();
    let mut ip_addr = None;
    let mut use_usb = false;
    if let Some(i) = args.iter().position(|s| s == "--ip") {
        if let Some(val) = args.get(i+1) {
            ip_addr = Some(val.clone());
        }
    }
    if ip_addr.is_none() {
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
    }

    if use_usb {
        println!("=== FFT + MATH DEMO ===");
        send_scpi_via_usb(USB_VID, USB_PID, ":MATH:DISP ON")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":MATH:OPER FFT")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":MATH:FFT:SOUR CHAN1")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":MATH:FFT:WIND HANN")?;
        println!(">> FFT – kék spektrum CH1‑ről,  Hanning ablak.");
        thread::sleep(Duration::from_secs(3));

        send_scpi_via_usb(USB_VID, USB_PID, ":MATH:OPER MULT")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":MATH:SOUR1 CHAN1")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":MATH:SOUR2 CHAN2")?;
        println!(">> MATH MULT – rózsaszín görbe megjelenik.");
        thread::sleep(Duration::from_secs(3));

        if let Some(vpp) = send_scpi_via_usb(USB_VID, USB_PID, ":MEAS:ITEM? PKPK,MATH")? {
            println!(">> MATH Pk‑Pk = {} V", vpp.trim());
        }

        send_scpi_via_usb(USB_VID, USB_PID, ":MATH:DISP OFF")?;
        println!("=== DEMO END ===");
        return Ok(());
    }

    let addr = ip_addr.unwrap().parse()?;
    println!("=== FFT + MATH DEMO ===");
    send_scpi(&addr, ":MATH:DISP ON").await?;
    send_scpi(&addr, ":MATH:OPER FFT").await?;
    send_scpi(&addr, ":MATH:FFT:SOUR CHAN1").await?;
    send_scpi(&addr, ":MATH:FFT:WIND HANN").await?;
    println!(">> FFT – kék spektrum CH1‑ről,  Hanning ablak.");
    sleep(Duration::from_secs(3)).await;

    send_scpi(&addr, ":MATH:OPER MULT").await?;
    send_scpi(&addr, ":MATH:SOUR1 CHAN1").await?;
    send_scpi(&addr, ":MATH:SOUR2 CHAN2").await?;
    println!(">> MATH MULT – rózsaszín görbe megjelenik.");
    sleep(Duration::from_secs(3)).await;

    let vpp = query_scpi(&addr, ":MEAS:ITEM? PKPK,MATH").await?;
    println!(">> MATH Pk‑Pk = {} V", vpp.trim());

    send_scpi(&addr, ":MATH:DISP OFF").await?;
    println!("=== DEMO END ===");
    Ok(())
}