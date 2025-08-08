// build.rs

use cxx_qt_build::CxxQtBuilder;
use cxx_qt_build::QmlModule;

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")
        .qt_module("QuickControls2")
        .qml_module(QmlModule {
            uri: "InstrumentUI",
            rust_files: &["src/oscillo_object.rs", "src/awg_object.rs"],
            qml_files: &["qml/main.qml", "qml/awg.qml"],
            ..Default::default()
        })
        .build();
    #[cfg(target_env = "msvc")]
    {
        println!("cargo:rustc-link-arg=/WHOLEARCHIVE:Instrument_UI-cxxqt-generated.lib");
    }
}