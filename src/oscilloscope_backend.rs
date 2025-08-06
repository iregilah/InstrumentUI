//! src/oscilloscope_backend.rs
use crate::ADDRESS;
use base64;
use once_cell::sync::OnceCell;
use rigol_cli::utils::parse_source_arg;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use tokio::runtime::Runtime;

#[cxx_qt::bridge]
mod ffi {
    use super::*;
    use cxx_qt_lib::QString;
    use std::pin::Pin;

    extern "Rust" {
        type SocketAddr;
        static ADDRESS: OnceCell<SocketAddr>;
    }

    // Helper functions for SCPI communication
    extern "Rust" {
        fn send_scpi(cmd: &QString);
        fn query_scpi(cmd: &QString) -> QString;
    }

    #[cxx_qt::qobject(thread_safe)]
    struct OscilloscopeBackend {
        /// Run/Stop state of the oscilloscope
        running: bool = true,
        /// Current timebase (s/div) setting
        current_timebase: f64 = 0.5,
        /// Shared average mode flag (for AVG checkboxes)
        #[qproperty]
        avg_enabled: bool = false,
        /// Latest scope screenshot as data URL for QML Image
        #[qproperty]
        scope_image_url: QString = QString::from(""),
    }

    impl Default for OscilloscopeBackend {
        fn default() -> Self {
            OscilloscopeBackend {
                running: true,
                current_timebase: 0.5,
                avg_enabled: false,
                scope_image_url: QString::from(""),
            }
        }
    }

    unsafe impl cxx_qt::Threading for OscilloscopeBackend {}

    impl qobject::OscilloscopeBackend {
        #[qinvokable]
        pub fn info_clicked(self: Pin<&mut Self>) {
            println!("ℹ Info");
        }
        #[qinvokable]
        pub fn settings_clicked(self: Pin<&mut Self>) {
            println!("⚙ Settings");
        }
        #[qinvokable]
        pub fn console_clicked(self: Pin<&mut Self>) {
            println!(">_ Console");
        }
        #[qinvokable]
        pub fn save_config_clicked(self: Pin<&mut Self>) {
            println!("💾 Save config (todo)");
            // Could call rigol_cli::io::save_config here if desired
        }
        #[qinvokable]
        pub fn load_config_clicked(self: Pin<&mut Self>) {
            println!("↑ Load config (todo)");
            // Could call rigol_cli::io::load_config here if desired
        }

        #[qinvokable]
        pub fn autoscale_clicked(self: Pin<&mut Self>) {
            send_scpi(&QString::from(":AUTOSCALE"));
        }
        #[qinvokable]
        pub fn trigger_source_selected(self: Pin<&mut Self>, source: &QString) {
            if let Ok(src) = parse_source_arg(&source.to_string()) {
                send_scpi(&QString::from(":TRIG:MODE EDGE"));
                let cmd = format!(":TRIG:EDGE:SOUR {}", src);
                send_scpi(&QString::from(cmd.as_str()));
            }
        }
        #[qinvokable]
        pub fn trigger_level_changed(self: Pin<&mut Self>, level: i32) {
            let cmd = format!(":TRIG:EDGE:LEV {}", level);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable]
        pub fn trigger_slope_up(self: Pin<&mut Self>) {
            send_scpi(&QString::from(":TRIG:EDGE:SLOP POS"));
        }
        #[qinvokable]
        pub fn trigger_slope_down(self: Pin<&mut Self>) {
            send_scpi(&QString::from(":TRIG:EDGE:SLOP NEG"));
        }
        #[qinvokable]
        pub fn single_trigger_clicked(self: Pin<&mut Self>) {
            send_scpi(&QString::from(":SING"));
        }
        #[qinvokable]
        pub fn run_stop_clicked(self: Pin<&mut Self>) {
            let currently_running = self.as_ref().running;
            if currently_running {
                send_scpi(&QString::from(":STOP"));
                self.as_mut().rust_mut().running = false;
            } else {
                send_scpi(&QString::from(":RUN"));
                self.as_mut().rust_mut().running = true;
            }
        }

        #[qinvokable]
        pub fn timebase_changed(self: Pin<&mut Self>, val: f64) {
            // Slider 1–100 -> 0.01–1.00 s/div
            let scale = val / 100.0;
            self.as_mut().rust_mut().current_timebase = scale;
            let cmd = format!(":TIM:SCAL {}", scale);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable]
        pub fn time_offset_changed(self: Pin<&mut Self>, val: f64) {
            // val ±100 corresponds to ±2 screens horizontal offset
            let offs = self.as_ref().current_timebase * (val / 50.0);
            let cmd = format!(":TIM:OFFS {}", offs);
            send_scpi(&QString::from(cmd.as_str()));
        }

