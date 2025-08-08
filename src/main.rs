// src/main.rs
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use std::env;

fn main() {
    // Set default style to Material Dark theme
    unsafe {
        env::set_var("QT_QUICK_CONTROLS_STYLE", "Material");
        env::set_var("QT_QUICK_CONTROLS_MATERIAL_THEME", "Dark");
    }
    println!("Starting Instrument_UI with Material Dark theme");
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/InstrumentUI/qml/main.qml"));
    }
    if let Some(engine) = engine.as_mut() {
        engine.as_qqmlengine().on_quit(|_| println!("QML Quit!")).release();
    }
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
