// build.rs – FRISSÍTVE
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")      // opcionális, ha kell a QtNetwork
        .file("src/bridge.rs")     // a #[cxx_qt::bridge] modul
        .qrc("qml/qml.qrc")        // QML erőforrások
        .build();                  // <-- csak ez kell, setup_linker NEM
}
/*
// build.rs  –  teljes, működő változat
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")                // ha kell QtNetwork
        // ----- QML‑MODUL DEFINÍCIÓ ---------------------------------
        .qml_module(QmlModule {
            uri: "RigolDemo",                // <-- ugyanaz, mint a QML import
            rust_files: &["src/bridge.rs"],  // bridge.rs-ben lévő #[qobject]
            qml_files: &["qml/main.qml"],    // (opcionális) felsorolás
            // version_major = 1, version_minor = 0  -> implicit 1.0
            ..Default::default()
        })
        // ----- további források / erőforrások ----------------------
        .file("src/bridge.rs")               // maga a bridge‑modul
        .qrc("qml/qml.qrc")                  // RCC erőforráshoz
        .build();                            // ennyi elég
}*/