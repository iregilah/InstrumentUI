// src/main.rs
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
fn main() {
    // Create Qt application and QML engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
    // Load QML UI files for both windows
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/cxx_qt/demo/qml/oscilloscope.qml"));
        engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/cxx_qt/demo/qml/function_generator.qml"));
    }
    // Connect quit signal (when all windows closed) to exit application
    if let Some(engine) = engine.as_mut() {
        engine
            .as_qqmlengine()
            .on_quit(|app_ptr| {
                if let Some(app) = unsafe { (app_ptr as *mut QGuiApplication).as_mut() } {
                    app.quit();
                }
            })
            .release();
    }
    // Execute the application event loop
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
