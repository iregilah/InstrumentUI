// src/main.rs
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/InstrumentUI/qml/main.qml"));
        engine.load(&QUrl::from("qrc:/qt/qml/InstrumentUI/qml/awg.qml"));
    }
    if let Some(engine) = engine.as_mut() {
        engine.as_qqmlengine().on_quit(|_| println!("QML Quit!")).release();
    }
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
