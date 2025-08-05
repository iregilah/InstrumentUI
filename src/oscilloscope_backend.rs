// src/oscilloscope_backend.rs

#[cxx_qt::bridge]
pub mod oscilloscope {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, scopeImage)]
                #[qproperty(bool,   avgEnabled)]
        type OscilloscopeBackend = super::OscilloscopeBackendRust;
    }

    extern "RustQt" {
        #[qinvokable]
        #[cxx_name = "infoClicked"]
        fn info_clicked(&self);

        #[qinvokable]
        #[cxx_name = "settingsClicked"]
        fn settings_clicked(&self);

        #[qinvokable]
        #[cxx_name = "consoleClicked"]
        fn console_clicked(&self);

        #[qinvokable]
        #[cxx_name = "autoscaleClicked"]
        fn autoscale_clicked(&self);

        #[qinvokable]
        #[cxx_name = "saveConfigClicked"]
        fn save_config_clicked(&self);

        #[qinvokable]
        #[cxx_name = "loadConfigClicked"]
        fn load_config_clicked(&self);

        #[qinvokable]
        #[cxx_name = "triggerSourceSelected"]
        fn trigger_source_selected(&self, source: &QString);

        #[qinvokable]
        #[cxx_name = "triggerLevelChanged"]
        fn trigger_level_changed(&self, level: i32);

        #[qinvokable]
        #[cxx_name = "triggerSlopeUp"]
        fn trigger_slope_up(&self);

        #[qinvokable]
        #[cxx_name = "triggerSlopeDown"]
        fn trigger_slope_down(&self);

        #[qinvokable]
        #[cxx_name = "singleTrigger"]
        fn single_trigger(&self);

        #[qinvokable]
        #[cxx_name = "runStop"]
        fn run_stop(&mut self);

        #[qinvokable]
        #[cxx_name = "timebaseChanged"]
        fn timebase_changed(&mut self, value: i32);

        #[qinvokable]
        #[cxx_name = "timeOffsetChanged"]
        fn time_offset_changed(&self, value: i32);

        #[qinvokable]
        #[cxx_name = "averageToggled"]
        fn average_toggled(&self, on: bool);

        #[qinvokable]
        #[cxx_name = "ch1EnableChanged"]
        fn ch1_enable_changed(&mut self, on: bool);
        #[qinvokable]
        #[cxx_name = "ch2EnableChanged"]
        fn ch2_enable_changed(&mut self, on: bool);
        #[qinvokable]
        #[cxx_name = "ch3EnableChanged"]
        fn ch3_enable_changed(&mut self, on: bool);
        #[qinvokable]
        #[cxx_name = "ch4EnableChanged"]
        fn ch4_enable_changed(&mut self, on: bool);

        #[qinvokable]
        #[cxx_name = "ch1ScaleChanged"]
        fn ch1_scale_changed(&self, value: i32);
        #[qinvokable]
        #[cxx_name = "ch2ScaleChanged"]
        fn ch2_scale_changed(&self, value: i32);
        #[qinvokable]
        #[cxx_name = "ch3ScaleChanged"]
        fn ch3_scale_changed(&self, value: i32);
        #[qinvokable]
        #[cxx_name = "ch4ScaleChanged"]
        fn ch4_scale_changed(&self, value: i32);

        #[qinvokable]
        #[cxx_name = "ch1OffsetChanged"]
        fn ch1_offset_changed(&self, value: i32);
        #[qinvokable]
        #[cxx_name = "ch2OffsetChanged"]
        fn ch2_offset_changed(&self, value: i32);
        #[qinvokable]
        #[cxx_name = "ch3OffsetChanged"]
        fn ch3_offset_changed(&self, value: i32);
        #[qinvokable]
        #[cxx_name = "ch4OffsetChanged"]
        fn ch4_offset_changed(&self, value: i32);

        #[qinvokable]
        #[cxx_name = "ch1CouplingSelected"]
        fn ch1_coupling_selected(&self, mode: &QString);
        #[qinvokable]
        #[cxx_name = "ch2CouplingSelected"]
        fn ch2_coupling_selected(&self, mode: &QString);
        #[qinvokable]
        #[cxx_name = "ch3CouplingSelected"]
        fn ch3_coupling_selected(&self, mode: &QString);
        #[qinvokable]
        #[cxx_name = "ch4CouplingSelected"]
        fn ch4_coupling_selected(&self, mode: &QString);

        #[qinvokable]
        #[cxx_name = "ch1ProbeSelected"]
        fn ch1_probe_selected(&self, probe: &QString);
        #[qinvokable]
        #[cxx_name = "ch2ProbeSelected"]
        fn ch2_probe_selected(&self, probe: &QString);
        #[qinvokable]
        #[cxx_name = "ch3ProbeSelected"]
        fn ch3_probe_selected(&self, probe: &QString);
        #[qinvokable]
        #[cxx_name = "ch4ProbeSelected"]
        fn ch4_probe_selected(&self, probe: &QString);

        #[qproperty(QString, scopeImage)]
        #[qproperty(bool, avgEnabled)]
    }
}

