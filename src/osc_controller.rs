// src/osc_controller.rs
#[cxx_qt::bridge]
mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, address)]
        #[qproperty(QString, imageSource)]
        #[qproperty(bool, avgEnabled)]
        type OscController = super::OscControllerRust;
    }

    extern "RustQt" {
        #[qinvokable]
        fn info(self: &OscController);
        #[qinvokable]
        fn settings(self: &OscController);
        #[qinvokable]
        fn autoscale(self: &OscController);
        #[qinvokable]
        fn console(self: &OscController);
        #[qinvokable]
        fn save_config(self: &OscController);
        #[qinvokable]
        fn load_config(self: &OscController);

        #[qinvokable]
        fn trigger_source(self: &OscController, source: &QString);
        #[qinvokable]
        fn trigger_level(self: &OscController, level: i32);
        #[qinvokable]
        fn trigger_slope_up(self: &OscController);
        #[qinvokable]
        fn trigger_slope_down(self: &OscController);
        #[qinvokable]
        fn single(self: &OscController);
        #[qinvokable]
        fn run_stop(self: &OscController);

        #[qinvokable]
        fn timebase(self: Pin<&mut OscController>, value: f64);
        #[qinvokable]
        fn time_offset(self: &OscController, value: f64);
        #[qinvokable]
        fn average(self: Pin<&mut OscController>, on: bool);

        #[qinvokable]
        fn ch1_enable(self: &OscController, on: bool);
        #[qinvokable]
        fn ch1_scale(self: &OscController, value: f64);
        #[qinvokable]
        fn ch1_offset(self: &OscController, value: f64);
        #[qinvokable]
        fn ch1_coupling(self: &OscController, mode: &QString);
        #[qinvokable]
        fn ch1_probe(self: &OscController, probe: &QString);

        #[qinvokable]
        fn ch2_enable(self: &OscController, on: bool);
        #[qinvokable]
        fn ch2_scale(self: &OscController, value: f64);
        #[qinvokable]
        fn ch2_offset(self: &OscController, value: f64);
        #[qinvokable]
        fn ch2_coupling(self: &OscController, mode: &QString);
        #[qinvokable]
        fn ch2_probe(self: &OscController, probe: &QString);

        #[qinvokable]
        fn ch3_enable(self: &OscController, on: bool);
        #[qinvokable]
        fn ch3_scale(self: &OscController, value: f64);
        #[qinvokable]
        fn ch3_offset(self: &OscController, value: f64);
        #[qinvokable]
        fn ch3_coupling(self: &OscController, mode: &QString);
        #[qinvokable]
        fn ch3_probe(self: &OscController, probe: &QString);

        #[qinvokable]
        fn ch4_enable(self: &OscController, on: bool);
        #[qinvokable]
        fn ch4_scale(self: &OscController, value: f64);
        #[qinvokable]
        fn ch4_offset(self: &OscController, value: f64);
        #[qinvokable]
        fn ch4_coupling(self: &OscController, mode: &QString);
        #[qinvokable]
        fn ch4_probe(self: &OscController, probe: &QString);

        #[qinvokable]
        fn start_update(self: Pin<&mut OscController>);
    }

    impl cxx_qt::Threading for OscController {}
}

use core::pin::Pin;
use std::net::{SocketAddr, TcpStream};
use std::io::{Write, Read};
use std::time::Duration;
use base64;
use rigol_cli::utils::parse_source_arg;

#[derive(Default)]
pub struct OscControllerRust {
    address: cxx_qt_lib::QString,
    image_source: cxx_qt_lib::QString,
    avg_enabled: bool,
    current_timebase: f64,
}

impl Default for OscControllerRust {
    fn default() -> Self {
        Self {
            address: cxx_qt_lib::QString::from("169.254.50.23:5555"),
            image_source: cxx_qt_lib::QString::default(),
            avg_enabled: false,
            current_timebase: 0.5,
        }
    }
}

