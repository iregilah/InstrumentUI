// src/main.rs
pub mod oscilloscope_ui;
pub mod function_generator_ui;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    // Create the Qt application and QML engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML files for both windows
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/instrument/ui/qml/oscilloscope.qml"));
        engine.load(&QUrl::from("qrc:/qt/qml/com/instrument/ui/qml/function_generator.qml"));
    }

    // Start the Qt event loop
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
