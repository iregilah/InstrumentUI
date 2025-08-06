// src/oscillo_object.rs
use core::pin::Pin;
use std::net::SocketAddr;
use std::io::{Read, Write};
use cxx_qt_lib::QString;

macro_rules! chan_handlers {
    ($idx:literal,
     $on:ident, $sc:ident, $off:ident,
    $coup:ident, $prob:ident) => {
        /// Láthatóság (csatorna be/ki)
        pub fn $on(&self, on: bool) {
            self.send_scpi_sync(&format!(
                ":CHAN{}:DISP {}",
                $idx,
                if on { "ON" } else { "OFF" }
            ));
        }
        /// Vertikális skála [V/div]
        pub fn $sc(&self, val: f64) {
            self.send_scpi_sync(&format!(":CHAN{}:SCAL {}", $idx, val));
        }
        /// Offset [V]
        pub fn $off(&self, val: f64) {
            self.send_scpi_sync(&format!(":CHAN{}:OFFS {}", $idx, val));
        }
        /// AC/DC/GND kapcsolás
        pub fn $coup(&self, mode: &QString) {
            self.send_scpi_sync(&format!(":CHAN{}:COUP {}", $idx, mode.to_string()));
        }
        /// Szonda (1× / 10×)
        pub fn $prob(&self, probe: &QString) {
            let factor = if probe.to_string().starts_with("10") { "10" } else { "1" };
            self.send_scpi_sync(&format!(":CHAN{}:PROB {}", $idx, factor));
        }
    };
}
#[cxx_qt::bridge]
pub mod oscillo_qobject {
    // Bring in Qt types
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        include!("cxx-qt-lib/qimage.h");
        type QString = cxx_qt_lib::QString;
        type QImage = cxx_qt_lib::QImage;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(bool, avgEnabled)]
        #[qproperty(QString, scopeImageUrl)]
        type OscilloObject = super::OscilloObjectRust;
    }

    extern "RustQt" {
        #[qinvokable]
        fn info_clicked(self: &OscilloObject);
        #[qinvokable]
        fn settings_clicked(self: &OscilloObject);
        #[qinvokable]
        fn autoscale(self: &OscilloObject);
        #[qinvokable]
        fn console_clicked(self: &OscilloObject);
        #[qinvokable]
        fn save_config(self: &OscilloObject);
        #[qinvokable]
        fn load_config(self: &OscilloObject);
        #[qinvokable]
        fn toggle_console_log(self: &OscilloObject);

        #[qinvokable]
        fn trigger_source_selected(self: &OscilloObject, source: &QString);
        #[qinvokable]
        fn trigger_level_changed(self: &OscilloObject, level: i32);
        #[qinvokable]
        fn trigger_slope_up(self: &OscilloObject);
        #[qinvokable]
        fn trigger_slope_down(self: &OscilloObject);
        #[qinvokable]
        fn single_trigger(self: &OscilloObject);
        #[qinvokable]
        fn run_stop(self: Pin<&mut OscilloObject>);

        #[qinvokable]
        fn timebase_changed(self: &OscilloObject, val: f64);
        #[qinvokable]
        fn time_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable]
        fn average_toggled(self: &OscilloObject, on: bool);

        #[qinvokable]
        fn ch1_enable_changed(self: &OscilloObject, on: bool);
        #[qinvokable]
        fn ch1_scale_changed(self: &OscilloObject, val: f64);
        #[qinvokable]
        fn ch1_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable]
        fn ch1_coupling_selected(self: &OscilloObject, mode: &QString);
        #[qinvokable]
        fn ch1_probe_selected(self: &OscilloObject, probe: &QString);

        #[qinvokable]
        fn ch2_enable_changed(self: &OscilloObject, on: bool);
        #[qinvokable]
        fn ch2_scale_changed(self: &OscilloObject, val: f64);
        #[qinvokable]
        fn ch2_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable]
        fn ch2_coupling_selected(self: &OscilloObject, mode: &QString);
        #[qinvokable]
        fn ch2_probe_selected(self: &OscilloObject, probe: &QString);

        #[qinvokable]
        fn ch3_enable_changed(self: &OscilloObject, on: bool);
        #[qinvokable]
        fn ch3_scale_changed(self: &OscilloObject, val: f64);
        #[qinvokable]
        fn ch3_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable]
        fn ch3_coupling_selected(self: &OscilloObject, mode: &QString);
        #[qinvokable]
        fn ch3_probe_selected(self: &OscilloObject, probe: &QString);

        #[qinvokable]
        fn ch4_enable_changed(self: &OscilloObject, on: bool);
        #[qinvokable]
        fn ch4_scale_changed(self: &OscilloObject, val: f64);
        #[qinvokable]
        fn ch4_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable]
        fn ch4_coupling_selected(self: &OscilloObject, mode: &QString);
        #[qinvokable]
        fn ch4_probe_selected(self: &OscilloObject, probe: &QString);

        #[qinvokable]
        fn start_capture(self: Pin<&mut OscilloObject>);
    }
}

