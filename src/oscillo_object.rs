// src/oscillo_object.rs
use core::pin::Pin;
use std::{
    io::{Read, Write},
    net::SocketAddr,
    thread,
    time::Duration,
};

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use image::{self, ImageFormat};
use rigol_cli::utils::parse_source_arg;


/* ---------- makró a csatorna‑függvényekhez ---------- */
macro_rules! chan_handlers {
    ($idx:literal,
     $on:ident, $sc:ident, $off:ident,
     $coup:ident, $prob:ident) => {
        pub fn $on(&self, on: bool) {
            self.send_scpi_sync(&format!(
                ":CHAN{}:DISP {}",
                $idx,
                if on { "ON" } else { "OFF" }
            ));
        }
        pub fn $sc(&self, val: f64)  { self.send_scpi_sync(&format!(":CHAN{}:SCAL {}", $idx, val)); }
        pub fn $off(&self, val: f64) { self.send_scpi_sync(&format!(":CHAN{}:OFFS {}", $idx, val)); }
        pub fn $coup(&self, mode: &QString) {
            self.send_scpi_sync(&format!(":CHAN{}:COUP {}", $idx, mode));
        }
        pub fn $prob(&self, probe: &QString) {
            let factor = if probe.to_string().starts_with('1') { "1" } else { "10" };
            self.send_scpi_sync(&format!(":CHAN{}:PROB {}", $idx, factor));
        }
    };
}

/* ---------- cxx‑qt híd ---------- */
#[cxx_qt::bridge]
pub mod oscillo_qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

        extern "RustQt" {
            #[qobject]
            #[qml_element]
            #[qproperty(bool,    avg_enabled,     cxx_name = "avgEnabled")]
            #[qproperty(QString, scope_image_url, cxx_name = "scopeImageUrl")]
            type OscilloObject = super::OscilloObjectRust;
        }
        // --- Threading trait implementáció a qt_thread() metódushoz ---
        impl cxx_qt::Threading for OscilloObject {}

    extern "RustQt" {
        /* gombok */
        #[qinvokable] fn info_clicked(self: &OscilloObject);
        #[qinvokable] fn settings_clicked(self: &OscilloObject);
        #[qinvokable] fn autoscale(self: &OscilloObject);
        #[qinvokable] fn console_clicked(self: &OscilloObject);
        #[qinvokable] fn save_config(self: &OscilloObject);
        #[qinvokable] fn load_config(self: &OscilloObject);
        #[qinvokable] fn toggle_console_log(self: &OscilloObject);

        /* trigger */
        #[qinvokable] fn trigger_source_selected(self: Pin<&mut OscilloObject>, source: &QString);
        #[qinvokable] fn trigger_level_changed(self: &OscilloObject, level: i32);
        #[qinvokable] fn trigger_slope_up(self: &OscilloObject);
        #[qinvokable] fn trigger_slope_down(self: &OscilloObject);
        #[qinvokable] fn single_trigger(self: &OscilloObject);
        #[qinvokable] fn run_stop(self: Pin<&mut OscilloObject>);

        /* időalap / átlag */
        #[qinvokable] fn timebase_changed(self: Pin<&mut OscilloObject>, val: f64);
        #[qinvokable] fn time_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn average_toggled(self: Pin<&mut OscilloObject>, on: bool);

        /* csatornák */
        #[qinvokable] fn ch1_enable_changed(self: &OscilloObject, on: bool);
        #[qinvokable] fn ch1_scale_changed (self: &OscilloObject, val: f64);
        #[qinvokable] fn ch1_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn ch1_coupling_selected(self: &OscilloObject, mode: &QString);
        #[qinvokable] fn ch1_probe_selected   (self: &OscilloObject, probe: &QString);

        #[qinvokable] fn ch2_enable_changed(self: &OscilloObject, on: bool);
        #[qinvokable] fn ch2_scale_changed (self: &OscilloObject, val: f64);
        #[qinvokable] fn ch2_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn ch2_coupling_selected(self: &OscilloObject, mode: &QString);
        #[qinvokable] fn ch2_probe_selected   (self: &OscilloObject, probe: &QString);

        #[qinvokable] fn ch3_enable_changed(self: &OscilloObject, on: bool);
        #[qinvokable] fn ch3_scale_changed (self: &OscilloObject, val: f64);
        #[qinvokable] fn ch3_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn ch3_coupling_selected(self: &OscilloObject, mode: &QString);
        #[qinvokable] fn ch3_probe_selected   (self: &OscilloObject, probe: &QString);

        #[qinvokable] fn ch4_enable_changed(self: &OscilloObject, on: bool);
        #[qinvokable] fn ch4_scale_changed (self: &OscilloObject, val: f64);
        #[qinvokable] fn ch4_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn ch4_coupling_selected(self: &OscilloObject, mode: &QString);
        #[qinvokable] fn ch4_probe_selected   (self: &OscilloObject, probe: &QString);

        /* képletöltés */
        #[qinvokable] fn start_capture(self: Pin<&mut OscilloObject>);
    }
}

/* ---------- belső Rust‑struktúra ---------- */
pub struct OscilloObjectRust {
    addr:            SocketAddr,
    running:         bool,
    current_timebase: f64,
    trigger_source:  String,
    avg_enabled:     bool,
    scope_image_url: QString,
}

impl Default for OscilloObjectRust {
    fn default() -> Self {
        Self {
            addr:             "0.0.0.0:0".parse().unwrap(),
            running:          false,
            current_timebase: 0.01,
            trigger_source:   "CHANnel1".into(),
            avg_enabled:      false,
            scope_image_url:  QString::from(""),
        }
    }
}