use std::pin::Pin;
use std::sync::Mutex;
use std::time::Duration;
use std::io::{Write, Read};
use std::net::TcpStream;
use base64;
use rigol_cli::utils::parse_source_arg;
use crate::ADDR;

#[derive(Default)]
pub struct OscilloscopeBackendRust {
    running: bool,
    current_timebase: f64,
    ch1_on: bool,
    ch2_on: bool,
    ch3_on: bool,
    ch4_on: bool,
}

impl cxx_qt::Threading for oscilloscope::qobject::OscilloscopeBackend {}

impl oscilloscope::qobject::OscilloscopeBackend {
    pub fn info_clicked(&self) {
        println!("ℹ Info");
    }
    pub fn settings_clicked(&self) {
        println!("⚙ Settings");
    }
    pub fn console_clicked(&self) {
        println!(">_ Console");
    }
    pub fn autoscale_clicked(&self) {
        send_scpi(":AUTOSCALE");
    }
    pub fn save_config_clicked(&self) {
        println!("💾 Save config (todo)");
    }
    pub fn load_config_clicked(&self) {
        println!("↑ Load config (todo)");
    }
    pub fn trigger_source_selected(&self, source: &cxx_qt_lib::QString) {
        let src_str = source.to_string();
        if let Ok(parsed) = parse_source_arg(&src_str) {
            send_scpi(":TRIG:MODE EDGE");
            send_scpi(&format!(":TRIG:EDGE:SOUR {}", parsed));
        }
    }
    pub fn trigger_level_changed(&self, level: i32) {
        send_scpi(&format!(":TRIG:EDGE:LEV {}", level));
    }
    pub fn trigger_slope_up(&self) {
        send_scpi(":TRIG:EDGE:SLOP POS");
    }
    pub fn trigger_slope_down(&self) {
        send_scpi(":TRIG:EDGE:SLOP NEG");
    }
    pub fn single_trigger(&self) {
        send_scpi(":SING");
    }
    pub fn run_stop(&mut self) {
        if self.running {
            send_scpi(":STOP");
            self.running = false;
        } else {
            send_scpi(":RUN");
            self.running = true;
        }
    }
    pub fn timebase_changed(&mut self, value: i32) {
        let scale = value as f64 / 100.0;
        self.current_timebase = scale;
        send_scpi(&format!(":TIM:SCAL {}", scale));
    }
    pub fn time_offset_changed(&self, value: i32) {
        let offs = self.current_timebase * (value as f64 / 50.0);
        send_scpi(&format!(":TIM:OFFS {}", offs));
    }
    pub fn average_toggled(&self, on: bool) {
        if on {
            send_scpi(":ACQ:TYPE AVER");
            send_scpi(":ACQ:AVER 16");
        } else {
            send_scpi(":ACQ:TYPE NORM");
        }
    }
    pub fn ch1_enable_changed(&mut self, on: bool) {
        send_scpi(if on { ":CHAN1:DISP ON" } else { ":CHAN1:DISP OFF" });
        self.ch1_on = on;
    }
    pub fn ch2_enable_changed(&mut self, on: bool) {
        send_scpi(if on { ":CHAN2:DISP ON" } else { ":CHAN2:DISP OFF" });
        self.ch2_on = on;
    }
    pub fn ch3_enable_changed(&mut self, on: bool) {
        send_scpi(if on { ":CHAN3:DISP ON" } else { ":CHAN3:DISP OFF" });
        self.ch3_on = on;
    }
    pub fn ch4_enable_changed(&mut self, on: bool) {
        send_scpi(if on { ":CHAN4:DISP ON" } else { ":CHAN4:DISP OFF" });
        self.ch4_on = on;
    }
    pub fn ch1_scale_changed(&self, value: i32) {
        send_scpi(&format!(":CHAN1:SCAL {}", value));
    }
    pub fn ch2_scale_changed(&self, value: i32) {
        send_scpi(&format!(":CHAN2:SCAL {}", value));
    }
    pub fn ch3_scale_changed(&self, value: i32) {
        send_scpi(&format!(":CHAN3:SCAL {}", value));
    }
    pub fn ch4_scale_changed(&self, value: i32) {
        send_scpi(&format!(":CHAN4:SCAL {}", value));
    }
    pub fn ch1_offset_changed(&self, value: i32) {
        send_scpi(&format!(":CHAN1:OFFS {}", value));
    }
    pub fn ch2_offset_changed(&self, value: i32) {
        send_scpi(&format!(":CHAN2:OFFS {}", value));
    }
    pub fn ch3_offset_changed(&self, value: i32) {
        send_scpi(&format!(":CHAN3:OFFS {}", value));
    }
    pub fn ch4_offset_changed(&self, value: i32) {
        send_scpi(&format!(":CHAN4:OFFS {}", value));
    }
    pub fn ch1_coupling_selected(&self, mode: &cxx_qt_lib::QString) {
        send_scpi(&format!(":CHAN1:COUP {}", mode.to_string()));
    }
    pub fn ch2_coupling_selected(&self, mode: &cxx_qt_lib::QString) {
        send_scpi(&format!(":CHAN2:COUP {}", mode.to_string()));
    }
    pub fn ch3_coupling_selected(&self, mode: &cxx_qt_lib::QString) {
        send_scpi(&format!(":CHAN3:COUP {}", mode.to_string()));
    }
    pub fn ch4_coupling_selected(&self, mode: &cxx_qt_lib::QString) {
        send_scpi(&format!(":CHAN4:COUP {}", mode.to_string()));
    }
    pub fn ch1_probe_selected(&self, probe: &cxx_qt_lib::QString) {
        let p = probe.to_string();
        let factor = if p.starts_with("10") { "10" } else { "1" };
        send_scpi(&format!(":CHAN1:PROB {}", factor));
    }
    pub fn ch2_probe_selected(&self, probe: &cxx_qt_lib::QString) {
        let p = probe.to_string();
        let factor = if p.starts_with("10") { "10" } else { "1" };
        send_scpi(&format!(":CHAN2:PROB {}", factor));
    }
    pub fn ch3_probe_selected(&self, probe: &cxx_qt_lib::QString) {
        let p = probe.to_string();
        let factor = if p.starts_with("10") { "10" } else { "1" };
        send_scpi(&format!(":CHAN3:PROB {}", factor));
    }
    pub fn ch4_probe_selected(&self, probe: &cxx_qt_lib::QString) {
        let p = probe.to_string();
        let factor = if p.starts_with("10") { "10" } else { "1" };
        send_scpi(&format!(":CHAN4:PROB {}", factor));
    }
}

