// src/oscilloscope_backend.rs
#[cxx_qt::bridge]
mod ffi {
    // Import Qt types
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(bool, avgEnabled)]
        #[qproperty(QString, scopeImageData)]
        type OscilloscopeBackend = super::OscilloscopeBackendRust;
    }
    extern "RustQt" {
        #[qinvokable]
        fn info_clicked(&self);
        #[qinvokable]
        fn settings_clicked(&self);
        #[qinvokable]
        fn autoscale_clicked(&self);
        #[qinvokable]
        fn console_clicked(&self);
        #[qinvokable]
        fn save_config_clicked(&self);
        #[qinvokable]
        fn load_config_clicked(&self);

        #[qinvokable]
        fn trigger_source_selected(&self, source: &QString);
        #[qinvokable]
        fn trigger_level_changed(&self, level: i32);
        #[qinvokable]
        fn trigger_slope_up(&self);
        #[qinvokable]
        fn trigger_slope_down(&self);
        #[qinvokable]
        fn single_trigger_clicked(&self);
        #[qinvokable]
        fn run_stop_clicked(&self);

        #[qinvokable]
        fn timebase_changed(&self, value: f64);
        #[qinvokable]
        fn time_offset_changed(&self, value: f64);
        #[qinvokable]
        fn average_toggled(self: Pin<&mut OscilloscopeBackend>, on: bool);

        #[qinvokable]
        fn ch1_enable_changed(&self, on: bool);
        #[qinvokable]
        fn ch1_scale_changed(&self, value: f64);
        #[qinvokable]
        fn ch1_offset_changed(&self, value: f64);
        #[qinvokable]
        fn ch1_coupling_selected(&self, mode: &QString);
        #[qinvokable]
        fn ch1_probe_selected(&self, probe: &QString);

        #[qinvokable]
        fn ch2_enable_changed(&self, on: bool);
        #[qinvokable]
        fn ch2_scale_changed(&self, value: f64);
        #[qinvokable]
        fn ch2_offset_changed(&self, value: f64);
        #[qinvokable]
        fn ch2_coupling_selected(&self, mode: &QString);
        #[qinvokable]
        fn ch2_probe_selected(&self, probe: &QString);

        #[qinvokable]
        fn ch3_enable_changed(&self, on: bool);
        #[qinvokable]
        fn ch3_scale_changed(&self, value: f64);
        #[qinvokable]
        fn ch3_offset_changed(&self, value: f64);
        #[qinvokable]
        fn ch3_coupling_selected(&self, mode: &QString);
        #[qinvokable]
        fn ch3_probe_selected(&self, probe: &QString);

        #[qinvokable]
        fn ch4_enable_changed(&self, on: bool);
        #[qinvokable]
        fn ch4_scale_changed(&self, value: f64);
        #[qinvokable]
        fn ch4_offset_changed(&self, value: f64);
        #[qinvokable]
        fn ch4_coupling_selected(&self, mode: &QString);
        #[qinvokable]
        fn ch4_probe_selected(&self, probe: &QString);

        #[qinvokable]
        fn initialize(self: Pin<&mut OscilloscopeBackend>);
    }
}
use std::net::{SocketAddr, TcpStream};
use std::io::{Write, Read};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use rigol_cli::utils::parse_source_arg;
use cxx_qt_lib::QString;
use base64;

pub struct OscilloscopeBackendRust {
    address: SocketAddr,
    running: Arc<AtomicBool>,
    current_timebase: Arc<Mutex<f64>>,
    ch1_on: Arc<AtomicBool>,
    ch2_on: Arc<AtomicBool>,
    ch3_on: Arc<AtomicBool>,
    ch4_on: Arc<AtomicBool>,
    avg_enabled: bool,
    scope_image_data: QString,
}
impl Default for OscilloscopeBackendRust {
    fn default() -> Self {
        OscilloscopeBackendRust {
            address: "169.254.50.23:5555".parse().unwrap(),
            running: Arc::new(AtomicBool::new(true)), // instrument initial running state
            current_timebase: Arc::new(Mutex::new(0.5)), // default 0.5 s/div (50% slider)
            ch1_on: Arc::new(AtomicBool::new(true)),
            ch2_on: Arc::new(AtomicBool::new(false)),
            ch3_on: Arc::new(AtomicBool::new(false)),
            ch4_on: Arc::new(AtomicBool::new(false)),
            avg_enabled: false,
            scope_image_data: QString::from(""),
        }
    }
}
impl ffi::OscilloscopeBackend {
    fn info_clicked(&self) {
        println!("ℹ Info");
    }
    fn settings_clicked(&self) {
        println!("⚙ Settings");
    }
    fn autoscale_clicked(&self) {
        send_scpi(&self.rust().address, ":AUTOSCALE");
    }
    fn console_clicked(&self) {
        println!(">_ Console");
    }
    fn save_config_clicked(&self) {
        println!("💾 Save config (todo)");
    }
    fn load_config_clicked(&self) {
        println!("↑ Load config (todo)");
    }

