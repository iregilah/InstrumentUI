// src/awg_object.rs
#[cxx_qt::bridge]
pub mod awg_qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, currentWaveCh1)]
        #[qproperty(QString, currentWaveCh2)]
        type AwgObject = super::AwgObjectRust;
    }

    extern "RustQt" {
        #[qinvokable]
        fn awg1_enable_changed(self: &AwgObject, on: bool);
        #[qinvokable]
        fn awg1_waveform_selected(self: Pin<&mut AwgObject>, wave: &QString);
        #[qinvokable]
        fn awg1_freq_changed(self: &AwgObject, freq: i32);
        #[qinvokable]
        fn awg1_amp_changed(self: &AwgObject, ampl: f64);
        #[qinvokable]
        fn awg1_offset_changed(self: &AwgObject, offset: f64);
        #[qinvokable]
        fn awg1_load_arb(self: &AwgObject, file_path: &QString);

        #[qinvokable]
        fn awg2_enable_changed(self: &AwgObject, on: bool);
        #[qinvokable]
        fn awg2_waveform_selected(self: Pin<&mut AwgObject>, wave: &QString);
        #[qinvokable]
        fn awg2_freq_changed(self: &AwgObject, freq: i32);
        #[qinvokable]
        fn awg2_amp_changed(self: &AwgObject, ampl: f64);
        #[qinvokable]
        fn awg2_offset_changed(self: &AwgObject, offset: f64);
        #[qinvokable]
        fn awg2_load_arb(self: &AwgObject, file_path: &QString);
    }
}

use core::pin::Pin;
use std::net::SocketAddr;
use std::io::Write;
use std::thread;
use tokio::runtime::Runtime;
use rigol_cli::io;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct AwgObjectRust {
    addr: SocketAddr,
    current_wave_ch1: cxx_qt_lib::QString,
    current_wave_ch2: cxx_qt_lib::QString,
}

impl qobject::AwgObject {
    pub fn on_construct(&mut self) {
        // Parse address similar to OscilloObject
        let args: Vec<String> = std::env::args().collect();
        let addr_str = match args.get(1) {
            Some(a) if !a.starts_with('-') => {
                if a.contains(':') { a.clone() } else { format!("{}:5555", a) }
            }
            _ => "169.254.50.23:5555".to_string(),
        };
        self.addr = addr_str.parse().unwrap_or_else(|_| "169.254.50.23:5555".parse().unwrap());
        // Default waveform types
        self.set_current_wave_ch1(QString::from("Sine"));
        self.set_current_wave_ch2(QString::from("Sine"));
    }

    pub fn awg1_enable_changed(&self, on: bool) {
        self.send_scpi(&format!(":OUTPUT1 {}", if on { "ON" } else { "OFF" }));
    }
    pub fn awg2_enable_changed(&self, on: bool) {
        self.send_scpi(&format!(":OUTPUT2 {}", if on { "ON" } else { "OFF" }));
    }

    pub fn awg1_waveform_selected(self: Pin<&mut Self>, wave: &cxx_qt_lib::QString) {
        let wf = wave.to_string().to_ascii_uppercase();
        let scpi_type = match wf.as_str() {
            "SINE" => "SIN",
            "SQUARE" => "SQU",
            "PULSE" => "PULS",
            "RAMP" => "RAMP",
            "NOISE" => "NOIS",
            "ARB" => "USER",
            other => other,
        };
        // update property
        self.set_current_wave_ch1(cxx_qt_lib::QString::from(wave));
        // send SCPI
        self.send_scpi(&format!(":SOUR1:FUNC {}", scpi_type));
    }
    pub fn awg2_waveform_selected(self: Pin<&mut Self>, wave: &cxx_qt_lib::QString) {
        let wf = wave.to_string().to_ascii_uppercase();
        let scpi_type = match wf.as_str() {
            "SINE" => "SIN",
            "SQUARE" => "SQU",
            "PULSE" => "PULS",
            "RAMP" => "RAMP",
            "NOISE" => "NOIS",
            "ARB" => "USER",
            other => other,
        };
        self.set_current_wave_ch2(cxx_qt_lib::QString::from(wave));
        self.send_scpi(&format!(":SOUR2:FUNC {}", scpi_type));
    }

    pub fn awg1_freq_changed(&self, freq: i32) {
        self.send_scpi(&format!(":SOUR1:FREQ {}", freq));
    }
    pub fn awg2_freq_changed(&self, freq: i32) {
        self.send_scpi(&format!(":SOUR2:FREQ {}", freq));
    }
    pub fn awg1_amp_changed(&self, ampl: f64) {
        self.send_scpi(&format!(":SOUR1:VOLT {}", ampl));
    }
    pub fn awg2_amp_changed(&self, ampl: f64) {
        self.send_scpi(&format!(":SOUR2:VOLT {}", ampl));
    }
    pub fn awg1_offset_changed(&self, offset: f64) {
        self.send_scpi(&format!(":SOUR1:VOLT:OFFS {}", offset));
    }
    pub fn awg2_offset_changed(&self, offset: f64) {
        self.send_scpi(&format!(":SOUR2:VOLT:OFFS {}", offset));
    }

    pub fn awg1_load_arb(&self, file_path: &cxx_qt_lib::QString) {
        let file = file_path.to_string();
        let addr = self.rust().addr;
        thread::spawn(move || {
            let rt = Runtime::new().expect("Tokio runtime init");
            if let Err(e) = rt.block_on(io::load_arb(&addr, 1, &file)) {
                eprintln!("Arb file load failed: {}", e);
            } else {
                // After loading, switch to USER waveform
                let _ = io::send_scpi(&addr, ":SOUR1:FUNC USER");
            }
        });
    }
    pub fn awg2_load_arb(&self, file_path: &cxx_qt_lib::QString) {
        let file = file_path.to_string();
        let addr = self.rust().addr;
        thread::spawn(move || {
            let rt = Runtime::new().expect("Tokio runtime init");
            if let Err(e) = rt.block_on(io::load_arb(&addr, 2, &file)) {
                eprintln!("Arb file load failed: {}", e);
            } else {
                let _ = io::send_scpi(&addr, ":SOUR2:FUNC USER");
            }
        });
    }

    // Helper to send SCPI (synchronous)
    fn send_scpi(&self, cmd: &str) {
        if let Ok(mut s) = std::net::TcpStream::connect_timeout(&self.rust().addr, std::time::Duration::from_millis(500)) {
            let _ = s.write_all(format!("{}\n", cmd).as_bytes());
            let _ = s.flush();
        }
    }
}
