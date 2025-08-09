mod oscillo_object;
mod awg_object;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use std::{
    env, fs,
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    time::Duration,
};

fn normalize_addr(s: &str) -> SocketAddr {
    let s = if s.contains(':') { s.to_string() } else { format!("{s}:5555") };
    s.parse().expect("Érvénytelen IP:port formátum")
}

fn parse_addr_from_cli() -> SocketAddr {
    let mut args = env::args().skip(1);

    while let Some(a) = args.next() {
        if a == "--" {
            break;
        }
        if a == "--addr" || a == "-a" {
            if let Some(val) = args.next() {
                return normalize_addr(&val);
            }
        }
        if !a.starts_with('-') {
            return normalize_addr(&a);
        }
    }

    "169.254.50.25:5555"
        .parse()
        .expect("hardcoded default addr must parse")
}

fn try_connect(addr: SocketAddr, timeout_ms: u64) -> std::io::Result<String> {
    let mut s = TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms))?;
    let _ = s.set_nodelay(true);
    let _ = s.set_read_timeout(Some(Duration::from_millis(timeout_ms)));
    let _ = s.set_write_timeout(Some(Duration::from_millis(timeout_ms)));
    let _ = s.write_all(b"*IDN?\n");
    let _ = s.flush();
    let mut buf = [0u8; 512];
    match s.read(&mut buf) {
        Ok(n) if n > 0 => Ok(String::from_utf8_lossy(&buf[..n]).trim().to_owned()),
        _ => Ok(String::new()),
    }
}

fn with_23_24_fallback(addr: SocketAddr) -> SocketAddr {
    if let IpAddr::V4(ipv4) = addr.ip() {
        let o = ipv4.octets();
        if o[0] == 169 && o[1] == 254 && o[2] == 50 && (o[3] == 23 || o[3] == 24) {
            if try_connect(addr, 700).is_ok() {
                return addr;
            }
            let alt_last = if o[3] == 23 { 24 } else { 23 };
            let alt = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(o[0], o[1], o[2], alt_last)), addr.port());
            if try_connect(alt, 700).is_ok() {
                return alt;
            }
        }
    }
    addr
}

fn main() {
    if let Ok(style) = fs::read_to_string("style.conf") {
        let style = style.trim();
        if !style.is_empty() {
            unsafe {
                env::set_var("QT_QUICK_CONTROLS_STYLE", style);
            }
            println!("[INIT] Style from config: {style}");
        } else {
            unsafe {
                env::set_var("QT_QUICK_CONTROLS_STYLE", "Material");
            }
            println!("[INIT] Default style 'Material' applied");
        }
    } else {
        unsafe {
            env::set_var("QT_QUICK_CONTROLS_STYLE", "Material");
        }
        println!("[INIT] Default style 'Material' applied");
    }

    let mut addr = parse_addr_from_cli();
    addr = with_23_24_fallback(addr);

    match try_connect(addr, 700) {
        Ok(idn) if !idn.is_empty() => {
            println!("[NET] Instrument address: {addr}  —  Connected: {idn}");
        }
        Ok(_) => {
            println!("[NET] Instrument address: {addr}  —  Connected (no IDN response)");
        }
        Err(e) => {
            println!("[NET] Instrument address: {addr}  —  Connection test failed: {e}");
        }
    }
    unsafe {
        env::set_var("INSTRUMENT_ADDR", addr.to_string());
        env::set_var("RIGOL_ADDR", addr.to_string());
        env::set_var("OSCILLOSCOPE_IP", addr.ip().to_string());
    }
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(mut eng) = engine.as_mut() {
        let main_url = QUrl::from("qrc:/qt/qml/InstrumentUI/qml/main.qml");
        let awg_url = QUrl::from("qrc:/qt/qml/InstrumentUI/qml/awg.qml");

        eng.as_mut().load(&main_url);
        println!("[QML] Main UI loaded");
        eng.as_mut().load(&awg_url);
        println!("[QML] Function Generator UI loaded");

        eng.as_mut()
            .as_qqmlengine()
            .on_quit(|_| println!("[EXIT] Application exited"))
            .release();
    } else {
        eprintln!("[ERR] QQmlApplicationEngine not created");
    }

    if let Some(mut app_pin) = app.as_mut() {
        println!("[RUN] Entering Qt event loop");
        app_pin.exec();
        println!("[RUN] Qt event loop returned");
    } else {
        eprintln!("[ERR] QGuiApplication not created");
    }
}
