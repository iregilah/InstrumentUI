// src/awg_controller.rs
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
        type AWGController = super::AWGControllerRust;
    }

    extern "RustQt" {
        #[qinvokable]
        fn set_enable1(self: &AWGController, on: bool);
        #[qinvokable]
        fn set_enable2(self: &AWGController, on: bool);
        #[qinvokable]
        fn waveform1(self: &AWGController, wave: &QString);
        #[qinvokable]
        fn waveform2(self: &AWGController, wave: &QString);
        #[qinvokable]
        fn freq1(self: &AWGController, freq: i32);
        #[qinvokable]
        fn freq2(self: &AWGController, freq: i32);
        #[qinvokable]
        fn amp1(self: &AWGController, ampl: f64);
        #[qinvokable]
        fn amp2(self: &AWGController, ampl: f64);
        #[qinvokable]
        fn offset1(self: &AWGController, offset: f64);
        #[qinvokable]
        fn offset2(self: &AWGController, offset: f64);
        #[qinvokable]
        fn load_arb1(self: &AWGController, path: &QString);
        #[qinvokable]
        fn load_arb2(self: &AWGController, path: &QString);
    }
}

use std::net::{SocketAddr, TcpStream};
use std::io::Write;
use std::time::Duration;
use tokio::runtime::Runtime;
use rigol_cli::io::load_arb;

#[derive(Default)]
pub struct AWGControllerRust {
    address: cxx_qt_lib::QString,
}

impl Default for AWGControllerRust {
    fn default() -> Self {
        Self {
            address: cxx_qt_lib::QString::from("169.254.50.23:5555"),
        }
    }
}

impl qobject::AWGController {
    fn set_enable1(&self, on: bool) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let cmd = if on { ":OUTPUT1 ON" } else { ":OUTPUT1 OFF" };
            send_scpi_blocking(&addr, cmd);
        }
    }
    fn set_enable2(&self, on: bool) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            let cmd = if on { ":OUTPUT2 ON" } else { ":OUTPUT2 OFF" };
            send_scpi_blocking(&addr, cmd);
        }
    }

    fn waveform1(&self, wave: &cxx_qt_lib::QString) {
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
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":SOUR1:FUNC {}", scpi_type));
        }
    }
    fn waveform2(&self, wave: &cxx_qt_lib::QString) {
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
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":SOUR2:FUNC {}", scpi_type));
        }
    }

    fn freq1(&self, freq: i32) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":SOUR1:FREQ {}", freq));
        }
    }
    fn freq2(&self, freq: i32) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":SOUR2:FREQ {}", freq));
        }
    }

    fn amp1(&self, ampl: f64) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":SOUR1:VOLT {}", ampl));
        }
    }
    fn amp2(&self, ampl: f64) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":SOUR2:VOLT {}", ampl));
        }
    }

    fn offset1(&self, offset: f64) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":SOUR1:VOLT:OFFS {}", offset));
        }
    }
    fn offset2(&self, offset: f64) {
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            send_scpi_blocking(&addr, &format!(":SOUR2:VOLT:OFFS {}", offset));
        }
    }

    fn load_arb1(&self, path: &cxx_qt_lib::QString) {
        let file = path.to_string();
        if file.is_empty() { return; }
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            std::thread::spawn(move || {
                let rt = Runtime::new().expect("Tokio runtime init");
                if let Err(e) = rt.block_on(load_arb(&addr, 1, &file)) {
                    eprintln!("Arb load1 failed: {}", e);
                } else {
                    send_scpi_blocking(&addr, ":SOUR1:FUNC USER");
                }
            });
        }
    }
    fn load_arb2(&self, path: &cxx_qt_lib::QString) {
        let file = path.to_string();
        if file.is_empty() { return; }
        if let Ok(addr) = self.address().to_string().parse::<SocketAddr>() {
            std::thread::spawn(move || {
                let rt = Runtime::new().expect("Tokio runtime init");
                if let Err(e) = rt.block_on(load_arb(&addr, 2, &file)) {
                    eprintln!("Arb load2 failed: {}", e);
                } else {
                    send_scpi_blocking(&addr, ":SOUR2:FUNC USER");
                }
            });
        }
    }
}

use std::net::SocketAddr;
use std::time::Duration;

// Blocking SCPI helper
fn send_scpi_blocking(addr: &SocketAddr, cmd: &str) {
    if let Ok(mut stream) = TcpStream::connect_timeout(addr, Duration::from_millis(500)) {
        let _ = stream.write_all(format!("{}\n", cmd).as_bytes());
        let _ = stream.flush();
    }
}
