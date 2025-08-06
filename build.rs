// build.rs
use cxx_qt_build::CxxQtBuilder;
use cxx_qt_build::QmlModule;

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "com.rigol.scope",
            rust_files: &["src/osc_controller.rs", "src/awg_controller.rs"],
            qml_files: &["src/qml/oscilloscope.qml", "src/qml/function_generator.qml"],
            ..Default::default()
        })
        .build();
}
