// src/awg_object.rs
use core::pin::Pin;
use std::{io::Write, net::SocketAddr, thread};

use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use rigol_cli::{io, send_scpi};
use tokio::runtime::Runtime;

pub struct AwgObjectRust {
    addr: SocketAddr,
    current_wave_ch1: QString,
    current_wave_ch2: QString,
}

impl Default for AwgObjectRust {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:0".parse().unwrap(),
            current_wave_ch1: QString::from("Sine"),
            current_wave_ch2: QString::from("Sine"),
        }
    }
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
    }
}

impl awg_qobject::AwgObject {
    pub fn on_construct(self: Pin<&mut Self>) {
        let mut this = self;
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
        }
        this.as_mut().set_current_wave_ch1(QString::from("Sine"));
        this.as_mut().set_current_wave_ch2(QString::from("Sine"));
        println!("AwgObject constructed with address: {}", addr_str);
    }

    fn send_scpi(&self, cmd: &str) {
        if let Ok(mut s) = std::net::TcpStream::connect_timeout(
            &self.rust().addr,
            std::time::Duration::from_millis(500),
        ) {
            let _ = s.write_all(format!("{}\n", cmd).as_bytes());
            let _ = s.flush();
        }
    }

    pub fn awg1_enable_changed(&self, on: bool) {
        println!("AWG1 Enable changed: {}", if on { "ON" } else { "OFF" });
        self.send_scpi(&format!(":OUTPUT1 {}", if on { "ON" } else { "OFF" }));
    }
    pub fn awg1_waveform_selected(mut self: Pin<&mut Self>, wave: &QString) {
        let wf = wave.to_string();
        println!("AWG1 waveform selected: {}", wf);
        let mut this = self;
        let wf_upper = wf.to_ascii_uppercase();
        let scpi = match wf_upper.as_str() {
            "SINE" => "SIN",
            "SQUARE" => "SQU",
            "PULSE" => "PULS",
            "RAMP" => "RAMP",
            "NOISE" => "NOIS",
            "ARB" => "USER",
            other => other,
        };
        this.as_ref().send_scpi(&format!(":SOUR1:FUNC {}", scpi));
        this.as_mut().set_current_wave_ch1(QString::from(&wf_upper));
    }
    pub fn awg1_freq_changed(&self, freq: i32) {
        println!("AWG1 frequency changed: {} Hz", freq);
        self.send_scpi(&format!(":SOUR1:FREQ {}", freq));
    }
    pub fn awg1_amp_changed(&self, ampl: f64) {
        println!("AWG1 amplitude changed: {} Vpp", ampl);
        self.send_scpi(&format!(":SOUR1:VOLT {}", ampl));
    }
    pub fn awg1_offset_changed(&self, offset: f64) {
        println!("AWG1 offset changed: {} V", offset);
        self.send_scpi(&format!(":SOUR1:VOLT:OFFS {}", offset));
    }

    pub fn awg1_load_arb(&self, file_path: &QString) {
        let file = file_path.to_string();
        println!("AWG1 load arb waveform from file: {}", file);
        let addr = self.rust().addr;
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            if rt.block_on(io::load_arb(&addr, 1, &file)).is_ok() {
                let _ = send_scpi(&addr, ":SOUR1:FUNC USER");
            } else {
                eprintln!("Arb file load failed for CH1: {}", file);
            }
        });
    }

    pub fn awg2_enable_changed(&self, on: bool) {
        println!("AWG2 Enable changed: {}", if on { "ON" } else { "OFF" });
        self.send_scpi(&format!(":OUTPUT2 {}", if on { "ON" } else { "OFF" }));
    }
    pub fn awg2_waveform_selected(mut self: Pin<&mut Self>, wave: &QString) {
        let wf = wave.to_string();
        println!("AWG2 waveform selected: {}", wf);
        let mut this = self;
        let wf_upper = wf.to_ascii_uppercase();
        let scpi = match wf_upper.as_str() {
            "SINE" => "SIN",
            "SQUARE" => "SQU",
            "PULSE" => "PULS",
            "RAMP" => "RAMP",
            "NOISE" => "NOIS",
            "ARB" => "USER",
            other => other,
        };
        this.as_ref().send_scpi(&format!(":SOUR2:FUNC {}", scpi));
        this.as_mut().set_current_wave_ch2(QString::from(&wf_upper));
    }
    pub fn awg2_freq_changed(&self, freq: i32) {
        println!("AWG2 frequency changed: {} Hz", freq);
        self.send_scpi(&format!(":SOUR2:FREQ {}", freq));
    }
    pub fn awg2_amp_changed(&self, ampl: f64) {
        println!("AWG2 amplitude changed: {} Vpp", ampl);
        self.send_scpi(&format!(":SOUR2:VOLT {}", ampl));
    }
    pub fn awg2_offset_changed(&self, offset: f64) {
        println!("AWG2 offset changed: {} V", offset);
        self.send_scpi(&format!(":SOUR2:VOLT:OFFS {}", offset));
    }

    pub fn awg2_load_arb(&self, file_path: &QString) {
        let file = file_path.to_string();
        println!("AWG2 load arb waveform from file: {}", file);
        let addr = self.rust().addr;
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            if rt.block_on(io::load_arb(&addr, 2, &file)).is_ok() {
                let _ = send_scpi(&addr, ":SOUR2:FUNC USER");
            } else {
                eprintln!("Arb file load failed for CH2: {}", file);
            }
        });
    }
}

