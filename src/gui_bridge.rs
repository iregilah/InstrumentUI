//! Qt‑bridge – Backend QObject, amelyből a QML egyetlen
//! gombnyomásra lefuttatja a „BASIC DEMO” szkriptet.

#[cxx_qt::bridge]
mod ffi {
    /// A Qt és a Rust közötti FFI‑deklarációk.
    extern "RustQt" {
        /// A QML‑ben `Backend` néven jelenik meg,
        /// `import RigolApp 1.0` modul alatt.
        #[qobject(qml_uri = "RigolApp", qml_version = "1.0")]
        type Backend = super::BackendRust;

        /// QML‑ből hívható metódus (Q_INVOKABLE).
        #[qinvokable]
        fn run_demo(self: &Backend);
    }
}

/// A valódi Rust‑oldali implementációs struktúra.
/// (Lehetnek mezők is, itt most nincs szükség rájuk.)
#[derive(Default)]
pub struct BackendRust;

impl ffi::qobject::Backend {
    /// A QML‑ből hívható függvény törzse.
    pub fn run_demo(&self) {
        if let Err(e) = run_basic_demo() {
            eprintln!("Demo error: {e:?}");
        }
    }
}

/* ---------- Innentől tiszta Rust‑kód, már nem a bridge része ---------- */

use rigol_cli::aggregator::Aggregator;
use rusb;
use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
    thread,
    time::Duration,
};

/// A teljes BASIC DEMO lejátszása (USB‑n vagy LXI‑n keresztül).
fn run_basic_demo() -> Result<(), Box<dyn std::error::Error>> {
    // ► Eszközfelderítés a rigol_cli könyvtárral
    let mut agg = Aggregator::new()?;
    let devices = agg.discover_all();

    let mut use_usb = false;
    let mut addr: Option<SocketAddr> = None;

    if let Some((_, info)) =
        devices.iter().find(|(_, i)| i.instrument_type.as_deref() == Some("Oscilloscope"))
    {
        if info.interface.starts_with("USB") {
            use_usb = true;
        } else if info.interface.to_ascii_uppercase().contains("LXI") {
            let ident = &info.identifier;
            addr = Some(
                if ident.contains(':') {
                    ident.parse()?
                } else {
                    format!("{ident}:5555").parse()?
                },
            );
        }
    }
    // Ha semmit sem találtunk → default link‑local IP
    if !use_usb && addr.is_none() {
        addr = Some("169.254.50.23:5555".parse()?);
    }

    const USB_VID: u16 = 0x1AB1;
    const USB_PID: u16 = 0x04CE;

    if use_usb {
        demo_usb(USB_VID, USB_PID)?;
    } else {
        demo_lxi(addr.expect("no LXI address found"))?;
    }
    Ok(())
}

/* --------------------------- USB útvonal ---------------------------- */

fn demo_usb(vid: u16, pid: u16) -> Result<(), Box<dyn std::error::Error>> {
    const CMDS: [&str; 4] = [
        ":CHAN1:DISP OFF",
        ":CHAN1:DISP ON",
        ":CHAN1:SCAL 1.5",
        ":AUTOSCALE",
    ];
    for (idx, cmd) in CMDS.iter().enumerate() {
        send_scpi_via_usb(vid, pid, cmd)?;
        println!(">> {cmd}");
        thread::sleep(Duration::from_secs(if idx == 3 { 2 } else { 1 }));
    }
    println!("=== BASIC DEMO END ===");
    Ok(())
}

fn send_scpi_via_usb(
    vid: u16,
    pid: u16,
    scpi: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let devices = rusb::devices()?;
    for device in devices.iter() {
        let dd = device.device_descriptor()?;
        if dd.vendor_id() == vid && dd.product_id() == pid {
            let mut handle = device.open()?;
            if handle.claim_interface(0).is_err() {
                handle.detach_kernel_driver(0).ok();
                handle.claim_interface(0)?;
            }
            let cfg = device.active_config_descriptor()?;
            let iface = cfg.interfaces().next().ok_or("no interface")?;
            let desc = iface.descriptors().next().ok_or("no descriptor")?;

            let (mut out_ep, mut in_ep) = (None, None);
            for ep in desc.endpoint_descriptors() {
                if ep.transfer_type() == rusb::TransferType::Bulk {
                    match ep.direction() {
                        rusb::Direction::Out => out_ep = Some(ep.address()),
                        rusb::Direction::In => in_ep = Some(ep.address()),
                    }
                }
            }
            let bulk_out = out_ep.ok_or("no OUT EP")?;
            let bulk_in = in_ep.ok_or("no IN EP")?;

            // ► Parancs kiküldése
            let mut data = scpi.as_bytes().to_vec();
            if !data.ends_with(&[b'\n']) {
                data.push(b'\n');
            }
            handle.write_bulk(bulk_out, &data, Duration::from_secs(1))?;

            // ► Ha kérdés volt, olvasunk választ
            if scpi.trim_end().ends_with('?') {
                let mut buf = [0u8; 1024];
                let len = handle.read_bulk(bulk_in, &mut buf, Duration::from_secs(1))?;
                return Ok(Some(String::from_utf8_lossy(&buf[..len]).to_string()));
            }
            return Ok(None);
        }
    }
    Err("USB device not found".into())
}

/* --------------------------- LXI ( TCP/IP ) útvonal ---------------- */

fn demo_lxi(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(addr)?;
    stream.set_write_timeout(Some(Duration::from_secs(1)))?;

    const CMDS: [&str; 4] = [
        ":CHAN1:DISP OFF\n",
        ":CHAN1:DISP ON\n",
        ":CHAN1:SCAL 1.5\n",
        ":AUTOSCALE\n",
    ];
    for (idx, cmd) in CMDS.iter().enumerate() {
        stream.write_all(cmd.as_bytes())?;
        println!(">> {}", cmd.trim());
        thread::sleep(Duration::from_secs(if idx == 3 { 2 } else { 1 }));
    }
    println!("=== BASIC DEMO END ===");
    Ok(())
}
