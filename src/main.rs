//! src/main.rs
use once_cell::sync::OnceCell;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use std::net::SocketAddr;

// Global static for instrument address (set at startup)
static ADDRESS: OnceCell<SocketAddr> = OnceCell::new();

fn main() {
    // Parse instrument address from command-line or use default
    let args: Vec<String> = std::env::args().collect();
    let addr_str = match args.get(1) {
        Some(a) if !a.starts_with('-') => {
            if a.contains(':') { a.clone() } else { format!("{a}:5555") }
        }
        _ => "169.254.50.23:5555".to_string(),
    };
    let addr: SocketAddr = addr_str.parse().expect("Invalid socket address");
    ADDRESS.set(addr).ok();

    // Create Qt application and QML engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
    if let Some(engine) = engine.as_mut() {
        // Load both QML UI files (two windows)
        engine.load(&QUrl::from("qrc:/qt/qml/InstrumentUI/qml/OscilloscopeUI.qml"));
        engine.load(&QUrl::from("qrc:/qt/qml/InstrumentUI/qml/FunctionGeneratorUI.qml"));
    }
    if let Some(app) = app.as_mut() {
        // Start the Qt event loop
        app.exec();
    }
}
