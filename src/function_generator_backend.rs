//! src/function_generator_backend.rs
use crate::ADDRESS;
use once_cell::sync::OnceCell;
use std::net::TcpStream;
use std::time::Duration;
use tokio::runtime::Runtime;

#[cxx_qt::bridge]
mod ffi {
    use super::*;
    use cxx_qt_lib::QString;

    extern "Rust" {
        static ADDRESS: OnceCell<std::net::SocketAddr>;
        fn send_scpi(cmd: &QString);
    }

    #[cxx_qt::qobject]
    struct FunctionGeneratorBackend {}

    impl qobject::FunctionGeneratorBackend {
        #[qinvokable]
        pub fn awg1_enable_changed(&self, on: bool) {
            let cmd = if on { ":OUTPUT1 ON" } else { ":OUTPUT1 OFF" };
            send_scpi(&QString::from(cmd));
        }
        #[qinvokable]
        pub fn awg2_enable_changed(&self, on: bool) {
            let cmd = if on { ":OUTPUT2 ON" } else { ":OUTPUT2 OFF" };
            send_scpi(&QString::from(cmd));
        }
        #[qinvokable]
        pub fn awg1_waveform_selected(&self, wave: &QString) {
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
            let cmd = format!(":SOUR1:FUNC {}", scpi_type);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable]
        pub fn awg2_waveform_selected(&self, wave: &QString) {
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
            let cmd = format!(":SOUR2:FUNC {}", scpi_type);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable]
        pub fn awg1_freq_changed(&self, freq: i32) {
            let cmd = format!(":SOUR1:FREQ {}", freq);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable]
        pub fn awg2_freq_changed(&self, freq: i32) {
            let cmd = format!(":SOUR2:FREQ {}", freq);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable]
        pub fn awg1_amp_changed(&self, ampl: f64) {
            let cmd = format!(":SOUR1:VOLT {}", ampl);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable]
        pub fn awg2_amp_changed(&self, ampl: f64) {
            let cmd = format!(":SOUR2:VOLT {}", ampl);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable]
        pub fn awg1_offset_changed(&self, offs: f64) {
            let cmd = format!(":SOUR1:VOLT:OFFS {}", offs);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable]
        pub fn awg2_offset_changed(&self, offs: f64) {
            let cmd = format!(":SOUR2:VOLT:OFFS {}", offs);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable]
        pub fn awg1_load_arb(&self, file_path: &QString) {
            let path = file_path.to_string();
            if path.is_empty() {
                return;
            }
            // Load arbitrary waveform file on a background thread (CH1)
            std::thread::spawn(move || {
                if let Some(addr) = ADDRESS.get() {
                    let rt = Runtime::new().expect("Tokio runtime init");
                    if let Err(e) = rt.block_on(rigol_cli::io::load_arb(addr, 1, &path)) {
                        eprintln!("Arb file load failed: {e}");
                    } else {
                        send_scpi(&QString::from(":SOUR1:FUNC USER"));
                    }
                }
            });
        }
        #[qinvokable]
        pub fn awg2_load_arb(&self, file_path: &QString) {
            let path = file_path.to_string();
            if path.is_empty() {
                return;
            }
            // Load arbitrary waveform file on a background thread (CH2)
            std::thread::spawn(move || {
                if let Some(addr) = ADDRESS.get() {
                    let rt = Runtime::new().expect("Tokio runtime init");
                    if let Err(e) = rt.block_on(rigol_cli::io::load_arb(addr, 2, &path)) {
                        eprintln!("Arb file load failed: {e}");
                    } else {
                        send_scpi(&QString::from(":SOUR2:FUNC USER"));
                    }
                }
            });
        }
    }
}

// Reuse send_scpi implementation from OscilloscopeBackend
extern "Rust" fn send_scpi(cmd: &QString) {
    if let Some(addr) = ADDRESS.get() {
        if let Ok(mut stream) = TcpStream::connect_timeout(addr, Duration::from_millis(500)) {
            let _ = stream.write_all(cmd.to_string().as_bytes());
            let _ = stream.write_all(b"\n");
            let _ = stream.flush();
        }
    }
}
