// build.rs
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network") // Link Qt Network (needed for Qt QML on some platforms)
        .qml_module(QmlModule {
            uri: "com.instrument.ui",
            version_major: 1,
            version_minor: 0,
            rust_files: &["src/oscilloscope_ui.rs", "src/function_generator_ui.rs"],
            qml_files: &["qml/oscilloscope.qml", "qml/function_generator.qml"],
            ..Default::default()
        })
        .build();
}
