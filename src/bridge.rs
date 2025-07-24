use std::thread;
use std::time::Duration;
use std::net::TcpStream;
use std::io::Write;
// src/bridge.rs
use core::pin::Pin;
use std::thread;

#[cxx_qt::bridge]
mod ffi {
    extern "RustQt" {
        // ---------- QObject definíció ----------
        #[qobject]          // QObject generálása
        #[qml_element]      // QML‑ből közvetlenül létrehozható
        type DeviceController = super::DeviceControllerRust;

        // ---------- QML‑ből hívható függvény ----
        #[qinvokable]
        fn run_demo(self: Pin<&mut Self>);
    }
}

// ---------- Rust‑oldali implementáció ----------
#[derive(Default)]
pub struct DeviceControllerRust;   // nincs mező – csak tesztelünk

impl DeviceControllerRust {
    /// Q_INVOKABLE → QML‑ből: `deviceController.runDemo()`
    pub fn run_demo(self: Pin<&mut Self>) {
        // Ne blokkoljuk a GUI‑t
        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new()
                .expect("Tokio runtime létrehozása sikertelen");
            if let Err(e) = rt.block_on(async {
                // A backend demó futtatása
                if let Err(err) = rigol_cli::examples::basic_demo::main().await {
                    eprintln!("Demo hiba: {err}");
                }
                Ok::<(), Box<dyn std::error::Error>>(())
            }) {
                eprintln!("Tokio hiba: {e}");
            }
        });
    }
}
