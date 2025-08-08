// src/main.rs

mod oscillo_object; // a bridge modulok ténylegesen bekerülnek a binárisba
mod awg_object;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use std::{env, fs, net::SocketAddr};

/// Parancssori cím felvétele (alapesetben 169.254.50.23:5555)
fn parse_addr_from_cli() -> SocketAddr {
    // Elfogadjuk az első nem kapcsoló argumentumot, illetve a --addr/-a formát
    let mut args = env::args().skip(1);

    // 1) --addr/-a <cím>
    while let Some(a) = args.next() {
        if a == "--" {
            break;
        }
        if a == "--addr" || a == "-a" {
            if let Some(val) = args.next() {
                return normalize_addr(&val);
            }
        }
        // első nem kapcsoló argumentum: cím
        if !a.starts_with('-') {
            return normalize_addr(&a);
        }
    }

    // 2) nincs argumentum → alapértelmezés
    "169.254.50.24:5555"
        .parse()
        .expect("hardcoded default addr must parse")
}

/// „1.2.3.4” → „1.2.3.4:5555”
fn normalize_addr(s: &str) -> SocketAddr {
    let s = if s.contains(':') { s.to_string() } else { format!("{s}:5555") };
    s.parse().expect("Érvénytelen IP:port formátum")
}

fn main() {
    // ➊ Stílus beállítása configból vagy Material alapértelmezéssel
    if let Ok(style) = fs::read_to_string("style.conf") {
        let style = style.trim();
        if !style.is_empty() {
            // Qt a processz indulásakor olvassa, ezért még app létrehozása előtt állítjuk
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

    // ➋ Cím felvétele a parancssorból és publikálása (a bridge-ek ebben fognak olvasni)
    let addr = parse_addr_from_cli();
    println!("[NET] Instrument address: {addr}");
    // A bridge-objektumok a saját (helyesen elnevezett) on_constructed‑jükben
    // std::env::args()-ból olvasnak. Ezen kívül környezeti változóként is
    // beállítjuk, ha később jól jönne.
    unsafe {
        env::set_var("INSTRUMENT_ADDR", addr.to_string());
    }
    // ➌ Qt app + QML engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(mut eng) = engine.as_mut() {
        // Mindkét QML betöltése
        let main_url = QUrl::from("qrc:/qt/qml/InstrumentUI/qml/main.qml");
        let awg_url  = QUrl::from("qrc:/qt/qml/InstrumentUI/qml/awg.qml");

        eng.as_mut().load(&main_url);
        println!("[QML] Main UI loaded");
        eng.as_mut().load(&awg_url);
        println!("[QML] Function Generator UI loaded");

        // Jelzés, ha az ablakot bezárják
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
