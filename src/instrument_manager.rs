// src/instrument_manager.rs
use cxx_qt::Threading;
use serde_json::json;
use crate::aggregator::{Aggregator, InstrumentInfo};
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use std::pin::Pin;

#[cxx_qt::bridge]
pub mod instrument_manager_qobject {
    // Qt/C++ types go here
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    // QObject and API
    #[auto_cxx_name] // optional: snake_case → camelCase for C++/QML
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, instrument_list)]
        type InstrumentManager = super::InstrumentManagerRust;

        #[qinvokable]
        fn scan(self: Pin<&mut InstrumentManager>); // <-- no ::std::pin::
    }

    impl cxx_qt::Threading for InstrumentManager {}
}

pub struct InstrumentManagerRust {
    aggregator: Aggregator,
    instrument_list: QString,
}

impl Default for InstrumentManagerRust {
    fn default() -> Self {
        let aggregator = Aggregator::new().unwrap_or_else(|e| {
            eprintln!("Failed to initialize Aggregator: {}", e);
            // Fallback to default config if initialization fails
            Aggregator::new().unwrap()
        });
        Self {
            aggregator,
            instrument_list: QString::from("[]"),
        }
    }
}

impl instrument_manager_qobject::InstrumentManager {
    pub fn scan(self: Pin<&mut Self>) {
        let mut this = self;
        println!("[UI] Scanning for instruments...");
        let agg = &mut unsafe { this.as_mut().rust_mut().aggregator };
        // Reset instrument list before scanning
        agg.connected_instruments.clear();
        agg.next_uuid = 0;
        // Perform discovery on all interfaces
        let devices = agg.discover_all();
        // Build JSON array of instruments
        let mut list_json = vec![];
        for (_uuid, info) in devices.iter() {
            // Construct display name
            let name = if let Some(model) = &info.model {
                if let Some(vendor) = &info.vendor {
                    format!("{} {}", vendor, model)
                } else {
                    model.clone()
                }
            } else if let Some(vendor) = &info.vendor {
                vendor.clone()
            } else if let Some(instr_type) = &info.instrument_type {
                instr_type.clone()
            } else {
                info.identifier.clone()
            };
            // Determine communication channel badge text
            let comm = if info.interface.contains("USB") {
                "USB"
            } else if info.interface.contains("GPIB") {
                "GPIB"
            } else if info.interface.contains("LXI") || info.interface.contains("TCP") {
                "LAN"
            } else if info.interface.contains("RS-485") || info.interface.contains("Serial") {
                "RS485"
            } else {
                info.interface.as_str()
            };
            list_json.push(json!({ "name": name, "comm": comm }));
        }
        let json_str = serde_json::to_string(&list_json).unwrap_or_else(|_| "[]".to_string());
        this.as_mut().set_instrument_list(QString::from(json_str));
        println!("[UI] Scan complete, found {} instrument(s)", list_json.len());
    }
}
