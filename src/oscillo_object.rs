// src/oscillo_object.rs

use core::pin::Pin;
use std::{
    env,
    fs::File,
    io::Write,
    io::{Read, Write as _},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    thread,
    time::Duration,
};
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use image::{self, ImageFormat};
use rigol_cli::utils::parse_source_arg;

fn parse_addr_str(s: &str) -> Option<SocketAddr> {
    let txt = if s.contains(':') { s.to_string() } else { format!("{s}:5555") };
    txt.parse().ok()
}
fn try_connect_once(addr: SocketAddr, timeout_ms: u64) -> bool {
    println!("[NET] try_connect_once -> {} ({} ms)", addr, timeout_ms);
    match TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms)) {
        Ok(mut s) => {
            let _ = s.set_nodelay(true);
            let _ = s.set_read_timeout(Some(Duration::from_millis(timeout_ms)));
            let _ = s.set_write_timeout(Some(Duration::from_millis(timeout_ms)));
            let _ = s.write_all(b"*IDN?\n");
            let _ = s.flush();
            let mut buf = [0u8; 256];
            match s.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let idn = String::from_utf8_lossy(&buf[..n]).trim().to_owned();
                    println!("[NET] try_connect_once({addr}) -> OK, IDN='{idn}'");
                    true
                }
                Ok(_) => {
                    println!("[NET] try_connect_once({addr}) -> OK, no data");
                    true
                }
                Err(e) => {
                    println!("[NET] try_connect_once({addr}) -> read error: {e}");
                    false
                }
            }
        }
        Err(e) => {
            println!("[NET] try_connect_once({addr}) -> connect error: {e}");
            false
        }
    }
}
fn with_23_24_25_fallback(addr: SocketAddr, timeout_ms: u64) -> SocketAddr {
    println!("[NET] with_23_24_25_fallback base={}", addr);
    if let IpAddr::V4(ipv4) = addr.ip() {
        let o = ipv4.octets();
        if o[0] == 169 && o[1] == 254 && o[2] == 50 {
            // allowed endings we rotate through
            let mut candidates = vec![o[3]];
            for c in [23u8, 24u8, 25u8] {
                if !candidates.contains(&c) {
                    candidates.push(c);
                }
            }
            for last in candidates {
                let cand = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(o[0], o[1], o[2], last)), addr.port());
                if try_connect_once(cand, timeout_ms) {
                    println!("[NET] fallback pick -> {}", cand);
                    return cand;
                }
            }
        }
    }
    println!("[NET] fallback not applicable or all failed, using base={}", addr);
    addr
}

macro_rules! chan_handlers {
    ($idx:literal,
     $on:ident, $sc:ident, $off:ident,
     $coup:ident, $prob:ident) => {
        pub fn $on(&self, on: bool) {
            println!("[OSC] ch{} enable -> {}", $idx, on);
            self.send_scpi_sync(&format!(
                ":CHAN{}:DISP {}",
                $idx,
                if on { "ON" } else { "OFF" }
            ));
        }
        pub fn $sc(&self, val: f64) {
            println!("[OSC] ch{} scale -> {}", $idx, val);
            self.send_scpi_sync(&format!(":CHAN{}:SCAL {}", $idx, val));
        }
        pub fn $off(&self, val: f64) {
            println!("[OSC] ch{} offset -> {}", $idx, val);
            self.send_scpi_sync(&format!(":CHAN{}:OFFS {}", $idx, val));
        }
        pub fn $coup(&self, mode: &QString) {
            let m = mode.to_string();
            println!("[OSC] ch{} coupling -> {}", $idx, m);
            self.send_scpi_sync(&format!(":CHAN{}:COUP {}", $idx, m));
        }
        pub fn $prob(&self, probe: &QString) {
            let ptxt = probe.to_string();
            let factor = if ptxt.starts_with('1') { "1" } else { "10" };
            println!("[OSC] ch{} probe -> {} (factor {})", $idx, ptxt, factor);
            self.send_scpi_sync(&format!(":CHAN{}:PROB {}", $idx, factor));
        }
    };
}

