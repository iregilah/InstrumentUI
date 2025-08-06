// src/main.rs
mod gui;  // the cxx-qt bridge module
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use std::net::SocketAddr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse instrument address (default if none provided)
    let args: Vec<String> = std::env::args().collect();
    let addr_str = if args.len() > 1 && !args[1].starts_with('-') {
        if args[1].contains(':') { args[1].clone() } else { format!("{}:5555", args[1]) }
    } else {
        "169.254.50.23:5555".to_string()
    };
    let addr: SocketAddr = addr_str.parse()?;
    // Store address in static for use by Backend
    gui::ffi::ADDR.set(addr).unwrap();

    // Create Qt application and QML engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
    if let Some(engine) = engine.as_mut() {
        // Load main window QML
        engine.load(&QUrl::from("qrc:/qt/qml/com/rust/instrument/qml/main.qml"));
        // Load function generator window QML
        engine.load(&QUrl::from("qrc:/qt/qml/com/rust/instrument/qml/function_generator.qml"));
        // Connect engine quit signal to set QUIT flag
        engine.as_qqmlengine().on_quit(|_| {
            gui::ffi::QUIT.store(true, std::sync::atomic::Ordering::SeqCst);
        }).release();
    }
    // Run the application event loop
    if let Some(app) = app.as_mut() {
        app.exec();
    }
    Ok(())
}
