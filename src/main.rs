use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use std::fs;

fn main() {
    // Set style from configuration or default to Material
    if let Ok(style) = fs::read_to_string("style.conf") {
        let style = style.trim();
        if !style.is_empty() {
            unsafe {
                std::env::set_var("QT_QUICK_CONTROLS_STYLE", style);
            }
            println!("Style from config: {}", style);
        }
    } else {
        unsafe {
            std::env::set_var("QT_QUICK_CONTROLS_STYLE", "Material");
        }
        println!("Default style 'Material' applied");
    }

    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/InstrumentUI/qml/main.qml"));
        println!("Main UI loaded");
        engine.load(&QUrl::from("qrc:/qt/qml/InstrumentUI/qml/awg.qml"));
        println!("Function Generator UI loaded");
    }
    if let Some(engine) = engine.as_mut() {
        engine
            .as_qqmlengine()
            .on_quit(|_| println!("Application exited"))
            .release();
    }
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