impl OscilloObjectRust {
    fn send_scpi_sync(&self, cmd: &str) {
        if let Ok(mut s) =
            std::net::TcpStream::connect_timeout(&self.addr, Duration::from_millis(500))
        {
            let _ = s.write_all(format!("{}\n", cmd).as_bytes());
            let _ = s.flush();
        }
    }
}

/* ---------- QObject‑implementáció ---------- */
impl oscillo_qobject::OscilloObject {
    fn send_scpi_sync(&self, cmd: &str) { self.rust().send_scpi_sync(cmd); }

    /* életciklus */
    pub fn on_construct(self: Pin<&mut Self>) {
        let mut this = self;                                   // mutábilis binden
        let args: Vec<String> = std::env::args().collect();
        let addr_str = match args.get(1) {
            Some(a) if !a.starts_with('-') => {
                if a.contains(':') { a.clone() } else { format!("{}:5555", a) }
            }
            _ => "169.254.50.23:5555".into(),
        };

        {
            let rust = unsafe { this.as_mut().rust_mut().get_unchecked_mut() };
            rust.addr = addr_str.parse().unwrap();
            rust.running = true;
            rust.trigger_source = "CHANnel1".into();
        }

        this.as_ref().send_scpi_sync(":CHAN1:DISP ON");
        this.as_ref().send_scpi_sync(":OUTPUT1 OFF");
        this.as_ref().send_scpi_sync(":OUTPUT2 OFF");
    }

    /* gomb‑callbackek */
    pub fn info_clicked(&self)      { println!("ℹ info"); }
    pub fn settings_clicked(&self)  { println!("⚙ settings"); }
    pub fn autoscale(&self)         { self.send_scpi_sync(":AUToscale"); }
    pub fn console_clicked(&self)   { println!(">_ console"); }
    pub fn save_config(&self)       { println!("💾 save (todo)"); }
    pub fn load_config(&self)       { println!("↑ load (todo)"); }
    pub fn toggle_console_log(&self){ println!("📄 toggle log"); }

    /* trigger */
    pub fn trigger_source_selected(self: Pin<&mut Self>, source: &QString) {
        let mut this = self;
        if let Ok(ch) = parse_source_arg(&source.to_string()) {
            unsafe { this.as_mut().rust_mut().get_unchecked_mut() }.trigger_source = ch.clone();
            this.as_ref().send_scpi_sync(":TRIG:MODE EDGE");
            this.as_ref().send_scpi_sync(&format!(":TRIG:EDGE:SOUR {}", ch));
        }
    }
    pub fn trigger_level_changed(&self, level: i32) { self.send_scpi_sync(&format!(":TRIG:EDGE:LEV {}", level)); }
    pub fn trigger_slope_up(&self)                  { self.send_scpi_sync(":TRIG:EDGE:SLOP POS"); }
    pub fn trigger_slope_down(&self)                { self.send_scpi_sync(":TRIG:EDGE:SLOP NEG"); }
    pub fn single_trigger(&self)                    { self.send_scpi_sync(":SING"); }

    pub fn run_stop(self: Pin<&mut Self>) {
        let mut this = self;
        let running_now = !this.rust().running;
        unsafe { this.as_mut().rust_mut().get_unchecked_mut() }.running = running_now;
        this.as_ref().send_scpi_sync(if running_now { ":RUN" } else { ":STOP" });
    }

    /* timebase / offset */
    pub fn timebase_changed(self: Pin<&mut Self>, val: f64) {
        let mut this = self;
        let scale = val / 100.0;
        unsafe { this.as_mut().rust_mut().get_unchecked_mut() }.current_timebase = scale;
        this.as_ref().send_scpi_sync(&format!(":TIM:SCAL {}", scale));
    }
    pub fn time_offset_changed(&self, val: f64) {
        let base = self.rust().current_timebase;
        let offs = base * (val / 50.0);
        self.send_scpi_sync(&format!(":TIM:OFFS {}", offs));
    }

    pub fn average_toggled(self: Pin<&mut Self>, on: bool) {
        let mut this = self;
        if on {
            this.as_ref().send_scpi_sync(":ACQ:TYPE AVER");
            this.as_ref().send_scpi_sync(":ACQ:AVER 16");
        } else {
            this.as_ref().send_scpi_sync(":ACQ:TYPE NORM");
        }
        this.as_mut().set_avg_enabled(on);
    }

    /* csatorna‑makrók */
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

    /* folyamatos képletöltés */
    pub fn start_capture(self: Pin<&mut Self>) {
        let qt_thread = self.as_ref().get_ref().qt_thread();          // elérhető a Threading trait miatt
        let addr      = self.rust().addr;

        thread::spawn(move || loop {
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
                let tot = std::str::from_utf8(&lenbuf)
                    .unwrap_or("0")
                    .parse::<usize>()
                    .unwrap_or(0);

                let mut img_data = vec![0u8; tot];
                if s.read_exact(&mut img_data).is_err() {
                    thread::sleep(Duration::from_millis(200));
                    continue;
                }

                if let Ok(img) = image::load_from_memory(&img_data) {
                    let mut png = Vec::new();
                    if img.write_to(&mut std::io::Cursor::new(&mut png), ImageFormat::Png).is_ok() {
                        let url = format!("data:image/png;base64,{}", STANDARD.encode(&png));
                        qt_thread.queue(move |qobj| {
                            qobj.set_scope_image_url(QString::from(&url));
                        });
                    }
                }
            }
            thread::sleep(Duration::from_millis(200));
        });
    }
}