#[cxx_qt::bridge]
pub mod oscillo_qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(bool,    avg_enabled,    cxx_name = "avgEnabled")]
        #[qproperty(QString, scope_image_url, cxx_name = "scopeImageUrl")]
        #[qproperty(bool,    dark_mode,      cxx_name = "darkMode")]
        #[qproperty(QString, current_style,  cxx_name = "currentStyle")]
        #[qproperty(QString, instrument_addr, cxx_name = "instrumentAddr")]
        type OscilloObject = super::OscilloObjectRust;
    }
    impl cxx_qt::Threading for OscilloObject {}

    extern "RustQt" {
        #[qinvokable] fn info_clicked(self: &OscilloObject);
        #[qinvokable] fn settings_clicked(self: &OscilloObject);
        #[qinvokable] fn autoscale(self: &OscilloObject);
        #[qinvokable] fn console_clicked(self: &OscilloObject);
        #[qinvokable] fn save_config(self: &OscilloObject);
        #[qinvokable] fn load_config(self: &OscilloObject);
        #[qinvokable] fn toggle_console_log(self: &OscilloObject);

        #[qinvokable] fn trigger_source_selected(self: Pin<&mut OscilloObject>, source: &QString);
        #[qinvokable] fn trigger_level_changed(self: &OscilloObject, level: i32);
        #[qinvokable] fn trigger_slope_up(self: &OscilloObject);
        #[qinvokable] fn trigger_slope_down(self: &OscilloObject);
        #[qinvokable] fn single_trigger(self: &OscilloObject);

        #[qinvokable] fn run_stop(self: Pin<&mut OscilloObject>);
        #[qinvokable] fn timebase_changed(self: Pin<&mut OscilloObject>, val: f64);
        #[qinvokable] fn time_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn average_toggled(self: Pin<&mut OscilloObject>, on: bool);

        // CH1
        #[qinvokable] fn ch1_enable_changed(self: &OscilloObject, on: bool);
        #[qinvokable] fn ch1_scale_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn ch1_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn ch1_coupling_selected(self: &OscilloObject, mode: &QString);
        #[qinvokable] fn ch1_probe_selected(self: &OscilloObject, probe: &QString);
        // CH2
        #[qinvokable] fn ch2_enable_changed(self: &OscilloObject, on: bool);
        #[qinvokable] fn ch2_scale_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn ch2_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn ch2_coupling_selected(self: &OscilloObject, mode: &QString);
        #[qinvokable] fn ch2_probe_selected(self: &OscilloObject, probe: &QString);
        // CH3
        #[qinvokable] fn ch3_enable_changed(self: &OscilloObject, on: bool);
        #[qinvokable] fn ch3_scale_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn ch3_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn ch3_coupling_selected(self: &OscilloObject, mode: &QString);
        #[qinvokable] fn ch3_probe_selected(self: &OscilloObject, probe: &QString);
        // CH4
        #[qinvokable] fn ch4_enable_changed(self: &OscilloObject, on: bool);
        #[qinvokable] fn ch4_scale_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn ch4_offset_changed(self: &OscilloObject, val: f64);
        #[qinvokable] fn ch4_coupling_selected(self: &OscilloObject, mode: &QString);
        #[qinvokable] fn ch4_probe_selected(self: &OscilloObject, probe: &QString);

        #[qinvokable] fn start_capture(self: Pin<&mut OscilloObject>);
        #[qinvokable] fn set_style(self: &OscilloObject, style: &QString);
        #[qinvokable] fn init_from_env(self: Pin<&mut OscilloObject>);

    }
}

pub struct OscilloObjectRust {
    addr:            SocketAddr,
    running:         bool,
    current_timebase: f64,
    trigger_source:  String,
    avg_enabled:     bool,
    scope_image_url: QString,
    dark_mode:       bool,
    current_style:   QString,
    instrument_addr: QString,
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
            dark_mode:        true,
            current_style:    QString::from(""),
            instrument_addr:  QString::from(""),
        }
    }
}

impl OscilloObjectRust {
    fn send_scpi_sync(&self, cmd: &str) {
        println!("[OSC] send_scpi_sync to {} -> {}", self.addr, cmd);
        if let Ok(mut s) = std::net::TcpStream::connect_timeout(&self.addr, Duration::from_millis(800)) {
            let _ = s.set_nodelay(true);
            let _ = s.set_read_timeout(Some(Duration::from_millis(800)));
            let _ = s.set_write_timeout(Some(Duration::from_millis(800)));
            let result = s.write_all(format!("{}\n", cmd).as_bytes());
            let flush_res = s.flush();
            if result.is_ok() && flush_res.is_ok() {
                println!("SCPI> {} ✓", cmd);
            } else {
                println!("SCPI> {} ✗ (send/flush error)", cmd);
            }
        } else {
            println!("SCPI> {} (connection failed)", cmd);
        }
    }
}

impl oscillo_qobject::OscilloObject {

    pub fn init_from_env(self: Pin<&mut Self>) {
        let mut this = self;
        let env_addr = env::var("INSTRUMENT_ADDR").ok();
        // próbáljuk sorban: INSTRUMENT_ADDR -> RIGOL_ADDR -> OSCILLOSCOPE_IP:5555 -> default
        let mut addr = env_addr
            .as_deref()
            .and_then(|s| s.parse::<SocketAddr>().ok())
            .or_else(|| env::var("RIGOL_ADDR").ok()?.parse().ok())
            .or_else(|| {
                env::var("OSCILLOSCOPE_IP").ok().and_then(|ip| {
                    ip.parse::<IpAddr>().ok().map(|ip| SocketAddr::new(ip, 5555))
                })
            })
            .unwrap_or_else(|| "169.254.50.25:5555".parse().unwrap());

        // (opcionális) 23/24 fallback próba
        addr = with_23_24_25_fallback(addr, 700);

        unsafe { this.as_mut().rust_mut().get_unchecked_mut() }.addr = addr;
        this.as_mut().set_instrument_addr(QString::from(addr.to_string()));
        println!("[OSC] init_from_env(): INSTRUMENT_ADDR={:?} -> {}", env_addr, addr);
    }

