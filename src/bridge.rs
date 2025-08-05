// src/bridge.rs   (rövidített példa)

#[cxx_qt::bridge]
mod ffi {
    // --- Qt típusok bekötése, ha kell -------------------------
    // unsafe extern "C++" { ... }

    // --- QObject deklaráció + QML‑export ----------------------
    extern "RustQt" {
        #[qobject]                   // !!! kötelező
        #[qml_element]               // hogy QML‑ben is elérd
        type DeviceController = super::DeviceControllerRust;
    }

    // --- Invokable metódusok ----------------------------------
    extern "RustQt" {
        #[qinvokable]
        // hivatkozás *a konkrét típusra*:
        fn run_demo(self: Pin<&mut DeviceController>);
    }
}

// -------- Rust‑oldali implementáció ---------------------------
use core::pin::Pin;

#[derive(Default)]
pub struct DeviceControllerRust { /* mezők */ }

impl ffi::DeviceController {
    /// Példa invokable metódus
    pub fn run_demo(self: Pin<&mut Self>) {
        println!("Running demo from Rust!");
    }
}
