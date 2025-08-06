// src/gui.rs
use std::net::{SocketAddr, TcpStream};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use std::time::Duration;
use tokio::runtime::Runtime;
use cxx_qt_lib::QString;

// Global static for instrument address and quit flag
static ADDR: OnceLock<SocketAddr> = OnceLock::new();
static QUIT: AtomicBool = AtomicBool::new(false);

// Helper function to send a SCPI command (no response expected)
fn send_scpi(cmd: &str) {
    if let Some(addr) = ADDR.get() {
        if let Ok(mut s) = TcpStream::connect_timeout(addr, Duration::from_millis(500)) {
            let _ = s.write_all(format!("{}\n", cmd).as_bytes());
            let _ = s.flush();
        }
    }
}

// Helper function to query a SCPI command (return response or empty string)
fn query_scpi(cmd: &str) -> String {
    let mut result = String::new();
    if let Some(addr) = ADDR.get() {
        if let Ok(mut s) = TcpStream::connect_timeout(addr, Duration::from_millis(500)) {
            let _ = s.write_all(format!("{}\n", cmd).as_bytes());
            let _ = s.flush();
            let mut buf = [0u8; 512];
            if let Ok(n) = s.read(&mut buf) {
                if n > 0 {
                    result = String::from_utf8_lossy(&buf[..n]).trim_end_matches(&['\r','\n'][..]).to_string();
                }
            }
        }
    }
    result
}

#[cxx_qt::bridge(cxx_file_stem = "instrument_ui")]
mod ffi {
    unsafe extern "C++" {
        include!("QtCore/QString");
        type QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(bool, avgEnabled)]
        #[qproperty(QString, scopeImageData)]
        type Backend = super::Backend;
    }
}

#[derive(Default)]
struct Backend {
    avg_enabled: bool,
    scope_image_data: QString,
    running: bool,
    current_timebase: f64,
}

impl qobject::Backend {
    // Invokable functions for UI interactions
    #[qinvokable]
    #[cxx_name = "infoClicked"]
    pub fn info_clicked(&self) {
        println!("ℹ Info");
    }

    #[qinvokable]
    #[cxx_name = "settingsClicked"]
    pub fn settings_clicked(&self) {
        println!("⚙ Settings");
    }

    #[qinvokable]
    #[cxx_name = "consoleClicked"]
    pub fn console_clicked(&self) {
        println!(">_ Console");
    }

    #[qinvokable]
    #[cxx_name = "saveConfig"]
    pub fn save_config(&self) {
        println!("💾 Save config (todo)");
    }

    #[qinvokable]
    #[cxx_name = "loadConfig"]
    pub fn load_config(&self) {
        println!("↑ Load config (todo)");
    }

    #[qinvokable]
    #[cxx_name = "autoscaleClicked"]
    pub fn autoscale_clicked(&self) {
        send_scpi(":AUToscale");
    }

    #[qinvokable]
    #[cxx_name = "triggerSourceSelected"]
    pub fn trigger_source_selected(&self, source: &QString) {
        // Determine SCPI trigger source string from combo selection
        let src_str = source.to_string().to_ascii_uppercase();
        let p = if src_str.starts_with("CH") {
            let num = src_str.trim_start_matches("CH");
            format!("CHANnel{}", num)
        } else if src_str == "EXT" {
            "EXT".to_string()
        } else {
            return;
        };
        send_scpi(":TRIG:MODE EDGE");
        send_scpi(&format!(":TRIG:EDGE:SOUR {}", p));
    }

    #[qinvokable]
    #[cxx_name = "triggerLevelChanged"]
    pub fn trigger_level_changed(&self, level: i32) {
        send_scpi(&format!(":TRIG:EDGE:LEV {}", level));
    }

    #[qinvokable]
    #[cxx_name = "triggerSlopeUp"]
    pub fn trigger_slope_up(&self) {
        send_scpi(":TRIG:EDGE:SLOP POS");
    }

    #[qinvokable]
    #[cxx_name = "triggerSlopeDown"]
    pub fn trigger_slope_down(&self) {
        send_scpi(":TRIG:EDGE:SLOP NEG");
    }

    #[qinvokable]
    #[cxx_name = "singleTriggerClicked"]
    pub fn single_trigger_clicked(&self) {
        send_scpi(":SING");
    }