use core::pin::Pin;
use std::net::SocketAddr;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use base64;
use image;
use tokio::runtime::Runtime;
use rigol_cli::utils::parse_source_arg;
use rigol_cli::{io};

#[derive(Default)]
pub struct OscilloObjectRust {
    addr: SocketAddr,
    running: bool,
    current_timebase: f64,
    trigger_source: String,
    avg_enabled: bool,
    scope_image_url: cxx_qt_lib::QString,
}

impl OscilloObjectRust {
    fn send_scpi_sync(&self, cmd: &str) {
        if let Ok(mut s) = std::net::TcpStream::connect_timeout(&self.addr, Duration::from_millis(500)) {
            let _ = s.write_all(format!("{}\n", cmd).as_bytes());
            let _ = s.flush();
        }
    }
    fn query_scpi_sync(&self, cmd: &str) -> Option<String> {
        if let Ok(mut s) = std::net::TcpStream::connect_timeout(&self.addr, Duration::from_millis(500)) {
            let _ = s.write_all(format!("{}\n", cmd).as_bytes());
            let _ = s.flush();
            let mut buf = [0u8; 512];
            if let Ok(n) = s.read(&mut buf) {
                if n > 0 {
                    return Some(String::from_utf8_lossy(&buf[..n]).trim_end_matches(&['\r','\n'][..]).to_string());
                }
            }
        }
        None
    }
}

impl qobject::OscilloObject {
    /// Initialize the object: parse address, set up device initial state.
    pub fn on_construct(&mut self) {
        // Parse instrument address from command-line or default
        let args: Vec<String> = std::env::args().collect();
        let addr_str = match args.get(1) {
            Some(a) if !a.starts_with('-') => {
                if a.contains(':') { a.clone() } else { format!("{}:5555", a) }
            }
            _ => "169.254.50.23:5555".to_string(),
        };
        self.addr = addr_str.parse().unwrap_or_else(|_| "169.254.50.23:5555".parse().unwrap());
        // Default states
        self.running = true;
        self.trigger_source = String::from("CHANnel1");
        // Turn on CH1 display and turn off AWG outputs
        self.send_scpi_sync(":CHAN1:DISP ON");
        self.send_scpi_sync(":OUTPUT1 OFF");
        self.send_scpi_sync(":OUTPUT2 OFF");
        println!("(Init) CH1 display → ON  [{}]", self.addr);
    }

    pub fn info_clicked(&self) {
        println!("ℹ Info");
    }
    pub fn settings_clicked(&self) {
        println!("⚙ Settings");
    }
    pub fn autoscale(&self) {
        self.send_scpi_sync(":AUToscale");
    }
    pub fn console_clicked(&self) {
        println!(">_ Console");
    }
    pub fn save_config(&self) {
        println!("💾 Save config (todo)");
    }
    pub fn load_config(&self) {
        println!("↑ Load config (todo)");
    }
    pub fn toggle_console_log(&self) {
        println!("📄 Console log view toggled (todo)");
    }

