// src/awg_object.rs

use core::pin::Pin;
use std::{
    env,
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    thread,
    time::Duration,
};
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use rigol_cli::{io, send_scpi};
use tokio::runtime::Runtime;
use std::io::Read;

fn parse_addr_str(s: &str) -> Option<SocketAddr> {
    let txt = if s.contains(':') { s.to_string() } else { format!("{s}:5555") };
    txt.parse().ok()
}
fn try_connect_once(addr: SocketAddr, timeout_ms: u64) -> bool {
    println!("[AWG] try_connect_once -> {} ({} ms)", addr, timeout_ms);
    match std::net::TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms)) {
        Ok(mut s) => {
            let _ = s.set_nodelay(true);
            let _ = s.set_read_timeout(Some(Duration::from_millis(timeout_ms)));
            let _ = s.set_write_timeout(Some(Duration::from_millis(timeout_ms)));
            let _ = s.write_all(b"*IDN?\n");
            let _ = s.flush();
            let mut buf = [0u8; 128];
            match s.read(&mut buf) {
                Ok(n) if n > 0 => {
                    println!("[AWG] try_connect_once({addr}) -> OK '{}'", String::from_utf8_lossy(&buf[..n]).trim());
                    true
                }
                Ok(_) => true,
                Err(e) => {
                    println!("[AWG] try_connect_once({addr}) -> read error: {e}");
                    false
                }
            }
        }
        Err(e) => {
            println!("[AWG] try_connect_once({addr}) -> connect error: {e}");
            false
        }
    }
}
fn with_23_24_25_fallback(addr: SocketAddr, timeout_ms: u64) -> SocketAddr {
    println!("[AWG] with_23_24_25_fallback base={}", addr);
    if let IpAddr::V4(ipv4) = addr.ip() {
        let o = ipv4.octets();
        if o[0] == 169 && o[1] == 254 && o[2] == 50 {
            let mut candidates = vec![o[3]];
            for c in [23u8, 24u8, 25u8] {
                if !candidates.contains(&c) {
                    candidates.push(c);
                }
            }
            for last in candidates {
                let cand = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(o[0], o[1], o[2], last)), addr.port());
                if try_connect_once(cand, timeout_ms) {
                    println!("[AWG] fallback pick -> {}", cand);
                    return cand;
                }
            }
        }
    }
    println!("[AWG] fallback not applicable or all failed, using base={}", addr);
    addr
}

#[cxx_qt::bridge]
pub mod awg_qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, current_wave_ch1, cxx_name = "currentWaveCh1")]
        #[qproperty(QString, current_wave_ch2, cxx_name = "currentWaveCh2")]
        #[qproperty(QString, instrument_addr, cxx_name = "instrumentAddr")]
        type AwgObject = super::AwgObjectRust;
    }
    impl cxx_qt::Threading for AwgObject {}

    extern "RustQt" {
        #[qinvokable] fn awg1_enable_changed(self: &AwgObject, on: bool);
        #[qinvokable] fn awg1_waveform_selected(self: Pin<&mut AwgObject>, wave: &QString);
        #[qinvokable] fn awg1_freq_changed(self: &AwgObject, freq: i32);
        #[qinvokable] fn awg1_amp_changed(self: &AwgObject, ampl: f64);
        #[qinvokable] fn awg1_offset_changed(self: &AwgObject, offset: f64);
        #[qinvokable] fn awg1_load_arb(self: &AwgObject, file_path: &QString);

        #[qinvokable] fn awg2_enable_changed(self: &AwgObject, on: bool);
        #[qinvokable] fn awg2_waveform_selected(self: Pin<&mut AwgObject>, wave: &QString);
        #[qinvokable] fn awg2_freq_changed(self: &AwgObject, freq: i32);
        #[qinvokable] fn awg2_amp_changed(self: &AwgObject, ampl: f64);
        #[qinvokable] fn awg2_offset_changed(self: &AwgObject, offset: f64);
        #[qinvokable] fn awg2_load_arb(self: &AwgObject, file_path: &QString);
        #[qinvokable] fn init_from_env(self: Pin<&mut AwgObject>);

    }

}

pub struct AwgObjectRust {
    addr: SocketAddr,
    current_wave_ch1: QString,
    current_wave_ch2: QString,
    instrument_addr: QString,
}

impl Default for AwgObjectRust {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:0".parse().unwrap(),
            current_wave_ch1: QString::from("Sine"),
            current_wave_ch2: QString::from("Sine"),
            instrument_addr: QString::from(""),
        }
    }
}

impl awg_qobject::AwgObject {

    pub fn init_from_env(self: Pin<&mut Self>) {
        let mut this = self;
        let env_addr = env::var("INSTRUMENT_ADDR").ok();
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

        addr = with_23_24_25_fallback(addr, 700);

        unsafe { this.as_mut().rust_mut().get_unchecked_mut() }.addr = addr;
        this.as_mut().set_instrument_addr(QString::from(addr.to_string()));
        println!("[AWG] init_from_env(): INSTRUMENT_ADDR={:?} -> {}", env_addr, addr);
    }

