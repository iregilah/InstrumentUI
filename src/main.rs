// src/main.rs

mod oscillo_object; // a bridge modulok ténylegesen bekerülnek a binárisba
mod awg_object;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use std::{
    env, fs,
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    time::Duration,
};

/// „1.2.3.4” → „1.2.3.4:5555”, illetve teljes `IP:port` normalizálása.
fn normalize_addr(s: &str) -> SocketAddr {
    let s = if s.contains(':') { s.to_string() } else { format!("{s}:5555") };
    s.parse().expect("Érvénytelen IP:port formátum")
}

/// Parancssori cím felvétele (alap: 169.254.50.24:5555).
/// Támogatott formák:
///   - `prog 192.168.1.10`
///   - `prog 192.168.1.10:5555`
///   - `prog --addr 192.168.1.10[:port]` vagy `-a 192.168.1.10[:port]`
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

    "169.254.50.23:5555"
        .parse()
        .expect("hardcoded default addr must parse")
}

/// Gyors kapcsolódási próba; siker esetén az *IDN?* választ adja vissza.
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

/// Ha a cím 169.254.50.23/24, és az első próbálkozás nem sikerül,
/// megpróbáljuk az alternatív .23↔.24 végű IP‑t is.
fn with_23_24_fallback(addr: SocketAddr) -> SocketAddr {
    if let IpAddr::V4(ipv4) = addr.ip() {
        let o = ipv4.octets();
        if o[0] == 169 && o[1] == 254 && o[2] == 50 && (o[3] == 23 || o[3] == 24) {
            // először az eredetit próbáljuk
            if try_connect(addr, 700).is_ok() {
                return addr;
            }
            // alternatív végű IP
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
    // ➊ Stílus beállítása configból vagy Material alapértelmezéssel
    if let Ok(style) = fs::read_to_string("style.conf") {
        let style = style.trim();
        if !style.is_empty() {
            unsafe { env::set_var("QT_QUICK_CONTROLS_STYLE", style) };
            println!("[INIT] Style from config: {style}");
        } else {
            unsafe { env::set_var("QT_QUICK_CONTROLS_STYLE", "Material") };
            println!("[INIT] Default style 'Material' applied");
        }
    } else {
        unsafe { env::set_var("QT_QUICK_CONTROLS_STYLE", "Material") };
        println!("[INIT] Default style 'Material' applied");
    }

    // ➋ Cím felvétele a parancssorból (+ .23/.24 fallback), publikálás környezetváltozóként
    let mut addr = parse_addr_from_cli();
    addr = with_23_24_fallback(addr);

    // Rövid csatlakozási próba és napló
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

    // A bridge‑objektumok számára is elérhetővé tesszük (ha majd on_constructed használva lesz),
    // és a QML‑oldal későbbi, esetleges felhasználásához is.
    unsafe {
        env::set_var("INSTRUMENT_ADDR", addr.to_string());
        env::set_var("RIGOL_ADDR", addr.to_string());
    }

    // ➌ Qt app + QML engine
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

    // ➍ Qt event loop
    if let Some(app) = app.as_mut() {
        println!("[RUN] Entering Qt event loop");
        app.exec();
        println!("[RUN] Qt event loop returned");
    } else {
        eprintln!("[ERR] QGuiApplication not created");
    }
}
