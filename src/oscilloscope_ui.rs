// src/oscilloscope_ui.rs
#[cxx_qt::bridge(cxx_file_stem = "oscilloscope_ui")]
mod ffi {
    use super::ffi::*;
    use std::net::{SocketAddr, TcpStream};
    use std::io::{Read, Write};
    use std::sync::{Mutex, atomic::{AtomicBool, Ordering}};
    use std::time::Duration;
    use std::thread;
    use rigol_cli::{utils::parse_source_arg, lxi, io};
    use tokio::runtime::Runtime;

    extern "C++" {
        include!("cxx-qt-lib/qtcore_qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject]
    pub struct OscilloscopeUI {
        #[qproperty]
        avg_enabled: bool,
        // Internal fields (not exposed to QML)
        address: SocketAddr,
        trigger_source: Mutex<String>,
        running: AtomicBool,
        current_timebase: Mutex<f64>,
    }

    impl Default for OscilloscopeUI {
        fn default() -> Self {
            // Default to instrument at 169.254.50.23:5555 if not overridden
            let addr: SocketAddr = "169.254.50.23:5555".parse().unwrap();
            Self {
                avg_enabled: false,
                address: addr,
                trigger_source: Mutex::new(String::from("CHANnel1")),
                running: AtomicBool::new(true),
                current_timebase: Mutex::new(0.5_f64),
            }
        }
    }

    impl qobject::OscilloscopeUI {
        // Helper functions to send/query SCPI over TCP (blocking)
        fn send_scpi(&self, cmd: &str) {
            if let Ok(mut s) = TcpStream::connect_timeout(&self.address, Duration::from_millis(500)) {
                let _ = s.write_all(format!("{}\n", cmd).as_bytes());
                let _ = s.flush();
            }
        }
        fn query_scpi(&self, cmd: &str) -> Option<String> {
            if let Ok(mut s) = TcpStream::connect_timeout(&self.address, Duration::from_millis(500)) {
                let _ = s.write_all(format!("{}\n", cmd).as_bytes());
                let _ = s.flush();
                let mut buf = [0u8; 512];
                if let Ok(n) = s.read(&mut buf) {
                    if n > 0 {
                        return Some(String::from_utf8_lossy(&buf[..n])
                            .trim_end_matches(&['\r', '\n'][..])
                            .to_string());
                    }
                }
            }
            None
        }

        #[qinvokable]
        fn initialize(&self) {
            // Initial instrument setup
            self.send_scpi(":CHAN1:DISP ON");
            self.send_scpi(":OUTPUT1 OFF");
            self.send_scpi(":OUTPUT2 OFF");
            // Start background thread to periodically fetch oscilloscope display image
            let addr = self.address;
            thread::spawn({
                let qobj = self.qt_thread();  // get QPointer to safely use later
                move || {
                    loop {
                        if let Ok(mut s) = TcpStream::connect(addr) {
                            let _ = s.write_all(b":DISP:DATA?\n");
                            let _ = s.flush();
                            // Read binary block header
                            let mut hdr = [0u8; 2];
                            if s.read_exact(&mut hdr).is_err() || hdr[0] != b'#' {
                                thread::sleep(Duration::from_millis(200));
                                continue;
                            }
                            let ndigits = (hdr[1] as char).to_digit(10).unwrap_or(0) as usize;
                            let mut len_buf = vec![0u8; ndigits];
                            if s.read_exact(&mut len_buf).is_err() {
                                thread::sleep(Duration::from_millis(200));
                                continue;
                            }
                            let data_len = String::from_utf8_lossy(&len_buf).parse::<usize>().unwrap_or(0);
                            let mut img_data = vec![0u8; data_len];
                            if s.read_exact(&mut img_data).is_err() {
                                thread::sleep(Duration::from_millis(200));
                                continue;
                            }
                            // Write image bytes to file
                            if let Ok(mut file) = std::fs::File::create("screenshot.png") {
                                let _ = file.write_all(&img_data);
                            }
                        }
                        thread::sleep(Duration::from_millis(200));
                        // (The QML UI will load the updated image periodically)
                    }
                }
            });
        }

        // Toolbar button callbacks
        #[qinvokable]
        fn info_clicked(&self) {
            println!("ℹ Info");
        }
        #[qinvokable]
        fn settings_clicked(&self) {
            println!("⚙ Settings");
        }
        #[qinvokable]
        fn console_clicked(&self) {
            println!(">_ Console");
        }
        #[qinvokable]
        fn save_config_clicked(&self) {
            println!("💾 Save config (todo)");
        }
        #[qinvokable]
        fn load_config_clicked(&self) {
            println!("↑ Load config (todo)");
        }
        #[qinvokable]
        fn autoscale_clicked(&self) {
            self.send_scpi(":AUToscale");
        }

        // Trigger controls
        #[qinvokable]
        fn trigger_source_selected(&self, source: &QString) {
            if let Ok(parsed) = parse_source_arg(&source.to_string()) {
                *self.rust().trigger_source.lock().unwrap() = parsed.clone();
                self.send_scpi(":TRIG:MODE EDGE");
                self.send_scpi(&format!(":TRIG:EDGE:SOUR {}", parsed));
            }
        }
        #[qinvokable]
        fn trigger_level_changed(&self, level: i32) {
            self.send_scpi(&format!(":TRIG:EDGE:LEV {}", level));
        }
        #[qinvokable]
        fn trigger_slope_up(&self) {
            self.send_scpi(":TRIG:EDGE:SLOP POS");
        }
        #[qinvokable]
        fn trigger_slope_down(&self) {
            self.send_scpi(":TRIG:EDGE:SLOP NEG");
        }
        #[qinvokable]
        fn single_trigger_clicked(&self) {
            self.send_scpi(":SING");
        }
        #[qinvokable]
        fn run_stop_clicked(&self) {
            if self.rust().running.load(Ordering::SeqCst) {
                self.send_scpi(":STOP");
                self.rust().running.store(false, Ordering::SeqCst);
            } else {
                self.send_scpi(":RUN");
                self.rust().running.store(true, Ordering::SeqCst);
            }
        }

        // Horizontal controls (timebase)
        #[qinvokable]
        fn timebase_changed(&self, val: f64) {
            let scale = val / 100.0;  // map slider (1-100) to 0.01-1.00 s/div
            *self.rust().current_timebase.lock().unwrap() = scale;
            self.send_scpi(&format!(":TIM:SCAL {}", scale));
        }
        #[qinvokable]
        fn time_offset_changed(&self, val: f64) {
            let base = *self.rust().current_timebase.lock().unwrap();
            let offs = base * (val / 50.0);  // ±2 screen widths (slider -100..100)
            self.send_scpi(&format!(":TIM:OFFS {}", offs));
        }

        // Averaging toggle (applies to all channels)
        #[qinvokable]
        fn average_toggled(&self, on: bool) {
            if on {
                self.send_scpi(":ACQ:TYPE AVER");
                self.send_scpi(":ACQ:AVER 16");
            } else {
                self.send_scpi(":ACQ:TYPE NORM");
            }
            self.set_avg_enabled(on);
        }

        // Channel controls (4 channels)
        #[qinvokable]
        fn ch1_enable_changed(&self, on: bool) {
            self.send_scpi(if on { ":CHAN1:DISP ON" } else { ":CHAN1:DISP OFF" });
        }
        #[qinvokable]
        fn ch1_scale_changed(&self, val: f64) {
            self.send_scpi(&format!(":CHAN1:SCAL {}", val));
        }
        #[qinvokable]
        fn ch1_offset_changed(&self, val: f64) {
            self.send_scpi(&format!(":CHAN1:OFFS {}", val));
        }
        #[qinvokable]
        fn ch1_coupling_selected(&self, mode: &QString) {
            let mode_str = mode.to_string();
            self.send_scpi(&format!(":CHAN1:COUP {}", mode_str));
        }
        #[qinvokable]
        fn ch1_probe_selected(&self, probe: &QString) {
            let p = probe.to_string();
            let factor = if p.starts_with("10") { "10" } else { "1" };
            self.send_scpi(&format!(":CHAN1:PROB {}", factor));
        }

        #[qinvokable]
        fn ch2_enable_changed(&self, on: bool) {
            self.send_scpi(if on { ":CHAN2:DISP ON" } else { ":CHAN2:DISP OFF" });
        }
        #[qinvokable]
        fn ch2_scale_changed(&self, val: f64) {
            self.send_scpi(&format!(":CHAN2:SCAL {}", val));
        }
        #[qinvokable]
        fn ch2_offset_changed(&self, val: f64) {
            self.send_scpi(&format!(":CHAN2:OFFS {}", val));
        }
        #[qinvokable]
        fn ch2_coupling_selected(&self, mode: &QString) {
            self.send_scpi(&format!(":CHAN2:COUP {}", mode.to_string()));
        }
        #[qinvokable]
        fn ch2_probe_selected(&self, probe: &QString) {
            let factor = if probe.to_string().starts_with("10") { "10" } else { "1" };
            self.send_scpi(&format!(":CHAN2:PROB {}", factor));
        }

        #[qinvokable]
        fn ch3_enable_changed(&self, on: bool) {
            self.send_scpi(if on { ":CHAN3:DISP ON" } else { ":CHAN3:DISP OFF" });
        }
        #[qinvokable]
        fn ch3_scale_changed(&self, val: f64) {
            self.send_scpi(&format!(":CHAN3:SCAL {}", val));
        }
        #[qinvokable]
        fn ch3_offset_changed(&self, val: f64) {
            self.send_scpi(&format!(":CHAN3:OFFS {}", val));
        }
        #[qinvokable]
        fn ch3_coupling_selected(&self, mode: &QString) {
            self.send_scpi(&format!(":CHAN3:COUP {}", mode.to_string()));
        }
        #[qinvokable]
        fn ch3_probe_selected(&self, probe: &QString) {
            let factor = if probe.to_string().starts_with("10") { "10" } else { "1" };
            self.send_scpi(&format!(":CHAN3:PROB {}", factor));
        }

        #[qinvokable]
        fn ch4_enable_changed(&self, on: bool) {
            self.send_scpi(if on { ":CHAN4:DISP ON" } else { ":CHAN4:DISP OFF" });
        }
        #[qinvokable]
        fn ch4_scale_changed(&self, val: f64) {
            self.send_scpi(&format!(":CHAN4:SCAL {}", val));
        }
        #[qinvokable]
        fn ch4_offset_changed(&self, val: f64) {
            self.send_scpi(&format!(":CHAN4:OFFS {}", val));
        }
        #[qinvokable]
        fn ch4_coupling_selected(&self, mode: &QString) {
            self.send_scpi(&format!(":CHAN4:COUP {}", mode.to_string()));
        }
        #[qinvokable]
        fn ch4_probe_selected(&self, probe: &QString) {
            let factor = if probe.to_string().starts_with("10") { "10" } else { "1" };
            self.send_scpi(&format!(":CHAN4:PROB {}", factor));
        }
    }
}