    pub fn trigger_source_selected(&self, source: &cxx_qt_lib::QString) {
        if let Ok(p) = parse_source_arg(&source.to_string()) {
            self.trigger_source = p.clone();
            self.send_scpi_sync(":TRIG:MODE EDGE");
            self.send_scpi_sync(&format!(":TRIG:EDGE:SOUR {}", p));
        }
    }
    pub fn trigger_level_changed(&self, level: i32) {
        self.send_scpi_sync(&format!(":TRIG:EDGE:LEV {}", level));
    }
    pub fn trigger_slope_up(&self) {
        self.send_scpi_sync(":TRIG:EDGE:SLOP POS");
    }
    pub fn trigger_slope_down(&self) {
        self.send_scpi_sync(":TRIG:EDGE:SLOP NEG");
    }
    pub fn single_trigger(&self) {
        self.send_scpi_sync(":SING");
    }
    pub fn run_stop(self: Pin<&mut Self>) {
        let this = unsafe { self.get_unchecked_mut() };
        if this.running {
            this.send_scpi_sync(":STOP");
            this.running = false;
        } else {
            this.send_scpi_sync(":RUN");
            this.running = true;
        }
    }

    pub fn timebase_changed(&self, val: f64) {
        // slider 1-100 -> 0.01-1.00 s/div
        let scale = val / 100.0;
        let this = self.rust();
        this.current_timebase = scale;
        self.send_scpi_sync(&format!(":TIM:SCAL {}", scale));
    }
    pub fn time_offset_changed(&self, val: f64) {
        let base = self.rust().current_timebase;
        let offs = base * (val / 50.0);  // ±2 screen widths
        self.send_scpi_sync(&format!(":TIM:OFFS {}", offs));
    }
    pub fn average_toggled(&self, on: bool) {
        if on {
            self.send_scpi_sync(":ACQ:TYPE AVER");
            self.send_scpi_sync(":ACQ:AVER 16");
        } else {
            self.send_scpi_sync(":ACQ:TYPE NORM");
        }
    }

    // ---- CH‑függvények generálása (makróval) -----------------
        chan_handlers!(1,
        ch1_enable_changed, ch1_scale_changed, ch1_offset_changed,
        ch1_coupling_selected, ch1_probe_selected);
        chan_handlers!(2,
        ch2_enable_changed, ch2_scale_changed, ch2_offset_changed,
        ch2_coupling_selected, ch2_probe_selected);
        chan_handlers!(3,
        ch3_enable_changed, ch3_scale_changed, ch3_offset_changed,
        ch3_coupling_selected, ch3_probe_selected);
        chan_handlers!(4,
        ch4_enable_changed, ch4_scale_changed, ch4_offset_changed,
        ch4_coupling_selected, ch4_probe_selected);
    pub fn start_capture(self: Pin<&mut Self>) {
        let qt_thread = self.qt_thread();
        let addr = self.rust().addr;
        thread::spawn(move || {
            loop {
                if let Ok(mut s) = std::net::TcpStream::connect(addr) {
                    let _ = s.write_all(b":DISP:DATA?\n");
                    let _ = s.flush();
                    let mut hdr = [0u8; 2];
                    if s.read_exact(&mut hdr).is_err() || hdr[0] != b'#' {
                        thread::sleep(Duration::from_millis(200));
                        continue;
                    }
                    let nd = (hdr[1] - b'0') as usize;
                    let mut lenbuf = vec![0u8; nd];
                    if s.read_exact(&mut lenbuf).is_err() {
                        thread::sleep(Duration::from_millis(200));
                        continue;
                    }
                    let tot = std::str::from_utf8(&lenbuf).unwrap_or("0").parse::<usize>().unwrap_or(0);
                    let mut img_data = vec![0u8; tot];
                    if s.read_exact(&mut img_data).is_err() {
                        thread::sleep(Duration::from_millis(200));
                        continue;
                    }
                    if let Ok(img) = image::load_from_memory(&img_data) {
                        // Convert image to PNG base64 data URL
                        let mut png_bytes: Vec<u8> = Vec::new();
                        if img.write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageOutputFormat::Png).is_ok() {
                            let base64_data = base64::encode(&png_bytes);
                            let url = format!("data:image/png;base64,{}", base64_data);
                            qt_thread.queue(move |qobject| {
                                qobject.set_scope_image_url(cxx_qt_lib::QString::from(&url));
                            });
                        }
                    }
                }
                thread::sleep(Duration::from_millis(200));
            }
        });
    }
}
