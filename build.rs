// build.rs
use cxx_qt_build::QmlModule;
fn main() {
    cxx_qt_build::CxxQtBuilder::new()
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "com.kdab.cxx_qt.demo",
            rust_files: &[
                "src/oscilloscope_backend.rs",
                "src/function_generator_backend.rs"
            ],
            qml_files: &[
                "qml/oscilloscope.qml",
                "qml/function_generator.qml"
            ],
            ..Default::default()
        })
        .build();
}
