// build.rs
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "InstrumentUI",
            rust_files: &[
                "src/oscilloscope_backend.rs",
                "src/function_generator_backend.rs"
            ],
            qml_files: &[
                "qml/OscilloscopeUI.qml",
                "qml/FunctionGeneratorUI.qml"
            ],
            ..Default::default()
        })
        .build();
}
