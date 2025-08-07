// src/main.rs
mod oscillo_object;
mod awg_object;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
        for path in [
            "qrc:/qt/qml/InstrumentUI/qml/main.qml",
            "qrc:/qt/qml/InstrumentUI/qml/awg.qml",
        ] {
            if let Some(eng) = engine.as_mut() {
                eng.load(&QUrl::from(path));
            }
        }

        // on_quit callback – szintén új as_mut()
        if let Some(eng) = engine.as_mut() {
            eng.as_qqmlengine()
                .on_quit(|_| println!("QML Quit!"))
                .release();
    }
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