    #[qinvokable]
    #[cxx_name = "runStopClicked"]
    pub fn run_stop_clicked(self: Pin<&mut Self>) {
        let was_running = self.running;
        if was_running {
            send_scpi(":STOP");
            self.running = false;
        } else {
            send_scpi(":RUN");
            self.running = true;
        }
    }

    #[qinvokable]
    #[cxx_name = "timebaseChanged"]
    pub fn timebase_changed(self: Pin<&mut Self>, val: f64) {
        // 1–100 slider -> 0.01–1.00 s/div scale
        let scale = val / 100.0;
        self.current_timebase = scale;
        send_scpi(&format!(":TIM:SCAL {}", scale));
    }

    #[qinvokable]
    #[cxx_name = "timeOffsetChanged"]
    pub fn time_offset_changed(&self, val: f64) {
        // Compute time offset (±2 screen widths corresponds to ±100)
        let base = self.current_timebase;
        let offs = base * (val / 50.0);
        send_scpi(&format!(":TIM:OFFS {}", offs));
    }

    #[qinvokable]
    #[cxx_name = "averageToggled"]
    pub fn average_toggled(self: Pin<&mut Self>, on: bool) {
        if on {
            send_scpi(":ACQ:TYPE AVER");
            send_scpi(":ACQ:AVER 16");
        } else {
            send_scpi(":ACQ:TYPE NORM");
        }
        // Update avgEnabled property for UI checkboxes
        self.set_avg_enabled(on);
    }

    // Channel 1-4 controls
    #[qinvokable]
    #[cxx_name = "ch1EnableChanged"]
    pub fn ch1_enable_changed(&self, on: bool) {
        send_scpi(if on { ":CHAN1:DISP ON" } else { ":CHAN1:DISP OFF" });
    }
    #[qinvokable]
    #[cxx_name = "ch1ScaleChanged"]
    pub fn ch1_scale_changed(&self, val: f64) {
        send_scpi(&format!(":CHAN1:SCAL {}", val));
    }
    #[qinvokable]
    #[cxx_name = "ch1OffsetChanged"]
    pub fn ch1_offset_changed(&self, val: f64) {
        send_scpi(&format!(":CHAN1:OFFS {}", val));
    }
    #[qinvokable]
    #[cxx_name = "ch1CouplingSelected"]
    pub fn ch1_coupling_selected(&self, mode: &QString) {
        let mode_str = mode.to_string();
        send_scpi(&format!(":CHAN1:COUP {}", mode_str));
    }
    #[qinvokable]
    #[cxx_name = "ch1ProbeSelected"]
    pub fn ch1_probe_selected(&self, probe: &QString) {
        let p = probe.to_string();
        let factor = if p.starts_with("10") { "10" } else { "1" };
        send_scpi(&format!(":CHAN1:PROB {}", factor));
    }

    #[qinvokable]
    #[cxx_name = "ch2EnableChanged"]
    pub fn ch2_enable_changed(&self, on: bool) {
        send_scpi(if on { ":CHAN2:DISP ON" } else { ":CHAN2:DISP OFF" });
    }
    #[qinvokable]
    #[cxx_name = "ch2ScaleChanged"]
    pub fn ch2_scale_changed(&self, val: f64) {
        send_scpi(&format!(":CHAN2:SCAL {}", val));
    }
    #[qinvokable]
    #[cxx_name = "ch2OffsetChanged"]
    pub fn ch2_offset_changed(&self, val: f64) {
        send_scpi(&format!(":CHAN2:OFFS {}", val));
    }
    #[qinvokable]
    #[cxx_name = "ch2CouplingSelected"]
    pub fn ch2_coupling_selected(&self, mode: &QString) {
        send_scpi(&format!(":CHAN2:COUP {}", mode.to_string()));
    }
    #[qinvokable]
    #[cxx_name = "ch2ProbeSelected"]
    pub fn ch2_probe_selected(&self, probe: &QString) {
        let factor = if probe.to_string().starts_with("10") { "10" } else { "1" };
        send_scpi(&format!(":CHAN2:PROB {}", factor));
    }