    fn trigger_source_selected(&self, source: &QString) {
        if let Ok(src) = parse_source_arg(&source.to_string()) {
            send_scpi(&self.rust().address, ":TRIG:MODE EDGE");
            send_scpi(&self.rust().address, &format!(":TRIG:EDGE:SOUR {}", src));
        }
    }
    fn trigger_level_changed(&self, level: i32) {
        send_scpi(&self.rust().address, &format!(":TRIG:EDGE:LEV {}", level));
    }
    fn trigger_slope_up(&self) {
        send_scpi(&self.rust().address, ":TRIG:EDGE:SLOP POS");
    }
    fn trigger_slope_down(&self) {
        send_scpi(&self.rust().address, ":TRIG:EDGE:SLOP NEG");
    }
    fn single_trigger_clicked(&self) {
        send_scpi(&self.rust().address, ":SING");
    }
    fn run_stop_clicked(&self) {
        let run_flag = &self.rust().running;
        if run_flag.load(Ordering::SeqCst) {
            send_scpi(&self.rust().address, ":STOP");
            run_flag.store(false, Ordering::SeqCst);
        } else {
            send_scpi(&self.rust().address, ":RUN");
            run_flag.store(true, Ordering::SeqCst);
        }
    }

    fn timebase_changed(&self, value: f64) {
        let scale = value / 100.0;
        *self.rust().current_timebase.lock().unwrap() = scale;
        send_scpi(&self.rust().address, &format!(":TIM:SCAL {}", scale));
    }
    fn time_offset_changed(&self, value: f64) {
        let base = *self.rust().current_timebase.lock().unwrap();
        let offs = base * (value as f64 / 50.0);
        send_scpi(&self.rust().address, &format!(":TIM:OFFS {}", offs));
    }
    fn average_toggled(self: Pin<&mut Self>, on: bool) {
        let this = self.rust();
        if on {
            send_scpi(&this.address, ":ACQ:TYPE AVER");
            send_scpi(&this.address, ":ACQ:AVER 16");
        } else {
            send_scpi(&this.address, ":ACQ:TYPE NORM");
        }
        self.set_avg_enabled(on);
    }

    fn ch1_enable_changed(&self, on: bool) {
        send_scpi(&self.rust().address, if on { ":CHAN1:DISP ON" } else { ":CHAN1:DISP OFF" });
        self.rust().ch1_on.store(on, Ordering::SeqCst);
    }
    fn ch1_scale_changed(&self, value: f64) {
        send_scpi(&self.rust().address, &format!(":CHAN1:SCAL {}", value));
    }
    fn ch1_offset_changed(&self, value: f64) {
        send_scpi(&self.rust().address, &format!(":CHAN1:OFFS {}", value));
    }
    fn ch1_coupling_selected(&self, mode: &QString) {
        send_scpi(&self.rust().address, &format!(":CHAN1:COUP {}", mode.to_string()));
    }
    fn ch1_probe_selected(&self, probe: &QString) {
        let p = probe.to_string();
        let factor = if p.starts_with("10") { "10" } else { "1" };
        send_scpi(&self.rust().address, &format!(":CHAN1:PROB {}", factor));
    }

    fn ch2_enable_changed(&self, on: bool) {
        send_scpi(&self.rust().address, if on { ":CHAN2:DISP ON" } else { ":CHAN2:DISP OFF" });
        self.rust().ch2_on.store(on, Ordering::SeqCst);
    }
    fn ch2_scale_changed(&self, value: f64) {
        send_scpi(&self.rust().address, &format!(":CHAN2:SCAL {}", value));
    }
    fn ch2_offset_changed(&self, value: f64) {
        send_scpi(&self.rust().address, &format!(":CHAN2:OFFS {}", value));
    }
    fn ch2_coupling_selected(&self, mode: &QString) {
        send_scpi(&self.rust().address, &format!(":CHAN2:COUP {}", mode.to_string()));
    }
    fn ch2_probe_selected(&self, probe: &QString) {
        let p = probe.to_string();
        let factor = if p.starts_with("10") { "10" } else { "1" };
        send_scpi(&self.rust().address, &format!(":CHAN2:PROB {}", factor));
    }

