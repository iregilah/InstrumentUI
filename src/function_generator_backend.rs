// src/function_generator_backend.rs

#[cxx_qt::bridge]
pub mod function_generator {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type FunctionGeneratorBackend = super::FunctionGeneratorBackendRust;
    }
    extern "RustQt" {
        #[qinvokable]
        #[cxx_name = "awg1EnableChanged"]
        fn awg1_enable_changed(&self, on: bool);
        #[qinvokable]
        #[cxx_name = "awg2EnableChanged"]
        fn awg2_enable_changed(&self, on: bool);

        #[qinvokable]
        #[cxx_name = "awg1WaveformSelected"]
        fn awg1_waveform_selected(&self, wave: &QString);
        #[qinvokable]
        #[cxx_name = "awg2WaveformSelected"]
        fn awg2_waveform_selected(&self, wave: &QString);

        #[qinvokable]
        #[cxx_name = "awg1FreqChanged"]
        fn awg1_freq_changed(&self, freq: i32);
        #[qinvokable]
        #[cxx_name = "awg2FreqChanged"]
        fn awg2_freq_changed(&self, freq: i32);

        #[qinvokable]
        #[cxx_name = "awg1AmpChanged"]
        fn awg1_amp_changed(&self, ampl: f64);
        #[qinvokable]
        #[cxx_name = "awg2AmpChanged"]
        fn awg2_amp_changed(&self, ampl: f64);

        #[qinvokable]
        #[cxx_name = "awg1OffsetChanged"]
        fn awg1_offset_changed(&self, offs: f64);
        #[qinvokable]
        #[cxx_name = "awg2OffsetChanged"]
        fn awg2_offset_changed(&self, offs: f64);

        #[qinvokable]
        #[cxx_name = "awg1LoadArb"]
        fn awg1_load_arb(&self, file_path: &QString);
        #[qinvokable]
        #[cxx_name = "awg2LoadArb"]
        fn awg2_load_arb(&self, file_path: &QString);
    }
}

use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
use rigol_cli::io;
use crate::ADDR;

#[derive(Default)]
pub struct FunctionGeneratorBackendRust;

impl qobject::FunctionGeneratorBackend {
    pub fn awg1_enable_changed(&self, on: bool) {
        send_scpi(&format!(":OUTPUT1 {}", if on { "ON" } else { "OFF" }));
    }
    pub fn awg2_enable_changed(&self, on: bool) {
        send_scpi(&format!(":OUTPUT2 {}", if on { "ON" } else { "OFF" }));
    }
    pub fn awg1_waveform_selected(&self, wave: &cxx_qt_lib::QString) {
        let wf = wave.to_string().to_ascii_uppercase();
        let scpi_type = match wf.as_str() {
            "SINE" => "SIN",
            "SQUARE" => "SQU",
            "PULSE" => "PULS",
            "RAMP" => "RAMP",
            "NOISE" => "NOIS",
            "ARB" => "USER",
            _ => wf.as_str(),
        };
        send_scpi(&format!(":SOUR1:FUNC {}", scpi_type));
    }
    pub fn awg2_waveform_selected(&self, wave: &cxx_qt_lib::QString) {
        let wf = wave.to_string().to_ascii_uppercase();
        let scpi_type = match wf.as_str() {
            "SINE" => "SIN",
            "SQUARE" => "SQU",
            "PULSE" => "PULS",
            "RAMP" => "RAMP",
            "NOISE" => "NOIS",
            "ARB" => "USER",
            _ => wf.as_str(),
        };
        send_scpi(&format!(":SOUR2:FUNC {}", scpi_type));
    }
    pub fn awg1_freq_changed(&self, freq: i32) {
        send_scpi(&format!(":SOUR1:FREQ {}", freq));
    }
    pub fn awg2_freq_changed(&self, freq: i32) {
        send_scpi(&format!(":SOUR2:FREQ {}", freq));
    }
    pub fn awg1_amp_changed(&self, ampl: f64) {
        send_scpi(&format!(":SOUR1:VOLT {}", ampl));
    }
    pub fn awg2_amp_changed(&self, ampl: f64) {
        send_scpi(&format!(":SOUR2:VOLT {}", ampl));
    }
    pub fn awg1_offset_changed(&self, offs: f64) {
        send_scpi(&format!(":SOUR1:VOLT:OFFS {}", offs));
    }
    pub fn awg2_offset_changed(&self, offs: f64) {
        send_scpi(&format!(":SOUR2:VOLT:OFFS {}", offs));
    }
    pub fn awg1_load_arb(&self, file_path: &cxx_qt_lib::QString) {
        let path = file_path.to_string();
        let addr_opt = *crate::ADDR.lock().unwrap();
        if let Some(addr) = addr_opt {
            thread::spawn(move || {
                let rt = Runtime::new().expect("Tokio runtime init");
                if let Err(e) = rt.block_on(io::load_arb(&addr, 1, &path)) {
                    eprintln!("Arb file load failed: {}", e);
                } else {
                    send_scpi(":SOUR1:FUNC USER");
                }
            });
        }
    }
    pub fn awg2_load_arb(&self, file_path: &cxx_qt_lib::QString) {
        let path = file_path.to_string();
        let addr_opt = *crate::ADDR.lock().unwrap();
        if let Some(addr) = addr_opt {
            thread::spawn(move || {
                let rt = Runtime::new().expect("Tokio runtime init");
                if let Err(e) = rt.block_on(io::load_arb(&addr, 2, &path)) {
                    eprintln!("Arb file load failed: {}", e);
                } else {
                    send_scpi(":SOUR2:FUNC USER");
                }
            });
        }
    }
}

fn send_scpi(cmd: &str) {
    if let Some(addr) = *crate::ADDR.lock().unwrap() {
        if let Ok(mut sock) = std::net::TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
            let _ = sock.write_all(format!("{}\n", cmd).as_bytes());
            let _ = sock.flush();
        }
    }
}
