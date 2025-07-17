// examples/sine_300khz_ch1.rs
use rigol_cli::aggregator::Aggregator;
use rigol_cli::lxi::send_scpi;
use std::{time::Duration, thread};
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

fn arg_after(flag: &str) -> Option<String> {
    let mut it = std::env::args().skip_while(|s| s != flag);
    it.next()?;
    it.next()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip_arg = arg_after("--ip").unwrap_or_else(|| "169.254.50.23:5555".into());
    let freq = arg_after("--freq").unwrap_or_else(|| "300000".into()).parse::<f64>()?;
    let vpp = arg_after("--vpp").unwrap_or_else(|| "2.0".into()).parse::<f64>()?;

    let mut ip_addr = None;
    let mut use_usb = false;
    if !ip_arg.is_empty() && !ip_arg.starts_with("169.254.50.23:5555") {
        ip_addr = Some(ip_arg);
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

    println!("Kapcsolódás: {}", if use_usb { "USB device" } else { ip_addr.as_deref().unwrap() });
    println!("AWG  : {} Hz  |  {} Vpp", freq, vpp);
    println!("Cél  : pontosan 2 periódus a 10 div‑en\n");

    if use_usb {
        send_scpi_via_usb(USB_VID, USB_PID, &format!(":SOUR1:APPL:SIN {freq},{vpp},0,0"))?;
        send_scpi_via_usb(USB_VID, USB_PID, ":OUTPUT1 ON")?;
        let v_scale = vpp / 4.0;
        send_scpi_via_usb(USB_VID, USB_PID, ":CHAN1:DISP ON")?;
        send_scpi_via_usb(USB_VID, USB_PID, &format!(":CHAN1:SCAL {v_scale}"))?;
        send_scpi_via_usb(USB_VID, USB_PID, ":CHAN1:OFFS 0")?;
        let t_scale_exact = 2.0 / freq / 10.0;
        send_scpi_via_usb(USB_VID, USB_PID, &format!(":TIM:SCAL {t_scale_exact}"))?;
        send_scpi_via_usb(USB_VID, USB_PID, ":TRIG:MODE EDGE")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":TRIG:EDGE:SOUR CHAN1")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":TRIG:EDGE:SLOP POS")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":TRIG:EDGE:LEV 0")?;
        send_scpi_via_usb(USB_VID, USB_PID, ":SINGLE")?;
        thread::sleep(Duration::from_millis(500));
        println!("Beállítás kész.");
        println!("Vertikális skála : {:.3} V/div  (eljárás = Vpp / 4)", v_scale);
        println!("Időskála         : {:.6e} s/div  (2 periódus / 10 div)", t_scale_exact);
        println!("Trigger          : CH1, pozitív él, 0 V");
        println!("\n✦ A kijelzőn 10 osztáson **pontosan két periódus** kell megjelenjen.");
        println!("✦ Ha a készülék a skálát kerekítené, ±1‑2 % eltérés még elfogadható.\n");
        return Ok(());
    }

    let addr: std::net::SocketAddr = ip_addr.unwrap().parse()?;
    send_scpi(&addr, &format!(":SOUR1:APPL:SIN {freq},{vpp},0,0")).await?;
    send_scpi(&addr, ":OUTPUT1 ON").await?;
    let v_scale = vpp / 4.0;
    send_scpi(&addr, ":CHAN1:DISP ON").await?;
    send_scpi(&addr, &format!(":CHAN1:SCAL {v_scale}")).await?;
    send_scpi(&addr, ":CHAN1:OFFS 0").await?;
    let t_scale_exact = 2.0 / freq / 10.0;
    send_scpi(&addr, &format!(":TIM:SCAL {t_scale_exact}")).await?;
    send_scpi(&addr, ":TRIG:MODE EDGE").await?;
    send_scpi(&addr, ":TRIG:EDGE:SOUR CHAN1").await?;
    send_scpi(&addr, ":TRIG:EDGE:SLOP POS").await?;
    send_scpi(&addr, ":TRIG:EDGE:LEV 0").await?;
    send_scpi(&addr, ":SINGLE").await?;
    sleep(Duration::from_millis(500)).await;
    println!("Beállítás kész.");
    println!("Vertikális skála : {:.3} V/div  (eljárás = Vpp / 4)", v_scale);
    println!("Időskála         : {:.6e} s/div  (2 periódus / 10 div)", t_scale_exact);
    println!("Trigger          : CH1, pozitív él, 0 V");
    println!("\n✦ A kijelzőn 10 osztáson **pontosan két periódus** kell megjelenjen.");
    println!("✦ Ha a készülék a skálát kerekítené, ±1‑2 % eltérés még elfogadható.\n");
    Ok(())
}