    fn ch3_enable_changed(&self, on: bool) {
        send_scpi(&self.rust().address, if on { ":CHAN3:DISP ON" } else { ":CHAN3:DISP OFF" });
        self.rust().ch3_on.store(on, Ordering::SeqCst);
    }
    fn ch3_scale_changed(&self, value: f64) {
        send_scpi(&self.rust().address, &format!(":CHAN3:SCAL {}", value));
    }
    fn ch3_offset_changed(&self, value: f64) {
        send_scpi(&self.rust().address, &format!(":CHAN3:OFFS {}", value));
    }
    fn ch3_coupling_selected(&self, mode: &QString) {
        send_scpi(&self.rust().address, &format!(":CHAN3:COUP {}", mode.to_string()));
    }
    fn ch3_probe_selected(&self, probe: &QString) {
        let p = probe.to_string();
        let factor = if p.starts_with("10") { "10" } else { "1" };
        send_scpi(&self.rust().address, &format!(":CHAN3:PROB {}", factor));
    }

    fn ch4_enable_changed(&self, on: bool) {
        send_scpi(&self.rust().address, if on { ":CHAN4:DISP ON" } else { ":CHAN4:DISP OFF" });
        self.rust().ch4_on.store(on, Ordering::SeqCst);
    }
    fn ch4_scale_changed(&self, value: f64) {
        send_scpi(&self.rust().address, &format!(":CHAN4:SCAL {}", value));
    }
    fn ch4_offset_changed(&self, value: f64) {
        send_scpi(&self.rust().address, &format!(":CHAN4:OFFS {}", value));
    }
    fn ch4_coupling_selected(&self, mode: &QString) {
        send_scpi(&self.rust().address, &format!(":CHAN4:COUP {}", mode.to_string()));
    }
    fn ch4_probe_selected(&self, probe: &QString) {
        let p = probe.to_string();
        let factor = if p.starts_with("10") { "10" } else { "1" };
        send_scpi(&self.rust().address, &format!(":CHAN4:PROB {}", factor));
    }

