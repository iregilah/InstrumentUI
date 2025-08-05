// src/function_generator_backend.rs
#[cxx_qt::bridge]
mod ffi {
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
        fn awg1_enable_changed(&self, on: bool);
        #[qinvokable]
        fn awg1_waveform_selected(&self, wave: &QString);
        #[qinvokable]
        fn awg1_freq_changed(&self, freq: i32);
        #[qinvokable]
        fn awg1_amp_changed(&self, ampl: f64);
        #[qinvokable]
        fn awg1_offset_changed(&self, offset: f64);
        #[qinvokable]
        fn awg1_load_arb(&self, file_path: &QString);

        #[qinvokable]
        fn awg2_enable_changed(&self, on: bool);
        #[qinvokable]
        fn awg2_waveform_selected(&self, wave: &QString);
        #[qinvokable]
        fn awg2_freq_changed(&self, freq: i32);
        #[qinvokable]
        fn awg2_amp_changed(&self, ampl: f64);
        #[qinvokable]
        fn awg2_offset_changed(&self, offset: f64);
        #[qinvokable]
        fn awg2_load_arb(&self, file_path: &QString);
    }
}
use std::net::{SocketAddr, TcpStream};
use std::io::Write;
use std::time::Duration;
use std::thread;
use tokio::runtime::Runtime;
use cxx_qt_lib::QString;
use rigol_cli;
pub struct FunctionGeneratorBackendRust {
    address: SocketAddr,
}
impl Default for FunctionGeneratorBackendRust {
    fn default() -> Self {
        FunctionGeneratorBackendRust {
            address: "169.254.50.23:5555".parse().unwrap()
        }
    }
}
impl ffi::FunctionGeneratorBackend {
    fn awg1_enable_changed(&self, on: bool) {
        send_scpi(&self.rust().address, &format!(":OUTPUT1 {}", if on { "ON" } else { "OFF" }));
    }
    fn awg1_waveform_selected(&self, wave: &QString) {
        let wf = wave.to_string().to_ascii_uppercase();
        let scpi_type = match wf.as_str() {
            "SINE" => "SIN",
            "SQUARE" => "SQU",
            "PULSE" => "PULS",
            "RAMP" => "RAMP",
            "NOISE" => "NOIS",
            "ARB" => "USER",
            _ => wf.as_str()
        };
        send_scpi(&self.rust().address, &format!(":SOUR1:FUNC {}", scpi_type));
    }
    fn awg1_freq_changed(&self, freq: i32) {
        send_scpi(&self.rust().address, &format!(":SOUR1:FREQ {}", freq));
    }
    fn awg1_amp_changed(&self, ampl: f64) {
        send_scpi(&self.rust().address, &format!(":SOUR1:VOLT {}", ampl));
    }
    fn awg1_offset_changed(&self, offset: f64) {
        send_scpi(&self.rust().address, &format!(":SOUR1:VOLT:OFFS {}", offset));
    }
    fn awg1_load_arb(&self, file_path: &QString) {
        let addr = self.rust().address;
        let file = file_path.to_string();
        thread::spawn(move || {
            let rt = Runtime::new().expect("Tokio runtime init");
            if let Err(e) = rt.block_on(rigol_cli::io::load_arb(&addr, 1, &file)) {
                eprintln!("Arb file load failed: {}", e);
            } else {
                send_scpi(&addr, ":SOUR1:FUNC USER");
            }
        });
    }
    fn awg2_enable_changed(&self, on: bool) {
        send_scpi(&self.rust().address, &format!(":OUTPUT2 {}", if on { "ON" } else { "OFF" }));
    }
    fn awg2_waveform_selected(&self, wave: &QString) {
        let wf = wave.to_string().to_ascii_uppercase();
        let scpi_type = match wf.as_str() {
            "SINE" => "SIN",
            "SQUARE" => "SQU",
            "PULSE" => "PULS",
            "RAMP" => "RAMP",
            "NOISE" => "NOIS",
            "ARB" => "USER",
            _ => wf.as_str()
        };
        send_scpi(&self.rust().address, &format!(":SOUR2:FUNC {}", scpi_type));
    }
    fn awg2_freq_changed(&self, freq: i32) {
        send_scpi(&self.rust().address, &format!(":SOUR2:FREQ {}", freq));
    }
    fn awg2_amp_changed(&self, ampl: f64) {
        send_scpi(&self.rust().address, &format!(":SOUR2:VOLT {}", ampl));
    }
    fn awg2_offset_changed(&self, offset: f64) {
        send_scpi(&self.rust().address, &format!(":SOUR2:VOLT:OFFS {}", offset));
    }
    fn awg2_load_arb(&self, file_path: &QString) {
        let addr = self.rust().address;
        let file = file_path.to_string();
        thread::spawn(move || {
            let rt = Runtime::new().expect("Tokio runtime init");
            if let Err(e) = rt.block_on(rigol_cli::io::load_arb(&addr, 2, &file)) {
                eprintln!("Arb file load failed: {}", e);
            } else {
                send_scpi(&addr, ":SOUR2:FUNC USER");
            }
        });
    }
}
fn send_scpi(addr: &SocketAddr, cmd: &str) {
    if let Ok(mut stream) = TcpStream::connect_timeout(addr, Duration::from_millis(500)) {
        let _ = stream.write_all(format!("{}\n", cmd).as_bytes());
        let _ = stream.flush();
    }
}
