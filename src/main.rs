// src/main.rs
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

pub mod osc_controller;
pub mod awg_controller;

fn main() {
    // Create Qt application and QML engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load QML files for the two windows from resources
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/rigol/scope/qml/oscilloscope.qml"));
        engine.load(&QUrl::from("qrc:/qt/qml/com/rigol/scope/qml/function_generator.qml"));
    }

    // Optionally handle quit signal
    if let Some(engine) = engine.as_mut() {
        engine
            .as_qqmlengine()
            .on_quit(|_| println!("QML Quit!"))
            .release();
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