    fn send_scpi_sync(&self, cmd: &str) {
        self.rust().send_scpi_sync(cmd);
    }

    pub fn on_construct(self: Pin<&mut Self>) {
        let mut this = self;
        let args: Vec<String> = std::env::args().collect();
        let env_addr = env::var("INSTRUMENT_ADDR").ok();
        let cli_addr = match args.get(1) {
            Some(a) if !a.starts_with('-') => Some(a.clone()),
            _ => None,
        };
        println!("[OSC] on_construct: env INSTRUMENT_ADDR={:?}, cli_arg={:?}", env_addr, cli_addr);
        let picked = env_addr
            .as_deref()
            .and_then(parse_addr_str)
            .or_else(|| cli_addr.as_deref().and_then(parse_addr_str))
            .unwrap_or_else(|| "169.254.50.25:5555".parse().unwrap());
        let final_addr = with_23_24_25_fallback(picked, 700);
        {
            let rust = unsafe { this.as_mut().rust_mut().get_unchecked_mut() };
            rust.addr = final_addr;
            rust.running = true;
            rust.trigger_source = "CHANnel1".into();
            rust.instrument_addr = QString::from(final_addr.to_string());
        }
        this.as_mut().set_instrument_addr(QString::from(final_addr.to_string()));

        let style_env = std::env::var("QT_QUICK_CONTROLS_STYLE").unwrap_or_else(|_| "".to_string());
        this.as_mut().set_current_style(QString::from(style_env.trim()));
        println!("[OSC] OscilloObject constructed addr={}", final_addr);

        this.as_ref().send_scpi_sync(":CHAN1:DISP ON");
        this.as_ref().send_scpi_sync(":OUTPUT1 OFF");
        this.as_ref().send_scpi_sync(":OUTPUT2 OFF");
        println!("[OSC] Initial SCPI commands sent");
    }

    /* Toolbar buttons */
    pub fn info_clicked(&self) {
        println!("ℹ info");
    }
    pub fn settings_clicked(&self) {
        println!("⚙ settings");
    }
    pub fn autoscale(&self) {
        println!("[OSC] autoscale()");
        self.send_scpi_sync(":AUToscale");
    }
    pub fn console_clicked(&self) {
        println!(">_ console");
    }
    pub fn save_config(&self) {
        println!("💾 save (todo)");
    }
    pub fn load_config(&self) {
        println!("↑ load (todo)");
    }
    pub fn toggle_console_log(&self) {
        println!("📄 toggle log");
    }

    /* Trigger controls */
    pub fn trigger_source_selected(self: Pin<&mut Self>, source: &QString) {
        let mut this = self;
        let src_txt = source.to_string();
        println!("[OSC] trigger_source_selected -> {}", src_txt);
        if let Ok(ch) = parse_source_arg(&src_txt) {
            unsafe { this.as_mut().rust_mut().get_unchecked_mut() }.trigger_source = ch.clone();
            this.as_ref().send_scpi_sync(":TRIG:MODE EDGE");
            this.as_ref().send_scpi_sync(&format!(":TRIG:EDGE:SOUR {}", ch));
        } else {
            println!("[OSC] trigger_source_selected parse failed for '{}'", src_txt);
        }
    }
    pub fn trigger_level_changed(&self, level: i32) {
        println!("[OSC] trigger_level_changed -> {}", level);
        self.send_scpi_sync(&format!(":TRIG:EDGE:LEV {}", level));
    }
    pub fn trigger_slope_up(&self) {
        println!("[OSC] trigger_slope_up()");
        self.send_scpi_sync(":TRIG:EDGE:SLOP POS");
    }
    pub fn trigger_slope_down(&self) {
        println!("[OSC] trigger_slope_down()");
        self.send_scpi_sync(":TRIG:EDGE:SLOP NEG");
    }
    pub fn single_trigger(&self) {
        println!("[OSC] single_trigger()");
        self.send_scpi_sync(":SING");
    }
    pub fn run_stop(self: Pin<&mut Self>) {
        let mut this = self;
        let running_now = !this.rust().running;
        unsafe { this.as_mut().rust_mut().get_unchecked_mut() }.running = running_now;
        println!("[OSC] run_stop() -> {}", if running_now { "RUN" } else { "STOP" });
        this.as_ref().send_scpi_sync(if running_now { ":RUN" } else { ":STOP" });
    }