impl qobject::OscController {
    fn info(&self) {
        println!("ℹ Info");
    }
    fn settings(&self) {
        println!("⚙ Settings");
    }
    fn autoscale(&self) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, ":AUTOSCALE");
        }
    }
    fn console(&self) {
        println!(">_ Console");
    }
    fn save_config(&self) {
        println!("💾 Save config (todo)");
    }
    fn load_config(&self) {
        println!("↑ Load config (todo)");
    }

    fn trigger_source(&self, source: &cxx_qt_lib::QString) {
        let src_str = source.to_string();
        if let Ok(param) = parse_source_arg(&src_str) {
            if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
                // Set trigger to edge mode with selected source
                send_scpi_blocking(&addr, ":TRIG:MODE EDGE");
                send_scpi_blocking(&addr, &format!(":TRIG:EDGE:SOUR {}", param));
            }
        }
    }
    fn trigger_level(&self, level: i32) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":TRIG:EDGE:LEV {}", level));
        }
    }
    fn trigger_slope_up(&self) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, ":TRIG:EDGE:SLOP POS");
        }
    }
    fn trigger_slope_down(&self) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, ":TRIG:EDGE:SLOP NEG");
        }
    }
    fn single(&self) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, ":SING");
        }
    }
    fn run_stop(&self) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            // If currently running (we don't track, assume toggle)
            // We can send a STOP if currently running or RUN if stopped.
            // For simplicity, always send RUN (start) if currently stopped would be known, but we don't know.
            // Instead, we toggle by querying? The original toggled a flag.
            // We cannot easily track run state unless we maintain a bool.
            // Alternatively, just send :RUN unconditionally or cycle STOP->RUN.
            // For demonstration, we'll send STOP the first time and then RUN next time toggled.
            // This requires storing state. We haven't stored, let's keep a static within this function:
            static mut IS_RUNNING: bool = true;
            unsafe {
                if IS_RUNNING {
                    send_scpi_blocking(&addr, ":STOP");
                    IS_RUNNING = false;
                } else {
                    send_scpi_blocking(&addr, ":RUN");
                    IS_RUNNING = true;
                }
            }
        }
    }

    fn timebase(mut self: Pin<&mut Self>, value: f64) {
        let scale = value / 100.0;
        self.as_mut().rust_mut().current_timebase = scale;
        if let Ok(addr) = self.as_ref().address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":TIM:SCAL {}", scale));
        }
    }
    fn time_offset(&self, value: f64) {
        let base = self.rust().current_timebase;
        let offs = base * (value / 50.0);
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":TIM:OFFS {}", offs));
        }
    }
    fn average(mut self: Pin<&mut Self>, on: bool) {
        if let Ok(addr) = self.as_ref().address().to_string().parse::<SocketAddr>() {
            if on {
                send_scpi_blocking(&addr, ":ACQ:TYPE AVER");
                send_scpi_blocking(&addr, ":ACQ:AVER 16");
            } else {
                send_scpi_blocking(&addr, ":ACQ:TYPE NORM");
            }
        }
        self.as_mut().set_avg_enabled(on);
    }

    fn ch1_enable(&self, on: bool) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let cmd = if on { ":CHAN1:DISP ON" } else { ":CHAN1:DISP OFF" };
            send_scpi_blocking(&addr, cmd);
        }
    }
    fn ch1_scale(&self, value: f64) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":CHAN1:SCAL {}", value));
        }
    }
    fn ch1_offset(&self, value: f64) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":CHAN1:OFFS {}", value));
        }
    }
    fn ch1_coupling(&self, mode: &cxx_qt_lib::QString) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let mode_str = mode.to_string();
            send_scpi_blocking(&addr, &format!(":CHAN1:COUP {}", mode_str));
        }
    }
    fn ch1_probe(&self, probe: &cxx_qt_lib::QString) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let p = probe.to_string();
            let factor = if p.starts_with("10") { "10" } else { "1" };
            send_scpi_blocking(&addr, &format!(":CHAN1:PROB {}", factor));
        }
    }

    fn ch2_enable(&self, on: bool) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let cmd = if on { ":CHAN2:DISP ON" } else { ":CHAN2:DISP OFF" };
            send_scpi_blocking(&addr, cmd);
        }
    }
    fn ch2_scale(&self, value: f64) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":CHAN2:SCAL {}", value));
        }
    }
    fn ch2_offset(&self, value: f64) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":CHAN2:OFFS {}", value));
        }
    }
    fn ch2_coupling(&self, mode: &cxx_qt_lib::QString) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let mode_str = mode.to_string();
            send_scpi_blocking(&addr, &format!(":CHAN2:COUP {}", mode_str));
        }
    }
    fn ch2_probe(&self, probe: &cxx_qt_lib::QString) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let p = probe.to_string();
            let factor = if p.starts_with("10") { "10" } else { "1" };
            send_scpi_blocking(&addr, &format!(":CHAN2:PROB {}", factor));
        }
    }

    fn ch3_enable(&self, on: bool) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let cmd = if on { ":CHAN3:DISP ON" } else { ":CHAN3:DISP OFF" };
            send_scpi_blocking(&addr, cmd);
        }
    }
    fn ch3_scale(&self, value: f64) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":CHAN3:SCAL {}", value));
        }
    }
    fn ch3_offset(&self, value: f64) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":CHAN3:OFFS {}", value));
        }
    }
    fn ch3_coupling(&self, mode: &cxx_qt_lib::QString) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let mode_str = mode.to_string();
            send_scpi_blocking(&addr, &format!(":CHAN3:COUP {}", mode_str));
        }
    }
    fn ch3_probe(&self, probe: &cxx_qt_lib::QString) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let p = probe.to_string();
            let factor = if p.starts_with("10") { "10" } else { "1" };
            send_scpi_blocking(&addr, &format!(":CHAN3:PROB {}", factor));
        }
    }

    fn ch4_enable(&self, on: bool) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let cmd = if on { ":CHAN4:DISP ON" } else { ":CHAN4:DISP OFF" };
            send_scpi_blocking(&addr, cmd);
        }
    }
    fn ch4_scale(&self, value: f64) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":CHAN4:SCAL {}", value));
        }
    }
    fn ch4_offset(&self, value: f64) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":CHAN4:OFFS {}", value));
        }
    }
    fn ch4_coupling(&self, mode: &cxx_qt_lib::QString) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let mode_str = mode.to_string();
            send_scpi_blocking(&addr, &format!(":CHAN4:COUP {}", mode_str));
        }
    }
    fn ch4_probe(&self, probe: &cxx_qt_lib::QString) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let p = probe.to_string();
            let factor = if p.starts_with("10") { "10" } else { "1" };
            send_scpi_blocking(&addr, &format!(":CHAN4:PROB {}", factor));
        }
    }

    fn start_update(mut self: Pin<&mut Self>) {
        let qt_thread = self.as_mut().qt_thread();
        if let Ok(addr) = self.as_ref().address().to_string().parse::<SocketAddr>() {
            std::thread::spawn(move || {
                loop {
                    match fetch_screenshot(&addr) {
                        Ok(image_data) => {
                            let encoded = base64::encode(&image_data);
                            let data_url = format!("data:image/png;base64,{}", encoded);
                            // Queue update of imageSource property on Qt thread
                            let _ = qt_thread.queue(|qobj| {
                                qobj.set_image_source(cxx_qt_lib::QString::from(&data_url));
                            });
                        }
                        Err(e) => {
                            eprintln!("Screenshot fetch error: {}", e);
                            break;
                        }
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
            });
        }
    }
}

// Helper functions for SCPI communication
fn send_scpi_blocking(addr: &SocketAddr, cmd: &str) {
    if let Ok(mut stream) = TcpStream::connect_timeout(addr, Duration::from_millis(500)) {
        let _ = stream.write_all(format!("{}\n", cmd).as_bytes());
        let _ = stream.flush();
    }
}

fn fetch_screenshot(addr: &SocketAddr) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect_timeout(addr, Duration::from_millis(1000))?;
    stream.write_all(b":DISP:DATA?\n")?;
    stream.flush()?;
    // Read binary block header
    let mut hdr = [0u8; 2];
    stream.read_exact(&mut hdr)?;
    if hdr[0] != b'#' {
        return Err("Invalid screenshot header".into());
    }
    let ndigits = (hdr[1] as char).to_digit(10).ok_or("Invalid length digit")? as usize;
    let mut len_buf = vec![0u8; ndigits];
    stream.read_exact(&mut len_buf)?;
    let len_str = std::str::from_utf8(&len_buf)?.trim();
    let data_len: usize = len_str.parse()?;
    let mut data = vec![0u8; data_len];
    stream.read_exact(&mut data)?;
    // Attempt to read trailing newline (not strictly needed)
    let _ = stream.read(&mut [0u8; 1]);
    Ok(data)
}
