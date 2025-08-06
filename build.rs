// build.rs
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "InstrumentUI",
            rust_files: &["src/oscillo_object.rs", "src/awg_object.rs"],
            qml_files: &["qml/main.qml", "qml/awg.qml"],
            ..Default::default()
        })
        .build();
        // ide eddig nem írt semmit a kód
            /* ----------------------------------------------------
     *  Windows (MSVC) alatt tegyük be a két statikus Qt‑
     *  plugin‑könyvtárat, hogy a  qt_static_plugin_…  szimbólum
     *  feloldódjon.  A CxxQtBuilder már felvette az OUT_DIR‑t
     *  a linker keresési útvonalai közé, ezért elég a név.
     * ---------------------------------------------------- */
            #[cfg(target_env = "msvc")]
        {
            // a generált C++ ragasztókód
            println!("cargo:rustc-link-lib=static=Instrument_UI-cxxqt-generated");
        }
}