// build.rs
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")  // ensure Qt QML has network support (for Qt6 on macOS)
        .qml_module(QmlModule {
            uri: "com.rust.instrument",
            rust_files: &["src/gui.rs"],
            qml_files: &["qml/main.qml", "qml/function_generator.qml"],
            ..Default::default()
        })
        .build();
}