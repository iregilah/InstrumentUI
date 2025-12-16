// build.rs

use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    // (optional but recommended)
    println!("cargo:rerun-if-changed=cpp/graph_object_helpers.h");
    println!("cargo:rerun-if-changed=cpp/graph_object_helpers.cpp");

    CxxQtBuilder::new()
        .qt_module("Network")
        .qt_module("QuickControls2")
        .qt_module("Quick")
        .qml_module(QmlModule {
            uri: "InstrumentUI",
            rust_files: &[
                "src/oscillo_object.rs",
                "src/awg_object.rs",
                "src/instrument_manager.rs",
                "src/graph_object.rs",
            ],
            qml_files: &["qml/main.qml", "qml/awg.qml", "qml/hub.qml"],
            ..Default::default()
        })
        .cc_builder(|cc| {
            // Add include path for your manually written headers
            cc.include("cpp");

            // Compile your helper implementation so the symbols exist at link time
            cc.file("cpp/graph_object_helpers.cpp");
        })
        .build();

    #[cfg(target_env = "msvc")]
    {
        println!("cargo:rustc-link-arg=/WHOLEARCHIVE:Instrument_UI-cxxqt-generated.lib");
    }
}