    pub fn on_construct(self: Pin<&mut Self>) {
        let mut this = self;
        let args: Vec<String> = std::env::args().collect();
        let env_addr = env::var("INSTRUMENT_ADDR").ok();
        let cli_addr = match args.get(1) {
            Some(a) if !a.starts_with('-') => Some(a.clone()),
            _ => None,
        };
        println!("[AWG] on_construct: env INSTRUMENT_ADDR={:?}, cli_arg={:?}", env_addr, cli_addr);
        let picked = env_addr
            .as_deref()
            .and_then(parse_addr_str)
            .or_else(|| cli_addr.as_deref().and_then(parse_addr_str))
            .unwrap_or_else(|| "169.254.50.25:5555".parse().unwrap());
        let final_addr = with_23_24_25_fallback(picked, 700);

        unsafe {
            let rust = this.as_mut().rust_mut().get_unchecked_mut();
            rust.addr = final_addr;
            rust.instrument_addr = QString::from(final_addr.to_string());
        }
        this.as_mut().set_current_wave_ch1(QString::from("Sine"));
        this.as_mut().set_current_wave_ch2(QString::from("Sine"));
        this.as_mut().set_instrument_addr(QString::from(final_addr.to_string()));
        println!("[AWG] AwgObject constructed addr={}", final_addr);
    }

    fn send_scpi(&self, cmd: &str) {
        println!("[AWG] send_scpi to {} -> {}", self.rust().addr, cmd);
        if let Ok(mut s) = std::net::TcpStream::connect_timeout(
            &self.rust().addr,
            Duration::from_millis(800),
        ) {
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

    /* AWG Channel 1 */
    pub fn awg1_enable_changed(&self, on: bool) {
        println!("[AWG] ch1 enable -> {}", on);
        self.send_scpi(&format!(":OUTPUT1 {}", if on { "ON" } else { "OFF" }));
    }
    pub fn awg1_waveform_selected(self: Pin<&mut Self>, wave: &QString) {
        let mut this = self;
        let wf = wave.to_string().to_ascii_uppercase();
        let scpi_type = match wf.as_str() {
            "SINE" => "SIN",
            "SQUARE" => "SQU",
            "PULSE" => "PULS",
            "RAMP" => "RAMP",
            "NOISE" => "NOIS",
            "ARB" => "USER",
            other => other,
        };
        println!("[AWG] ch1 waveform -> {} (SCPI {})", wf, scpi_type);
        this.as_ref().send_scpi(&format!(":SOUR1:FUNC {}", scpi_type));
        this.as_mut().set_current_wave_ch1(QString::from(&wf));
    }
    pub fn awg1_freq_changed(&self, freq: i32) {
        println!("[AWG] ch1 freq -> {}", freq);
        self.send_scpi(&format!(":SOUR1:FREQ {}", freq));
    }
    pub fn awg1_amp_changed(&self, ampl: f64) {
        println!("[AWG] ch1 amp -> {}", ampl);
        self.send_scpi(&format!(":SOUR1:VOLT {}", ampl));
    }
    pub fn awg1_offset_changed(&self, offset: f64) {
        println!("[AWG] ch1 offset -> {}", offset);
        self.send_scpi(&format!(":SOUR1:VOLT:OFFS {}", offset));
    }
    pub fn awg1_load_arb(&self, file_path: &QString) {
        let file = file_path.to_string();
        let addr = self.rust().addr;
        println!("[AWG] ch1 load_arb -> {}", file);
        thread::spawn(move || {
            let rt = Runtime::new().expect("Tokio runtime init");
            if rt.block_on(io::load_arb(&addr, 1, &file)).is_err() {
                eprintln!("[AWG] Arb file load failed");
            } else {
                let _ = send_scpi(&addr, ":SOUR1:FUNC USER");
            }
        });
    }

    /* AWG Channel 2 */
    pub fn awg2_enable_changed(&self, on: bool) {
        println!("[AWG] ch2 enable -> {}", on);
        self.send_scpi(&format!(":OUTPUT2 {}", if on { "ON" } else { "OFF" }));
    }
    pub fn awg2_waveform_selected(self: Pin<&mut Self>, wave: &QString) {
        let mut this = self;
        let wf = wave.to_string().to_ascii_uppercase();
        let scpi_type = match wf.as_str() {
            "SINE" => "SIN",
            "SQUARE" => "SQU",
            "PULSE" => "PULS",
            "RAMP" => "RAMP",
            "NOISE" => "NOIS",
            "ARB" => "USER",
            other => other,
        };
        println!("[AWG] ch2 waveform -> {} (SCPI {})", wf, scpi_type);
        this.as_ref().send_scpi(&format!(":SOUR2:FUNC {}", scpi_type));
        this.as_mut().set_current_wave_ch2(QString::from(&wf));
    }
    pub fn awg2_freq_changed(&self, freq: i32) {
        println!("[AWG] ch2 freq -> {}", freq);
        self.send_scpi(&format!(":SOUR2:FREQ {}", freq));
    }
    pub fn awg2_amp_changed(&self, ampl: f64) {
        println!("[AWG] ch2 amp -> {}", ampl);
        self.send_scpi(&format!(":SOUR2:VOLT {}", ampl));
    }
    pub fn awg2_offset_changed(&self, offset: f64) {
        println!("[AWG] ch2 offset -> {}", offset);
        self.send_scpi(&format!(":SOUR2:VOLT:OFFS {}", offset));
    }
    pub fn awg2_load_arb(&self, file_path: &QString) {
        let file = file_path.to_string();
        let addr = self.rust().addr;
        println!("[AWG] ch2 load_arb -> {}", file);
        thread::spawn(move || {
            let rt = Runtime::new().expect("Tokio runtime init");
            if rt.block_on(io::load_arb(&addr, 2, &file)).is_err() {
                eprintln!("[AWG] Arb file load failed");
            } else {
                let _ = send_scpi(&addr, ":SOUR2:FUNC USER");
            }
        });
    }
}