    #[qinvokable]
    #[cxx_name = "ch3EnableChanged"]
    pub fn ch3_enable_changed(&self, on: bool) {
        send_scpi(if on { ":CHAN3:DISP ON" } else { ":CHAN3:DISP OFF" });
    }
    #[qinvokable]
    #[cxx_name = "ch3ScaleChanged"]
    pub fn ch3_scale_changed(&self, val: f64) {
        send_scpi(&format!(":CHAN3:SCAL {}", val));
    }
    #[qinvokable]
    #[cxx_name = "ch3OffsetChanged"]
    pub fn ch3_offset_changed(&self, val: f64) {
        send_scpi(&format!(":CHAN3:OFFS {}", val));
    }
    #[qinvokable]
    #[cxx_name = "ch3CouplingSelected"]
    pub fn ch3_coupling_selected(&self, mode: &QString) {
        send_scpi(&format!(":CHAN3:COUP {}", mode.to_string()));
    }
    #[qinvokable]
    #[cxx_name = "ch3ProbeSelected"]
    pub fn ch3_probe_selected(&self, probe: &QString) {
        let factor = if probe.to_string().starts_with("10") { "10" } else { "1" };
        send_scpi(&format!(":CHAN3:PROB {}", factor));
    }

    #[qinvokable]
    #[cxx_name = "ch4EnableChanged"]
    pub fn ch4_enable_changed(&self, on: bool) {
        send_scpi(if on { ":CHAN4:DISP ON" } else { ":CHAN4:DISP OFF" });
    }
    #[qinvokable]
    #[cxx_name = "ch4ScaleChanged"]
    pub fn ch4_scale_changed(&self, val: f64) {
        send_scpi(&format!(":CHAN4:SCAL {}", val));
    }
    #[qinvokable]
    #[cxx_name = "ch4OffsetChanged"]
    pub fn ch4_offset_changed(&self, val: f64) {
        send_scpi(&format!(":CHAN4:OFFS {}", val));
    }
    #[qinvokable]
    #[cxx_name = "ch4CouplingSelected"]
    pub fn ch4_coupling_selected(&self, mode: &QString) {
        send_scpi(&format!(":CHAN4:COUP {}", mode.to_string()));
    }
    #[qinvokable]
    #[cxx_name = "ch4ProbeSelected"]
    pub fn ch4_probe_selected(&self, probe: &QString) {
        let factor = if probe.to_string().starts_with("10") { "10" } else { "1" };
        send_scpi(&format!(":CHAN4:PROB {}", factor));
    }

    // AWG (Function Generator) channel 1 & 2 controls
    #[qinvokable]
    #[cxx_name = "awg1EnableChanged"]
    pub fn awg1_enable_changed(&self, on: bool) {
        send_scpi(&format!(":OUTPUT1 {}", if on { "ON" } else { "OFF" }));
    }

    #[qinvokable]
    #[cxx_name = "awg2EnableChanged"]
    pub fn awg2_enable_changed(&self, on: bool) {
        send_scpi(&format!(":OUTPUT2 {}", if on { "ON" } else { "OFF" }));
    }

