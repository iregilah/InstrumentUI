// src/main.rs
mod oscilloscope_backend;
mod function_generator_backend;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use std::env;
use std::net::SocketAddr;
use std::sync::Mutex;

// Global instrument address (set at startup)
pub static ADDR: Mutex<Option<SocketAddr>> = Mutex::new(None);

fn main() {
    // Parse instrument address from command line or use default
    let addr: SocketAddr = env::args()
        .nth(1)
        .unwrap_or_else(|| "169.254.50.23:5555".to_string())
        .parse()
        .expect("Invalid instrument address");
    *ADDR.lock().unwrap() = Some(addr);

    // Create Qt application and QML engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        // Load the QML UI files from the resource
        engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/cxx_qt/scope/qml/oscilloscope.qml"));
        engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/cxx_qt/scope/qml/function_generator.qml"));
        // Optional: handle quit signal
        engine
            .as_qqmlengine()
            .on_quit(|_| println!("QML Quit!"))
            .release();
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