impl cxx_qt::Initialize for oscilloscope::qobject::OscilloscopeBackend {
    fn initialize(self: Pin<&mut Self>) {
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            loop {
                if let Some(addr) = *crate::ADDR.lock().unwrap() {
                    if let Ok(mut stream) = TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
                        let _ = stream.write_all(b":DISP:DATA?\n");
                        let _ = stream.flush();
                        if let Ok(image_data) = read_ieee_block(&mut stream) {
                            let base64_string = base64::encode(&image_data);
                            let data_url = format!("data:image/png;base64,{}", base64_string);
                            qt_thread.queue(|qobj| {
                                qobj.set_scope_image(cxx_qt_lib::QString::from(&data_url));
                            });
                        }
                    }
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        });
    }
}

fn send_scpi(cmd: &str) {
    if let Some(addr) = *crate::ADDR.lock().unwrap() {
        if let Ok(mut sock) = TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
            let _ = sock.write_all(format!("{}\n", cmd).as_bytes());
            let _ = sock.flush();
        }
    }
}

fn read_ieee_block(stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
    let mut header = [0u8; 2];
    stream.read_exact(&mut header)?;
    if header[0] != b'#' {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing block header"));
    }
    let ndigits = (header[1] as char).to_digit(10).unwrap_or(0) as usize;
    if ndigits == 0 {
        return Ok(Vec::new());
    }
    let mut len_buf = vec![0u8; ndigits];
    stream.read_exact(&mut len_buf)?;
    let len_str = String::from_utf8_lossy(&len_buf);
    let total_len: usize = len_str.trim().parse().unwrap_or(0);
    let mut data = vec![0u8; total_len];
    stream.read_exact(&mut data)?;
    Ok(data)
}