    #[qinvokable]
    #[cxx_name = "awg1WaveformSelected"]
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
        send_scpi(&format!(":SOUR1:FUNC {}", scpi_type));
    }

    #[qinvokable]
    #[cxx_name = "awg2WaveformSelected"]
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
        send_scpi(&format!(":SOUR2:FUNC {}", scpi_type));
    }

    #[qinvokable]
    #[cxx_name = "awg1FreqChanged"]
    pub fn awg1_freq_changed(&self, freq: i32) {
        send_scpi(&format!(":SOUR1:FREQ {}", freq));
    }

    #[qinvokable]
    #[cxx_name = "awg2FreqChanged"]
    pub fn awg2_freq_changed(&self, freq: i32) {
        send_scpi(&format!(":SOUR2:FREQ {}", freq));
    }

    #[qinvokable]
    #[cxx_name = "awg1AmpChanged"]
    pub fn awg1_amp_changed(&self, ampl: f64) {
        send_scpi(&format!(":SOUR1:VOLT {}", ampl));
    }

    #[qinvokable]
    #[cxx_name = "awg2AmpChanged"]
    pub fn awg2_amp_changed(&self, ampl: f64) {
        send_scpi(&format!(":SOUR2:VOLT {}", ampl));
    }

    #[qinvokable]
    #[cxx_name = "awg1OffsetChanged"]
    pub fn awg1_offset_changed(&self, offs: f64) {
        send_scpi(&format!(":SOUR1:VOLT:OFFS {}", offs));
    }

    #[qinvokable]
    #[cxx_name = "awg2OffsetChanged"]
    pub fn awg2_offset_changed(&self, offs: f64) {
        send_scpi(&format!(":SOUR2:VOLT:OFFS {}", offs));
    }

    #[qinvokable]
    #[cxx_name = "awg1LoadArb"]
    pub fn awg1_load_arb(&self, file_path: &QString) {
        let path_str = file_path.to_string();
        let addr = *ADDR.get().unwrap();
        std::thread::spawn(move || {
            // Use rigol_cli's asynchronous loader on a fresh runtime
            let rt = Runtime::new().expect("Tokio runtime init");
            if let Err(e) = rt.block_on(rigol_cli::io::load_arb(&addr, 1, &path_str)) {
                eprintln!("Arb file load failed: {}", e);
            } else {
                // After loading, set AWG1 to USER waveform
                if let Ok(mut s) = TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
                    let _ = s.write_all(b":SOUR1:FUNC USER\n");
                    let _ = s.flush();
                }
            }
        });
    }

    #[qinvokable]
    #[cxx_name = "awg2LoadArb"]
    pub fn awg2_load_arb(&self, file_path: &QString) {
        let path_str = file_path.to_string();
        let addr = *ADDR.get().unwrap();
        std::thread::spawn(move || {
            let rt = Runtime::new().expect("Tokio runtime init");
            if let Err(e) = rt.block_on(rigol_cli::io::load_arb(&addr, 2, &path_str)) {
                eprintln!("Arb file load failed: {}", e);
            } else {
                if let Ok(mut s) = TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
                    let _ = s.write_all(b":SOUR2:FUNC USER\n");
                    let _ = s.flush();
                }
            }
        });
    }

    #[qinvokable]
    #[cxx_name = "startAcquisition"]
    pub fn start_acquisition(self: Pin<&mut Self>) {
        // Initial instrument setup
        if let Some(addr) = ADDR.get() {
            send_scpi(":CHAN1:DISP ON");
            send_scpi(":OUTPUT1 OFF");
            send_scpi(":OUTPUT2 OFF");
            println!("(Init) CH1 display → ON  [{}]", addr);
        }
        // Start background thread to continuously fetch oscilloscope display data
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            loop {
                if QUIT.load(Ordering::SeqCst) {
                    break;
                }
                if let Some(addr) = ADDR.get() {
                    if let Ok(mut s) = TcpStream::connect(addr) {
                        let _ = s.write_all(b":DISP:DATA?\n");
                        let _ = s.flush();
                        // Read IEEE-488.2 binary block
                        let mut hdr = [0u8; 2];
                        if s.read_exact(&mut hdr).is_err() || hdr[0] != b'#' {
                            std::thread::sleep(Duration::from_millis(200));
                            continue;
                        }
                        let ndigits = (hdr[1] as char).to_digit(10).unwrap_or(0) as usize;
                        let mut len_buf = vec![0u8; ndigits];
                        if s.read_exact(&mut len_buf).is_err() {
                            std::thread::sleep(Duration::from_millis(200));
                            continue;
                        }
                        let total_bytes = String::from_utf8_lossy(&len_buf).parse::<usize>().unwrap_or(0);
                        if total_bytes == 0 {
                            std::thread::sleep(Duration::from_millis(200));
                            continue;
                        }
                        let mut img_data = vec![0u8; total_bytes];
                        if s.read_exact(&mut img_data).is_err() {
                            std::thread::sleep(Duration::from_millis(200));
                            continue;
                        }
                        // Convert image data (PNG) to base64 data URL
                        let base64_png = base64::encode(&img_data);
                        let data_url = format!("data:image/png;base64,{}", base64_png);
                        let qstr = QString::from(data_url.as_str());
                        qt_thread.queue(move |mut backend| {
                            backend.set_scope_image_data(qstr.clone());
                        });
                    }
                }
                std::thread::sleep(Duration::from_millis(200));
            }
        });
    }
}