        #[qinvokable]
        pub fn average_toggled(self: Pin<&mut Self>, on: bool) {
            self.as_mut().set_avg_enabled(on);
            if on {
                send_scpi(&QString::from(":ACQ:TYPE AVER"));
                send_scpi(&QString::from(":ACQ:AVER 16"));
            } else {
                send_scpi(&QString::from(":ACQ:TYPE NORM"));
            }
        }

        // Channel 1 controls
        #[qinvokable] pub fn ch1_enable_changed(self: Pin<&mut Self>, on: bool) {
            let cmd = if on { ":CHAN1:DISP ON" } else { ":CHAN1:DISP OFF" };
            send_scpi(&QString::from(cmd));
        }
        #[qinvokable] pub fn ch1_scale_changed(self: Pin<&mut Self>, val: f64) {
            let cmd = format!(":CHAN1:SCAL {}", val);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable] pub fn ch1_offset_changed(self: Pin<&mut Self>, val: f64) {
            let cmd = format!(":CHAN1:OFFS {}", val);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable] pub fn ch1_coupling_selected(self: Pin<&mut Self>, mode: &QString) {
            let cmd = format!(":CHAN1:COUP {}", mode.to_string());
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable] pub fn ch1_probe_selected(self: Pin<&mut Self>, probe: &QString) {
            let factor = if probe.to_string().starts_with("10") { "10" } else { "1" };
            let cmd = format!(":CHAN1:PROB {}", factor);
            send_scpi(&QString::from(cmd.as_str()));
        }
        // Channel 2
        #[qinvokable] pub fn ch2_enable_changed(self: Pin<&mut Self>, on: bool) {
            let cmd = if on { ":CHAN2:DISP ON" } else { ":CHAN2:DISP OFF" };
            send_scpi(&QString::from(cmd));
        }
        #[qinvokable] pub fn ch2_scale_changed(self: Pin<&mut Self>, val: f64) {
            let cmd = format!(":CHAN2:SCAL {}", val);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable] pub fn ch2_offset_changed(self: Pin<&mut Self>, val: f64) {
            let cmd = format!(":CHAN2:OFFS {}", val);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable] pub fn ch2_coupling_selected(self: Pin<&mut Self>, mode: &QString) {
            let cmd = format!(":CHAN2:COUP {}", mode.to_string());
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable] pub fn ch2_probe_selected(self: Pin<&mut Self>, probe: &QString) {
            let factor = if probe.to_string().starts_with("10") { "10" } else { "1" };
            let cmd = format!(":CHAN2:PROB {}", factor);
            send_scpi(&QString::from(cmd.as_str()));
        }
        // Channel 3
        #[qinvokable] pub fn ch3_enable_changed(self: Pin<&mut Self>, on: bool) {
            let cmd = if on { ":CHAN3:DISP ON" } else { ":CHAN3:DISP OFF" };
            send_scpi(&QString::from(cmd));
        }
        #[qinvokable] pub fn ch3_scale_changed(self: Pin<&mut Self>, val: f64) {
            let cmd = format!(":CHAN3:SCAL {}", val);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable] pub fn ch3_offset_changed(self: Pin<&mut Self>, val: f64) {
            let cmd = format!(":CHAN3:OFFS {}", val);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable] pub fn ch3_coupling_selected(self: Pin<&mut Self>, mode: &QString) {
            let cmd = format!(":CHAN3:COUP {}", mode.to_string());
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable] pub fn ch3_probe_selected(self: Pin<&mut Self>, probe: &QString) {
            let factor = if probe.to_string().starts_with("10") { "10" } else { "1" };
            let cmd = format!(":CHAN3:PROB {}", factor);
            send_scpi(&QString::from(cmd.as_str()));
        }
        // Channel 4
        #[qinvokable] pub fn ch4_enable_changed(self: Pin<&mut Self>, on: bool) {
            let cmd = if on { ":CHAN4:DISP ON" } else { ":CHAN4:DISP OFF" };
            send_scpi(&QString::from(cmd));
        }
        #[qinvokable] pub fn ch4_scale_changed(self: Pin<&mut Self>, val: f64) {
            let cmd = format!(":CHAN4:SCAL {}", val);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable] pub fn ch4_offset_changed(self: Pin<&mut Self>, val: f64) {
            let cmd = format!(":CHAN4:OFFS {}", val);
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable] pub fn ch4_coupling_selected(self: Pin<&mut Self>, mode: &QString) {
            let cmd = format!(":CHAN4:COUP {}", mode.to_string());
            send_scpi(&QString::from(cmd.as_str()));
        }
        #[qinvokable] pub fn ch4_probe_selected(self: Pin<&mut Self>, probe: &QString) {
            let factor = if probe.to_string().starts_with("10") { "10" } else { "1" };
            let cmd = format!(":CHAN4:PROB {}", factor);
            send_scpi(&QString::from(cmd.as_str()));
        }
    }

    impl cxx_qt::Initialize for qobject::OscilloscopeBackend {
        fn initialize(&mut self) {
            // Initial SCPI configuration: ensure CH1 on and AWG outputs off
            if let Some(addr) = ADDRESS.get() {
                let _ = TcpStream::connect_timeout(addr, Duration::from_millis(500))
                    .and_then(|mut s| s.write_all(b":CHAN1:DISP ON\n"));
                let _ = TcpStream::connect_timeout(addr, Duration::from_millis(500))
                    .and_then(|mut s| s.write_all(b":OUTPUT1 OFF\n"));
                let _ = TcpStream::connect_timeout(addr, Duration::from_millis(500))
                    .and_then(|mut s| s.write_all(b":OUTPUT2 OFF\n"));
                println!("(Init) CH1 display → ON  [{addr}]");
            }
            self.running = true;
            // Start background thread to fetch oscilloscope screen images periodically
            let qt_thread = self.qt_thread();
            std::thread::spawn(move || {
                loop {
                    if let Some(addr) = ADDRESS.get() {
                        if let Ok(mut stream) = TcpStream::connect(addr) {
                            // Request display data (PNG)
                            let _ = stream.write_all(b":DISP:DATA?\n");
                            let _ = stream.flush();
                            // Read IEEE-488.2 binary block header
                            let mut hdr = [0u8; 2];
                            if stream.read_exact(&mut hdr).is_err() || hdr[0] != b'#' {
                                std::thread::sleep(Duration::from_millis(200));
                                continue;
                            }
                            let ndigits = (hdr[1] - b'0') as usize;
                            if ndigits == 0 || ndigits > 9 {
                                std::thread::sleep(Duration::from_millis(200));
                                continue;
                            }
                            let mut len_buf = vec![0u8; ndigits];
                            if stream.read_exact(&mut len_buf).is_err() {
                                std::thread::sleep(Duration::from_millis(200));
                                continue;
                            }
                            let total_len = String::from_utf8_lossy(&len_buf).parse::<usize>().unwrap_or(0);
                            if total_len == 0 {
                                std::thread::sleep(Duration::from_millis(200));
                                continue;
                            }
                            let mut img_data = vec![0u8; total_len];
                            if stream.read_exact(&mut img_data).is_err() {
                                std::thread::sleep(Duration::from_millis(200));
                                continue;
                            }
                            // Convert image bytes to base64 data URL
                            let encoded = base64::encode(&img_data);
                            let data_url = format!("data:image/png;base64,{}", encoded);
                            // Update image property on Qt GUI thread
                            qt_thread.queue(|qobj| {
                                qobj.set_scope_image_url(QString::from(&data_url));
                            });
                        }
                    }
                    std::thread::sleep(Duration::from_millis(200));
                }
            });
        }
    }
}

// SCPI helper implementations
fn send_scpi(cmd: &QString) {
    if let Some(addr) = ADDRESS.get() {
        if let Ok(mut stream) = TcpStream::connect_timeout(addr, Duration::from_millis(500)) {
            let _ = stream.write_all(cmd.to_string().as_bytes());
            let _ = stream.write_all(b"\n");
            let _ = stream.flush();
        }
    }
}
fn query_scpi(cmd: &QString) -> QString {
    if let Some(addr) = ADDRESS.get() {
        if let Ok(mut stream) = TcpStream::connect_timeout(addr, Duration::from_millis(500)) {
            let _ = stream.write_all(cmd.to_string().as_bytes());
            let _ = stream.write_all(b"\n");
            let _ = stream.flush();
            let mut buf = [0u8; 512];
            if let Ok(n) = stream.read(&mut buf) {
                if n > 0 {
                    let resp = String::from_utf8_lossy(&buf[..n]).trim_end().to_owned();
                    return QString::from(resp.as_str());
                }
            }
        }
    }
    QString::from("")
}
