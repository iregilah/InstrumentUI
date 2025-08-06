// src/function_generator_ui.rs
#[cxx_qt::bridge(cxx_file_stem = "function_generator_ui")]
mod ffi {
    use std::net::SocketAddr;
    use std::io::Write;
    use std::sync::Mutex;
    use tokio::runtime::Runtime;
    use rigol_cli::{lxi::send_scpi, io};
    use std::thread;
    use super::ffi::*;

    extern "C++" {
        include!("cxx-qt-lib/qtcore_qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject]
    pub struct FunctionGeneratorUI {
        address: SocketAddr,
        // we can store any state if needed (not much, maybe not required)
    }

    impl Default for FunctionGeneratorUI {
        fn default() -> Self {
            // use same default address as OscilloscopeUI
            let addr: SocketAddr = "169.254.50.23:5555".parse().unwrap();
            Self { address: addr }
        }
    }

    impl qobject::FunctionGeneratorUI {
        #[qinvokable]
        fn awg1_enable_changed(&self, on: bool) {
            let cmd = format!(":OUTPUT1 {}", if on { "ON" } else { "OFF" });
            let _ = send_scpi(&self.address, &cmd);
        }
        #[qinvokable]
        fn awg2_enable_changed(&self, on: bool) {
            let cmd = format!(":OUTPUT2 {}", if on { "ON" } else { "OFF" });
            let _ = send_scpi(&self.address, &cmd);
        }
        #[qinvokable]
        fn awg1_waveform_selected(&self, wave: &QString) {
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
            let _ = send_scpi(&self.address, &cmd);
        }
        #[qinvokable]
        fn awg2_waveform_selected(&self, wave: &QString) {
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
            let _ = send_scpi(&self.address, &cmd);
        }
        #[qinvokable]
        fn awg1_freq_changed(&self, freq: i32) {
            let _ = send_scpi(&self.address, &format!(":SOUR1:FREQ {}", freq));
        }
        #[qinvokable]
        fn awg2_freq_changed(&self, freq: i32) {
            let _ = send_scpi(&self.address, &format!(":SOUR2:FREQ {}", freq));
        }
        #[qinvokable]
        fn awg1_amp_changed(&self, ampl: f64) {
            let _ = send_scpi(&self.address, &format!(":SOUR1:VOLT {}", ampl));
        }
        #[qinvokable]
        fn awg2_amp_changed(&self, ampl: f64) {
            let _ = send_scpi(&self.address, &format!(":SOUR2:VOLT {}", ampl));
        }
        #[qinvokable]
        fn awg1_offset_changed(&self, offs: f64) {
            let _ = send_scpi(&self.address, &format!(":SOUR1:VOLT:OFFS {}", offs));
        }
        #[qinvokable]
        fn awg2_offset_changed(&self, offs: f64) {
            let _ = send_scpi(&self.address, &format!(":SOUR2:VOLT:OFFS {}", offs));
        }
        #[qinvokable]
        fn awg1_load_arb(&self, file_path: &QString) {
            let path = file_path.to_string();
            let addr = self.address;
            thread::spawn(move || {
                let rt = Runtime::new().expect("Tokio runtime init");
                if let Err(e) = rt.block_on(io::load_arb(&addr, 1, &path)) {
                    eprintln!("Arb file load failed: {}", e);
                } else {
                    let _ = send_scpi(&addr, ":SOUR1:FUNC USER");
                }
            });
        }
        #[qinvokable]
        fn awg2_load_arb(&self, file_path: &QString) {
            let path = file_path.to_string();
            let addr = self.address;
            thread::spawn(move || {
                let rt = Runtime::new().expect("Tokio runtime init");
                if let Err(e) = rt.block_on(io::load_arb(&addr, 2, &path)) {
                    eprintln!("Arb file load failed: {}", e);
                } else {
                    let _ = send_scpi(&addr, ":SOUR2:FUNC USER");
                }
            });
        }
    }
}
