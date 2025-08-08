// src/main.rs

mod oscillo_object; // <- a Rust bridge modulok ténylegesen bekerülnek a binárisba
mod awg_object;

use awg_object::ffi as awg_ffi;
use oscillo_object::ffi as osc_ffi;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use std::fs;

fn main() {
    // Stílus beállítása configból vagy Material alapértelmezéssel
    if let Ok(style) = fs::read_to_string("style.conf") {
        let style = style.trim();
        if !style.is_empty() {
            unsafe {
                std::env::set_var("QT_QUICK_CONTROLS_STYLE", style);
            }
            println!("[INIT] Style from config: {}", style);
        }
    } else {
        unsafe {
            std::env::set_var("QT_QUICK_CONTROLS_STYLE", "Material");
        }
        println!("[INIT] Default style 'Material' applied");
    }

    // A típusok kézi regisztrálása: így a Rust oldali szimbólumok biztosan hivatkozva lesznek
    // (MSVC/OPT:REF mellett sem esnek ki linkeléskor), és QML-ből is elérhetők.
    println!("[INIT] Registering QML types …");
    osc_ffi::register_types();
    awg_ffi::register_types();

    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(mut eng) = engine.as_mut() {
        let main_url = QUrl::from("qrc:/qt/qml/InstrumentUI/qml/main.qml");
        let awg_url  = QUrl::from("qrc:/qt/qml/InstrumentUI/qml/awg.qml");

        // Reborrow minden hívás előtt, hogy a Pin<&mut _> ne "move"-olódjon
        eng.as_mut().load(&main_url);
        println!("[QML] Main UI loaded");

        eng.as_mut().load(&awg_url);
        println!("[QML] Function Generator UI loaded");
    } else {
        eprintln!("[ERR] QQmlApplicationEngine not created");
    }

    if let Some(engine) = engine.as_mut() {
        engine
            .as_qqmlengine()
            .on_quit(|_| println!("[EXIT] Application exited"))
            .release();
    }

    if let Some(app) = app.as_mut() {
        println!("[RUN] Entering Qt event loop");
        app.exec();
        println!("[RUN] Qt event loop returned");
    } else {
        eprintln!("[ERR] QGuiApplication not created");
    }
}