    /* Timebase and offset */
    pub fn timebase_changed(self: Pin<&mut Self>, val: f64) {
        let mut this = self;
        let scale = val / 100.0;
        unsafe { this.as_mut().rust_mut().get_unchecked_mut() }.current_timebase = scale;
        println!("[OSC] timebase_changed -> raw:{}, scale:{}", val, scale);
        this.as_ref().send_scpi_sync(&format!(":TIM:SCAL {}", scale));
    }
    pub fn time_offset_changed(&self, val: f64) {
        let base = self.rust().current_timebase;
        let offs = base * (val / 50.0);
        println!("[OSC] time_offset_changed -> raw:{}, base:{}, offs:{}", val, base, offs);
        self.send_scpi_sync(&format!(":TIM:OFFS {}", offs));
    }

    pub fn average_toggled(self: Pin<&mut Self>, on: bool) {
        let mut this = self;
        println!("[OSC] average_toggled -> {}", on);
        if on {
            this.as_ref().send_scpi_sync(":ACQ:TYPE AVER");
            this.as_ref().send_scpi_sync(":ACQ:AVER 16");
        } else {
            this.as_ref().send_scpi_sync(":ACQ:TYPE NORM");
        }
        this.as_mut().set_avg_enabled(on);
        println!("Averaging {}", if on { "enabled (16x)" } else { "disabled" });
    }

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
        let qt_thread = self.as_ref().get_ref().qt_thread();
        let addr = self.rust().addr;
        println!("[CAP] start_capture() using addr {}", addr);
        thread::spawn(move || loop {
            println!("[CAP] loop tick; connecting {}", addr);
            match std::net::TcpStream::connect_timeout(&addr, Duration::from_millis(800)) {
                Ok(mut s) => {
                    let _ = s.set_read_timeout(Some(Duration::from_millis(1500)));
                    let _ = s.set_write_timeout(Some(Duration::from_millis(800)));
                    println!("[CAP] connected {}", addr);
                    if s.write_all(b":DISP:DATA?\n").is_err() {
                        println!("[CAP] write ':DISP:DATA?' failed");
                        thread::sleep(Duration::from_millis(200));
                        continue;
                    }
                    if s.flush().is_err() {
                        println!("[CAP] flush failed");
                        thread::sleep(Duration::from_millis(200));
                        continue;
                    }
                    let mut hdr = [0u8; 2];
                    if s.read_exact(&mut hdr).is_err() || hdr[0] != b'#' {
                        println!("[CAP] invalid/absent header, hdr={:?}", hdr);
                        thread::sleep(Duration::from_millis(200));
                        continue;
                    }
                    let nd = (hdr[1] - b'0') as usize;
                    println!("[CAP] header ok, ndigits={}", nd);
                    let mut lenbuf = vec![0u8; nd];
                    if s.read_exact(&mut lenbuf).is_err() {
                        println!("[CAP] reading len digits failed");
                        thread::sleep(Duration::from_millis(200));
                        continue;
                    }
                    let tot = std::str::from_utf8(&lenbuf)
                        .unwrap_or("0")
                        .parse::<usize>()
                        .unwrap_or(0);
                    println!("[CAP] expecting {} bytes image", tot);
                    if tot == 0 {
                        thread::sleep(Duration::from_millis(200));
                        continue;
                    }
                    let mut img_data = vec![0u8; tot];
                    if s.read_exact(&mut img_data).is_err() {
                        println!("[CAP] read_exact(payload) failed");
                        thread::sleep(Duration::from_millis(200));
                        continue;
                    }
                    if let Ok(img) = image::load_from_memory(&img_data) {
                        let mut png = Vec::new();
                        if img.write_to(&mut std::io::Cursor::new(&mut png), ImageFormat::Png).is_ok() {
                            let url = format!("data:image/png;base64,{}", STANDARD.encode(&png));
                            let _ = qt_thread.queue(move |qobj| {
                                qobj.set_scope_image_url(QString::from(&url));
                            });
                            println!("[CAP] image updated ({} bytes png)", png.len());
                        } else {
                            println!("[CAP] image write_to(PNG) failed");
                        }
                    } else {
                        println!("[CAP] image decode failed ({} bytes)", img_data.len());
                    }
                }
                Err(e) => {
                    println!("[CAP] connect {} failed: {}", addr, e);
                }
            }
            thread::sleep(Duration::from_millis(300));
        });
    }

    pub fn set_style(&self, style: &QString) {
        let style_str = style.to_string();
        if let Ok(mut file) = File::create("style.conf") {
            let _ = writeln!(file, "{}", style_str);
        }
        println!("Style set to '{}' (will apply on next restart)", style_str);
    }
}
