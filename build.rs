// build.rs – FRISSÍTVE
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")      // opcionális, ha kell a QtNetwork
        .file("src/bridge.rs")     // a #[cxx_qt::bridge] modul
        .qrc("qml/qml.qrc")        // QML erőforrások
        .build();                  // <-- csak ez kell, setup_linker NEM
}