    fn initialize(self: Pin<&mut Self>) {
        let this = self.rust();
        let addr = this.address;
        // Initial device setup
        send_scpi(&addr, ":CHAN1:DISP ON");
        send_scpi(&addr, ":CHAN2:DISP OFF");
        send_scpi(&addr, ":CHAN3:DISP OFF");
        send_scpi(&addr, ":CHAN4:DISP OFF");
        send_scpi(&addr, ":OUTPUT1 OFF");
        send_scpi(&addr, ":OUTPUT2 OFF");
        println!("(Init) CH1 ON, CH2-CH4 OFF, Outputs OFF [{}]", addr);

        // Start background thread for waveform capture and display
        let ui_weak_flags = [
            this.ch1_on.clone(),
            this.ch2_on.clone(),
            this.ch3_on.clone(),
            this.ch4_on.clone()
        ];
        let image_data_ref = this.scope_image_data.clone();
        let scope_obj = self.clone();
        thread::spawn(move || {
            let display_width: usize = 750;
            let display_height: usize = 650;
            let mut stream: Option<TcpStream> = None;
            loop {
                // If the UI object was destroyed (scope_obj dropped), break
                if scope_obj.as_ref().is_null() {
                    break;
                }
                // Connect if not connected
                if stream.is_none() {
                    if let Ok(s) = TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
                        stream = Some(s);
                    } else {
                        thread::sleep(Duration::from_millis(50));
                        continue;
                    }
                }
                let s = stream.as_mut().unwrap();
                // Configure waveform data mode
                if s.write_all(b":WAV:MODE NORM\n:WAV:FORM BYTE\n").is_err() {
                    stream = None;
                    continue;
                }
                if s.flush().is_err() {
                    stream = None;
                    continue;
                }
                // Determine active channels from flags
                let mut active_channels: Vec<usize> = Vec::new();
                for (i, flag) in ui_weak_flags.iter().enumerate() {
                    if flag.load(Ordering::SeqCst) {
                        active_channels.push(i + 1);
                    }
                }
                if active_channels.is_empty() {
                    // No active channels: fill image with background color
                    let bg = 0x1e;
                    let mut rgba = vec![bg; display_width * display_height * 4];
                    for i in 0..display_width * display_height {
                        rgba[i*4+3] = 0xff;
                    }
                    // Encode to PNG and update QML
                    update_scope_image(&scope_obj, display_width as u32, display_height as u32, &rgba);
                    stream = None;
                    continue;
                }
                // Use first active channel for preamble
                let ref_ch = active_channels[0];
                let pre_cmd = format!(":WAV:SOUR CHAN{ref_ch}\n:WAV:PRE?\n");
                if s.write_all(pre_cmd.as_bytes()).is_err() {
                    stream = None;
                    continue;
                }
                if s.flush().is_err() {
                    stream = None;
                    continue;
                }
                // Read preamble response
                let mut pre_buffer = Vec::new();
                let mut temp = [0u8; 256];
                loop {
                    match s.read(&mut temp) {
                        Ok(n) if n > 0 => {
                            pre_buffer.extend_from_slice(&temp[..n]);
                            if pre_buffer.contains(&b'\n') { break; }
                        }
                        Ok(_) => break,
                        Err(_) => { stream = None; continue; }
                    }
                }
                if let Some(pos) = pre_buffer.iter().position(|&b| b == b'\n') {
                    pre_buffer.truncate(pos);
                }
                let pre_str = String::from_utf8_lossy(&pre_buffer);
                let parts: Vec<&str> = pre_str.split(',').collect();
                let (x_inc, x_org, y_inc, y_org, y_ref) = if parts.len() >= 10 {
                    (
                        parts[4].parse().unwrap_or(0.0),
                        parts[5].parse().unwrap_or(0.0),
                        parts[7].parse().unwrap_or(1.0),
                        parts[8].parse().unwrap_or(0.0),
                        parts[9].parse().unwrap_or(0.0)
                    )
                } else {
                    (0.0, 0.0, 1.0, 0.0, 0.0)
                };
                // Query vertical scale and offset for each active channel
                let mut scales = [0.0_f64; 4];
                let mut offsets = [0.0_f64; 4];
                for &ch in &active_channels {
                    if s.write_all(format!(":CHAN{ch}:SCAL?\n").as_bytes()).is_err() { stream = None; continue; }
                    if s.flush().is_err() { stream = None; continue; }
                    let mut resp = Vec::new();
                    let mut buf = [0u8; 64];
                    loop {
                        match s.read(&mut buf) {
                            Ok(n) if n > 0 => {
                                resp.extend_from_slice(&buf[..n]);
                                if resp.contains(&b'\n') { break; }
                            }
                            Ok(_) => break,
                            Err(_) => { stream = None; continue; }
                        }
                    }
                    if let Some(pos) = resp.iter().position(|&b| b == b'\n') {
                        resp.truncate(pos);
                    }
                    let scale_str = String::from_utf8_lossy(&resp);
                    scales[ch-1] = scale_str.trim().parse().unwrap_or(0.0);
                    if s.write_all(format!(":CHAN{ch}:OFFS?\n").as_bytes()).is_err() { stream = None; continue; }
                    if s.flush().is_err() { stream = None; continue; }
                    resp.clear();
                    loop {
                        match s.read(&mut buf) {
                            Ok(n) if n > 0 => {
                                resp.extend_from_slice(&buf[..n]);
                                if resp.contains(&b'\n') { break; }
                            }
                            Ok(_) => break,
                            Err(_) => { stream = None; continue; }
                        }
                    }
                    if let Some(pos) = resp.iter().position(|&b| b == b'\n') {
                        resp.truncate(pos);
                    }
                    let offs_str = String::from_utf8_lossy(&resp);
                    offsets[ch-1] = offs_str.trim().parse().unwrap_or(0.0);
                }
                // Retrieve waveform data for each active channel
                let mut channel_data: Vec<(usize, Vec<u8>)> = Vec::new();
                for &ch in &active_channels {
                    let cmd = format!(":WAV:SOUR CHAN{ch}\n:WAV:DATA?\n");
                    if s.write_all(cmd.as_bytes()).is_err() { stream = None; continue; }
                    if s.flush().is_err() { stream = None; continue; }
                    // Read the binary block header
                    let mut hdr = [0u8; 2];
                    if s.read_exact(&mut hdr).is_err() || hdr[0] != b'#' {
                        thread::sleep(Duration::from_millis(200));
                        continue;
                    }
                    let ndigits = (hdr[1] as char).to_digit(10).unwrap_or(0) as usize;
                    let mut len_buf = vec![0u8; ndigits];
                    if s.read_exact(&mut len_buf).is_err() { stream = None; continue; }
                    let data_len = String::from_utf8_lossy(&len_buf).parse().unwrap_or(0);
                    let mut raw_data = vec![0u8; data_len];
                    if s.read_exact(&mut raw_data).is_err() { stream = None; continue; }
                    // consume trailing newline if present
                    let _ = s.read(&mut [0u8; 1]);
                    channel_data.push((ch, raw_data));
                }
                // Prepare an RGB buffer for plotting
                let mut rgb_buf = vec![0u8; display_width * display_height * 3];
                {
                    use plotters::prelude::*;
                    let backend = BitMapBackend::with_buffer(&mut rgb_buf, (display_width as u32, display_height as u32));
                    let root = backend.into_drawing_area();
                    let bg_color = RGBColor(30, 30, 30);
                    if root.fill(&bg_color).is_err() {
                        drop(root);
                        stream = None;
                        continue;
                    }
                    let color_for_channel = |ch: usize| -> RGBColor {
                        match ch {
                            1 => RGBColor(255, 255, 0),
                            2 => RGBColor(0, 255, 255),
                            3 => RGBColor(255, 0, 255),
                            4 => RGBColor(0, 255, 0),
                            _ => RGBColor(255, 255, 255),
                        }
                    };
                    for ch in (1..=4).rev() {
                        if let Some((_, data_bytes)) = channel_data.iter().find(|(c, _)| *c == ch) {
                            let scale = scales[ch-1];
                            let offs = offsets[ch-1];
                            let n = data_bytes.len();
                            if n < 2 || scale == 0.0 {
                                continue;
                            }
                            let time_span = x_inc * ((n-1) as f64);
                            let mut points: Vec<(i32, i32)> = Vec::with_capacity(n);
                            for (i, &byte) in data_bytes.iter().enumerate() {
                                let t = x_org + i as f64 * x_inc;
                                let x_pix = ((t - x_org) / time_span * ((display_width - 1) as f64)).round() as i32;
                                let volt = ((byte as f64) - y_ref) * y_inc + y_org;
                                let disp_v = volt + offs;
                                let center_y = (display_height - 1) as f64 / 2.0;
                                let pix_per_div = (display_height - 1) as f64 / 8.0;
                                let y_pix = (center_y - (disp_v / scale) * pix_per_div).round() as i32;
                                points.push((x_pix, y_pix));
                            }
                            let trace_color = color_for_channel(ch);
                            let style = ShapeStyle::from(&trace_color).stroke_width(2);
                            if root.draw(&PathElement::new(points, style)).is_err() {
                                drop(root);
                                stream = None;
                                continue;
                            }
                        }
                    }
                    if root.present().is_err() {
                        drop(root);
                        stream = None;
                        continue;
                    }
                }
                // Convert RGB buffer to RGBA
                let total_px = display_width * display_height;
                let mut rgba_data = Vec::with_capacity(total_px * 4);
                for i in 0..total_px {
                    let r = rgb_buf[i*3];
                    let g = rgb_buf[i*3+1];
                    let b = rgb_buf[i*3+2];
                    rgba_data.push(r);
                    rgba_data.push(g);
                    rgba_data.push(b);
                    rgba_data.push(0xFF);
                }
                // Update image in QML
                update_scope_image(&scope_obj, display_width as u32, display_height as u32, &rgba_data);
                // small yield to allow UI thread to process
                thread::yield_now();
            }
        });
    }
}
impl Drop for OscilloscopeBackendRust {
    fn drop(&mut self) {
        // When UI is closed, no special action needed (thread will exit via weak check)
    }
}
// Helper functions for SCPI I/O
fn send_scpi(addr: &SocketAddr, cmd: &str) {
    if let Ok(mut stream) = TcpStream::connect_timeout(addr, Duration::from_millis(500)) {
        let _ = stream.write_all(format!("{}\n", cmd).as_bytes());
        let _ = stream.flush();
    }
}
fn update_scope_image(obj: &ffi::OscilloscopeBackend, width: u32, height: u32, rgba_pixels: &[u8]) {
    // Encode RGBA pixel buffer to PNG data URL
    let img = image::RgbaImage::from_raw(width, height, rgba_pixels.to_vec()).unwrap();
    let mut png_bytes = Vec::new();
    let _ = image::DynamicImage::ImageRgba8(img).write_to(&mut png_bytes, image::ImageOutputFormat::Png);
    let b64 = base64::encode(&png_bytes);
    let data_url = QString::from(&format!("data:image/png;base64,{}", b64));
    obj.set_scope_image_data(data_url);
}
