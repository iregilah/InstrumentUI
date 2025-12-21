// src/graph_object.rs

use crate::oscillo_data_provider;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{
    PenStyle, QColor, QLineF, QPainterRenderHint, QPen, QPoint, QRectF, QSizeF, QString,
};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
#[cxx_qt::bridge]
pub mod graph_object_qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = cxx_qt_lib::QSizeF;
        include!("cxx-qt-lib/qrectf.h");
        type QRectF = cxx_qt_lib::QRectF;
        include!("cxx-qt-lib/qpen.h");
        type QPen = cxx_qt_lib::QPen;
        include!("cxx-qt-lib/qpainter.h");
        type QPainter = cxx_qt_lib::QPainter;
        include!(<QtQuick/QQuickPaintedItem>);
        type QQuickPaintedItem;
        include!(<QtQuick/QQuickItem>);
        type QQuickItem;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QQuickPaintedItem]
        #[qproperty(bool, legend_visible, cxx_name = "legendVisible")]
        #[qproperty(i32, legend_position, cxx_name = "legendPosition")]
        #[qproperty(bool, grid_visible, cxx_name = "gridVisible")]
        #[qproperty(bool, x_auto_range, cxx_name = "xAutoRange")]
        #[qproperty(bool, y_auto_range, cxx_name = "yAutoRange")]
        #[qproperty(f64, x_min, cxx_name = "xMin")]
        #[qproperty(f64, x_max, cxx_name = "xMax")]
        #[qproperty(f64, y_min, cxx_name = "yMin")]
        #[qproperty(f64, y_max, cxx_name = "yMax")]
        #[qproperty(QString, x_label, cxx_name = "xLabel")]
        #[qproperty(QString, y_label, cxx_name = "yLabel")]
        #[qproperty(i32, mode)]
        #[qproperty(bool, separate_series, cxx_name = "separateSeries")]
        #[qproperty(i32, buffer_size, cxx_name = "bufferSize")]
        #[qproperty(bool, dark_mode, cxx_name = "darkMode")]
        #[qproperty(bool, x_log_scale, cxx_name = "xLogScale")]
        #[qproperty(bool, y_log_scale, cxx_name = "yLogScale")]
        #[qproperty(i32, x_divisions, cxx_name = "xDivisions")]
        #[qproperty(i32, y_divisions, cxx_name = "yDivisions")]
        #[qproperty(bool, bode_mode, cxx_name = "bodeMode")]
        type GraphObject = super::GraphObjectRust;
    }
    impl cxx_qt::Threading for GraphObject {}

    impl cxx_qt::Constructor<()> for GraphObject {}
    extern "RustQt" {
        #[qinvokable]
        #[cxx_name = "addSeries"]
        fn add_series(
            self: Pin<&mut GraphObject>,
            name: &QString,
            series_type: i32,
            color: &QColor,
            thickness: f64,
            line_style: i32,
            marker: bool,
        );
        #[qinvokable]
        #[cxx_name = "removeSeries"]
        fn remove_series(self: Pin<&mut GraphObject>, name: &QString);
        #[qinvokable]
        #[cxx_name = "addDataPoint"]
        fn add_data_point(self: Pin<&mut GraphObject>, series_name: &QString, x: f64, y: f64);
        #[qinvokable]
        #[cxx_name = "loadOscilloscopeData"]
        fn load_oscilloscope_data(self: Pin<&mut GraphObject>, channel: i32);
        // Live acquisition (background thread + UI pump)
        #[qinvokable]
        #[cxx_name = "startLive"]
        fn start_live(self: Pin<&mut GraphObject>, channel: i32, period_ms: i32);
        #[qinvokable]
        #[cxx_name = "stopLive"]
        fn stop_live(self: Pin<&mut GraphObject>);
        #[qinvokable]
        #[cxx_name = "pumpLive"]
        fn pump_live(self: Pin<&mut GraphObject>);
        #[qinvokable]
        #[cxx_name = "zoomToRegion"]
        fn zoom_to_region(self: Pin<&mut GraphObject>, x1: f64, x2: f64, y1: f64, y2: f64);
        #[qinvokable]
        #[cxx_name = "zoomX"]
        fn zoom_x(self: Pin<&mut GraphObject>, x1: f64, x2: f64);
        #[qinvokable]
        #[cxx_name = "zoomY"]
        fn zoom_y(self: Pin<&mut GraphObject>, y1: f64, y2: f64);
        #[qinvokable]
        #[cxx_name = "zoomAtPoint"]
        fn zoom_at_point(self: Pin<&mut GraphObject>, center_x: f64, center_y: f64, factor: f64);
        #[qinvokable]
        #[cxx_name = "pan"]
        fn pan(self: Pin<&mut GraphObject>, delta_x: f64, delta_y: f64);
        #[qinvokable]
        #[cxx_name = "resetZoom"]
        fn reset_zoom(self: Pin<&mut GraphObject>);
        #[qinvokable]
        #[cxx_name = "saveCsv"]
        fn save_csv(self: Pin<&mut GraphObject>, file_path: &QString);
        #[qinvokable]
        #[cxx_name = "saveImage"]
        fn save_image(self: Pin<&mut GraphObject>, file_path: &QString);
        #[qinvokable]
        #[cxx_name = "copyData"]
        fn copy_data(self: Pin<&mut GraphObject>);
        #[qinvokable]
        #[cxx_name = "copyImage"]
        fn copy_image(self: Pin<&mut GraphObject>);
        #[qinvokable]
        #[cxx_name = "placeVerticalCursor"]
        fn place_vertical_cursor(self: Pin<&mut GraphObject>, x: f64);
        #[qinvokable]
        #[cxx_name = "placeHorizontalCursor"]
        fn place_horizontal_cursor(self: Pin<&mut GraphObject>, y: f64);
        #[qinvokable]
        #[cxx_name = "clearCursors"]
        fn clear_cursors(self: Pin<&mut GraphObject>);
        #[qinvokable]
        #[cxx_name = "requestRepaint"]
        fn request_repaint(self: Pin<&mut GraphObject>);
        // Rust -> QML: kérünk clipboard szöveg másolást
        #[qsignal]
        #[cxx_name = "requestCopyData"]
        fn request_copy_data(self: Pin<&mut GraphObject>, text: &QString);
        // Rust -> QML: kérünk egy aszinkron grabToImage-ot + clipboard image-et
        #[qsignal]
        #[cxx_name = "requestCopyImage"]
        fn request_copy_image(self: Pin<&mut GraphObject>);

        #[qsignal]
        #[cxx_name = "requestSaveImage"]
        fn request_save_image(self: Pin<&mut GraphObject>, file_path: &QString);
        #[cxx_override]
        unsafe fn paint(self: Pin<&mut GraphObject>, painter: *mut QPainter);
    }

    unsafe extern "RustQt" {
        #[inherit]
        fn update(self: Pin<&mut GraphObject>);
        #[inherit]
        fn size(self: &GraphObject) -> QSizeF;
    }
}

// Custom constructor implementáció (QQuickItem* parent)
impl cxx_qt::Constructor<()> for graph_object_qobject::GraphObject {
    type NewArguments = ();
    type BaseArguments = ();
    type InitializeArguments = ();
    fn route_arguments(
        _: (),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        ((), (), ())
    }

    fn new((): Self::NewArguments) -> <Self as cxx_qt::CxxQtType>::Rust {
        GraphObjectRust::default()
    }
}

pub struct DataSeries {
    name: String,
    is_digital: bool,
    color: QColor,
    thickness: f64,
    line_style: i32,
    marker: bool,
    data_x: Vec<f64>,
    data_y: Vec<f64>,
    min_y: f64,
    max_y: f64,
}
impl Default for DataSeries {
    fn default() -> Self {
        Self {
            name: String::new(),
            is_digital: false,
            color: QColor::from_rgba(255, 255, 255, 255),
            thickness: 1.0,
            line_style: 1, // SolidLine
            marker: false,
            data_x: Vec::new(),
            data_y: Vec::new(),
            min_y: 0.0,
            max_y: 0.0,
        }
    }
}

pub struct GraphObjectRust {
    series_list: Vec<DataSeries>,
    legend_visible: bool,
    legend_position: i32,
    grid_visible: bool,
    x_auto_range: bool,
    y_auto_range: bool,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    mode: i32, // 0: mode1 (compress), 1: mode2 (scroll), 2: mode3 (triggered)
    separate_series: bool,
    buffer_size: i32,
    dark_mode: bool,
    x_label: QString,
    y_label: QString,
    // internal state:
    initial_x_set: bool,
    last_frame_span: Option<f64>,
    cursor_x_positions: Vec<f64>,
    cursor_y_positions: Vec<f64>,
    x_log_scale: bool,
    y_log_scale: bool,
    // oscilloscope-like grid divisions
    x_divisions: i32,
    y_divisions: i32,
    bode_mode: bool,

    // units for SI formatting (optional, empty for generic graphs)
    x_unit: QString,
    y_unit: QString,

    // live acquisition state
    live_channel: u8,
    live_period_ms: u64,
    live_stop: Arc<AtomicBool>,
    live_latest: Arc<Mutex<Option<oscillo_data_provider::Waveform>>>,
    live_thread: Option<thread::JoinHandle<()>>,
}
impl Default for GraphObjectRust {
    fn default() -> Self {
        Self {
            series_list: Vec::new(),
            legend_visible: false,
            legend_position: 1, // default top-right
            grid_visible: true,
            x_auto_range: true,
            y_auto_range: true,
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
            mode: 0,
            separate_series: false,
            buffer_size: 1000,
            dark_mode: true,
            x_label: QString::from(""),
            y_label: QString::from(""),
            initial_x_set: false,
            last_frame_span: None,
            cursor_x_positions: Vec::new(),
            cursor_y_positions: Vec::new(),
            x_log_scale: false,
            y_log_scale: false,
            x_divisions: 10,
            y_divisions: 8,
            x_unit: QString::from(""),
            y_unit: QString::from(""),
            bode_mode: false,

            live_channel: 1,
            live_period_ms: 200,
            live_stop: Arc::new(AtomicBool::new(false)),
            live_latest: Arc::new(Mutex::new(None)),
            live_thread: None,
        }
    }
}

impl graph_object_qobject::GraphObject {
    fn si_scale(&self, reference: f64) -> (f64, &'static str) {
        // Returns (multiplier, prefix) where displayed_value = value * multiplier
        // Example: reference=0.002 -> (1e3, "m")  => display in ms/mV, etc.
        if !reference.is_finite() || reference == 0.0 {
            return (1.0, "");
        }
        let exp = reference.abs().log10().floor() as i32;
        let mut exp3 = (exp / 3) * 3;
        if exp3 < -9 {
            exp3 = -9;
        }
        if exp3 > 9 {
            exp3 = 9;
        }
        let mult = 10.0_f64.powi(-exp3);
        let prefix = match exp3 {
            -9 => "n",
            -6 => "µ",
            -3 => "m",
            0 => "",
            3 => "k",
            6 => "M",
            9 => "G",
            _ => "",
        };
        (mult, prefix)
    }
    fn format_value(&self, val: f64) -> QString {
        // Format a value with appropriate significant digits or scientific notation
        if val == 0.0 {
            return QString::from("0");
        }
        // If val is nearly an integer, format without decimals
        if (val - val.round()).abs() < 1e-9 {
            return QString::from(&format!("{:.0}", val));
        }
        let absval = val.abs();
        let formatted = if absval >= 1000.0 || (absval > 0.0 && absval < 0.01) {
            // Scientific notation with 2 decimals
            format!("{:.2e}", val)
        } else if absval >= 100.0 {
            format!("{:.0}", val)
        } else if absval >= 10.0 {
            format!("{:.1}", val)
        } else if absval >= 1.0 {
            format!("{:.2}", val)
        } else {
            format!("{:.3}", val)
        };
        // Trim trailing zeros and decimal point if not needed
        if formatted.contains('e') {
            // For scientific, trim trailing zeros in mantissa
            if let Some(e_idx) = formatted.find('e') {
                let (mantissa, exponent) = formatted.split_at(e_idx);
                let mut mantissa_trim = mantissa.trim_end_matches('0').trim_end_matches('.');
                if mantissa_trim.is_empty() {
                    mantissa_trim = "0";
                }
                return QString::from(&format!("{}{}", mantissa_trim, exponent));
            }
        } else if formatted.contains('.') {
            let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
            return QString::from(trimmed);
        }
        QString::from(&formatted)
    }

    fn setup_painter(self: Pin<&Self>, painter: &mut Pin<&mut graph_object_qobject::QPainter>) {
        painter
            .as_mut()
            .set_render_hint(QPainterRenderHint::Antialiasing, true);
        painter
            .as_mut()
            .set_render_hint(QPainterRenderHint::TextAntialiasing, true);
    }

    fn draw_line(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) {
        let mut line = QLineF::default();
        line.set_line(x1, y1, x2, y2);
        painter.as_mut().draw_linef(&line);
    }

    fn draw_text(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        x: f64,
        y: f64,
        text: &QString,
    ) {
        let pt = QPoint::new(x.round() as i32, y.round() as i32);
        painter.as_mut().draw_text(&pt, text);
    }

    fn pixel_x(
        self: Pin<&Self>,
        x: f64,
        x_min: f64,
        x_max: f64,
        plot_x: f64,
        plot_width: f64,
    ) -> Option<f64> {
        let this = self.rust();
        if this.x_log_scale {
            if x <= 0.0 {
                None
            } else {
                let log_min = x_min.log10();
                let log_max = x_max.log10();
                Some(plot_x + ((x.log10() - log_min) / (log_max - log_min)) * plot_width)
            }
        } else {
            Some(plot_x + ((x - x_min) / (x_max - x_min)) * plot_width)
        }
    }

    fn pixel_y(
        self: Pin<&Self>,
        y: f64,
        y_min: f64,
        y_max: f64,
        plot_y: f64,
        plot_height: f64,
    ) -> Option<f64> {
        let this = self.rust();
        if this.y_log_scale {
            if y <= 0.0 {
                None
            } else {
                let log_min = y_min.log10();
                let log_max = y_max.log10();
                Some(
                    plot_y + plot_height
                        - ((y.log10() - log_min) / (log_max - log_min)) * plot_height,
                )
            }
        } else {
            Some(plot_y + plot_height - ((y - y_min) / (y_max - y_min)) * plot_height)
        }
    }

    fn pixel_y_separate(
        self: Pin<&Self>,
        y: f64,
        local_min: f64,
        local_max: f64,
        band_bottom_y: f64,
        band_height: f64,
    ) -> Option<f64> {
        let this = self.rust();
        if this.y_log_scale {
            if y <= 0.0 {
                None
            } else {
                let log_min = local_min.log10();
                let log_max = local_max.log10();
                let denom = (log_max - log_min).max(1e-9);
                Some(band_bottom_y - ((y.log10() - log_min) / denom) * band_height)
            }
        } else {
            Some(band_bottom_y - ((y - local_min) / (local_max - local_min)) * band_height)
        }
    }

    fn draw_background(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        width: f64,
        height: f64,
    ) {
        let this = self.rust();
        let bg_color = if this.dark_mode {
            QColor::from_rgb(0, 0, 0)
        } else {
            QColor::from_rgb(255, 255, 255)
        };
        painter
            .as_mut()
            .fill_rect(&QRectF::new(0.0, 0.0, width, height), &bg_color);
    }

    fn compute_plot_area(
        self: Pin<&Self>,
        width: f64,
        height: f64,
    ) -> (f64, f64, f64, f64, f64, f64) {
        let this = self.rust();
        let left_margin = 60.0;
        let right_margin = 10.0;
        let top_margin = if this.legend_visible && this.legend_position < 2 {
            20.0
        } else {
            10.0
        };
        let bottom_margin = 50.0;

        let plot_x = left_margin;
        let plot_y = top_margin;
        let plot_width = (width - left_margin - right_margin).max(1.0);
        let plot_height = (height - top_margin - bottom_margin).max(1.0);

        let x_axis_y = plot_y + plot_height;
        let y_axis_x = plot_x;
        (plot_x, plot_y, plot_width, plot_height, x_axis_y, y_axis_x)
    }

    fn effective_x_range(self: Pin<&Self>) -> (f64, f64) {
        let this = self.rust();
        if this.x_log_scale {
            if this.x_max <= 0.0 {
                (0.1, 1.0)
            } else {
                let min_val = if this.x_min > 0.0 {
                    this.x_min
                } else {
                    (this.x_max / 1e6).max(1e-9)
                };
                (min_val, this.x_max)
            }
        } else {
            (this.x_min, this.x_max)
        }
    }

    fn effective_y_range(self: Pin<&Self>) -> (f64, f64) {
        let this = self.rust();
        if this.y_log_scale {
            if this.y_max <= 0.0 {
                (0.1, 1.0)
            } else {
                let min_val = if this.y_min > 0.0 {
                    this.y_min
                } else {
                    (this.y_max / 1e6).max(1e-9)
                };
                (min_val, this.y_max)
            }
        } else {
            (this.y_min, this.y_max)
        }
    }

    fn draw_grid_and_axes(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        plot_x: f64,
        plot_y: f64,
        plot_width: f64,
        plot_height: f64,
        x_axis_y: f64,
        y_axis_x: f64,
        x_min_val: f64,
        x_max_val: f64,
        y_min_val: f64,
        y_max_val: f64,
    ) {
        let this = self.rust();
        let axis_color = if this.dark_mode {
            QColor::from_rgb(255, 255, 255)
        } else {
            QColor::from_rgb(0, 0, 0)
        };
        let grid_color = QColor::from_rgb(136, 136, 136);

        // Vertical grid lines
        let mut grid_pen = QPen::default();
        grid_pen.set_color(&grid_color);
        grid_pen.set_width(0);
        grid_pen.set_style(if this.grid_visible {
            PenStyle::DashLine
        } else {
            PenStyle::SolidLine
        });
        painter.as_mut().set_pen(&grid_pen);

        let x_divs = this.x_divisions.max(1) as usize;
        for i in 0..=x_divs {
            let t = i as f64 / x_divs as f64;
            let data_x_val = if this.x_log_scale {
                let log_min = x_min_val.log10();
                let log_max = x_max_val.log10();
                (10.0_f64).powf(log_min + t * (log_max - log_min))
            } else {
                x_min_val + t * (x_max_val - x_min_val)
            };

            if this.grid_visible {
                if let Some(x_pixel) =
                    self.pixel_x(data_x_val, x_min_val, x_max_val, plot_x, plot_width)
                {
                    self.draw_line(painter, x_pixel, plot_y, x_pixel, plot_y + plot_height);
                }
            }
        }

        // Vertical axis line (left)
        let mut axis_pen = QPen::default();
        axis_pen.set_color(&axis_color);
        axis_pen.set_width(0);
        axis_pen.set_style(PenStyle::SolidLine);
        painter.as_mut().set_pen(&axis_pen);
        self.draw_line(painter, y_axis_x, plot_y, y_axis_x, plot_y + plot_height);

        // Horizontal grid lines
        let mut grid_pen2 = QPen::default();
        grid_pen2.set_color(&grid_color);
        grid_pen2.set_width(0);
        grid_pen2.set_style(if this.grid_visible {
            PenStyle::DashLine
        } else {
            PenStyle::SolidLine
        });
        painter.as_mut().set_pen(&grid_pen2);

        let y_divs = this.y_divisions.max(1) as usize;
        for j in 0..=y_divs {
            let t = j as f64 / y_divs as f64;
            let data_y_val = if this.y_log_scale {
                let log_min = y_min_val.log10();
                let log_max = y_max_val.log10();
                (10.0_f64).powf(log_min + t * (log_max - log_min))
            } else {
                y_min_val + t * (y_max_val - y_min_val)
            };

            if this.grid_visible && !this.separate_series {
                if let Some(y_pixel) =
                    self.pixel_y(data_y_val, y_min_val, y_max_val, plot_y, plot_height)
                {
                    self.draw_line(painter, plot_x, y_pixel, plot_x + plot_width, y_pixel);
                }
            }
        }

        // Horizontal axis line (bottom)
        let mut axis_pen2 = QPen::default();
        axis_pen2.set_color(&axis_color);
        axis_pen2.set_width(0);
        axis_pen2.set_style(PenStyle::SolidLine);
        painter.as_mut().set_pen(&axis_pen2);
        self.draw_line(painter, plot_x, x_axis_y, plot_x + plot_width, x_axis_y);
    }

    fn draw_x_axis_ticks_and_labels(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        plot_x: f64,
        x_axis_y: f64,
        plot_width: f64,
        x_min_val: f64,
        x_max_val: f64,
        x_mul: f64,
    ) {
        let this = self.rust();
        let axis_color = if this.dark_mode {
            QColor::from_rgb(255, 255, 255)
        } else {
            QColor::from_rgb(0, 0, 0)
        };
        let mut axis_pen = QPen::default();
        axis_pen.set_color(&axis_color);
        axis_pen.set_width(0);
        axis_pen.set_style(PenStyle::SolidLine);
        painter.as_mut().set_pen(&axis_pen);

        let x_divs = this.x_divisions.max(1) as usize;
        for i in 0..=x_divs {
            let t = i as f64 / x_divs as f64;
            let data_x_val = if this.x_log_scale {
                let log_min = x_min_val.log10();
                let log_max = x_max_val.log10();
                (10.0_f64).powf(log_min + t * (log_max - log_min))
            } else {
                x_min_val + t * (x_max_val - x_min_val)
            };

            if let Some(x_pixel) =
                self.pixel_x(data_x_val, x_min_val, x_max_val, plot_x, plot_width)
            {
                // tick mark (vertical small line)
                let tick_len = 5.0;
                self.draw_line(painter, x_pixel, x_axis_y, x_pixel, x_axis_y - tick_len);

                // label
                let label = self.format_value(data_x_val * x_mul);
                let label_str = label.to_string();
                let approx_width = label_str.len() as f64 * 7.0;

                let text_x = if i == 0 {
                    x_pixel
                } else if i == x_divs {
                    x_pixel - approx_width
                } else {
                    x_pixel - approx_width / 2.0
                };
                let text_y = x_axis_y + 15.0;
                self.draw_text(painter, text_x, text_y, &label);
            }
        }
    }

    fn draw_y_axis_ticks_and_labels(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        plot_x: f64,
        plot_y: f64,
        plot_height: f64,
        y_min_val: f64,
        y_max_val: f64,
        y_mul: f64,
    ) {
        let this = self.rust();
        if this.separate_series {
            return;
        }

        let axis_color = if this.dark_mode {
            QColor::from_rgb(255, 255, 255)
        } else {
            QColor::from_rgb(0, 0, 0)
        };
        let mut axis_pen = QPen::default();
        axis_pen.set_color(&axis_color);
        axis_pen.set_width(0);
        axis_pen.set_style(PenStyle::SolidLine);
        painter.as_mut().set_pen(&axis_pen);

        let y_divs = this.y_divisions.max(1) as usize;
        for j in 0..=y_divs {
            let t = j as f64 / y_divs as f64;
            let data_y_val = if this.y_log_scale {
                let log_min = y_min_val.log10();
                let log_max = y_max_val.log10();
                (10.0_f64).powf(log_min + t * (log_max - log_min))
            } else {
                y_min_val + t * (y_max_val - y_min_val)
            };

            if let Some(y_pixel) =
                self.pixel_y(data_y_val, y_min_val, y_max_val, plot_y, plot_height)
            {
                // tick mark (horizontal small line)
                let tick_len = 5.0;
                self.draw_line(painter, plot_x, y_pixel, plot_x + tick_len, y_pixel);

                // label, skip top label to avoid cut off
                if j == y_divs {
                    continue;
                }

                let label = self.format_value(data_y_val * y_mul);
                let label_str = label.to_string();
                let approx_width = label_str.len() as f64 * 7.0;
                let text_x = plot_x - approx_width - 2.0;
                let text_y = y_pixel + 4.0;
                self.draw_text(painter, text_x, text_y, &label);
            }
        }
    }

    fn draw_axis_labels(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        plot_x: f64,
        plot_y: f64,
        plot_width: f64,
        plot_height: f64,
        x_axis_y: f64,
        _y_axis_x: f64,
        x_prefix: &str,
        y_prefix: &str,
    ) {
        let this = self.rust();
        let axis_color = if this.dark_mode {
            QColor::from_rgb(255, 255, 255)
        } else {
            QColor::from_rgb(0, 0, 0)
        };

        let mut text_pen = QPen::default();
        text_pen.set_color(&axis_color);
        text_pen.set_width(0);
        text_pen.set_style(PenStyle::SolidLine);
        painter.as_mut().set_pen(&text_pen);

        // X-axis label
        if !this.x_label.to_string().is_empty() {
            let mut x_label_str = this.x_label.to_string();
            let x_unit_str = this.x_unit.to_string();
            if !x_unit_str.is_empty() {
                x_label_str = format!("{} ({}{})", x_label_str, x_prefix, x_unit_str);
            }
            let x_label_q = QString::from(x_label_str.as_str());
            let label_width = x_label_str.len() as f64 * 7.0;
            let text_x = plot_x + plot_width / 2.0 - label_width / 2.0;
            let text_y = x_axis_y + 35.0;
            self.draw_text(painter, text_x, text_y, &x_label_q);
        }

        // Y-axis label (rotated vertical)
        if !this.y_label.to_string().is_empty() {
            let mut y_label_str = this.y_label.to_string();
            let y_unit_str = this.y_unit.to_string();
            if !y_unit_str.is_empty() {
                y_label_str = format!("{} ({}{})", y_label_str, y_prefix, y_unit_str);
            }
            let y_label_q = QString::from(y_label_str.as_str());

            painter.as_mut().save();
            let center_y = plot_y + plot_height / 2.0;
            let offset = QPoint::new(15, center_y.round() as i32);
            painter.as_mut().translate(&offset);
            painter.as_mut().rotate(-90.0);
            self.draw_text(painter, 0.0, 0.0, &y_label_q);
            painter.as_mut().restore();
        }
    }

    fn draw_series_data(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        plot_x: f64,
        plot_y: f64,
        plot_width: f64,
        plot_height: f64,
        x_min_val: f64,
        x_max_val: f64,
        y_min_val: f64,
        y_max_val: f64,
    ) {
        let this = self.rust();

        for (si, s) in this.series_list.iter().enumerate() {
            if s.data_x.is_empty() {
                continue;
            }

            // Choose pen for series (color, thickness, style)
            let style_val = if s.line_style <= 0 { 1 } else { s.line_style };
            let mut pen = QPen::default();
            pen.set_color(&s.color);
            let width_i = if s.thickness <= 0.0 {
                1
            } else {
                s.thickness.round() as i32
            };
            pen.set_width(width_i);
            pen.set_style(match style_val {
                2 => PenStyle::DashLine,
                3 => PenStyle::DotLine,
                4 => PenStyle::DashDotLine,
                5 => PenStyle::DashDotDotLine,
                _ => PenStyle::SolidLine,
            });
            painter.as_mut().set_pen(&pen);

            if !this.separate_series {
                // Combined series mode
                if s.data_x.len() > 1 {
                    if s.is_digital {
                        self.draw_digital_series_combined(
                            painter,
                            s,
                            x_min_val,
                            x_max_val,
                            y_min_val,
                            y_max_val,
                            plot_x,
                            plot_y,
                            plot_width,
                            plot_height,
                        );
                    } else {
                        self.draw_analog_series_combined(
                            painter,
                            s,
                            x_min_val,
                            x_max_val,
                            y_min_val,
                            y_max_val,
                            plot_x,
                            plot_y,
                            plot_width,
                            plot_height,
                        );
                    }
                }
                if s.marker {
                    self.draw_markers_combined(
                        painter,
                        s,
                        x_min_val,
                        x_max_val,
                        y_min_val,
                        y_max_val,
                        plot_x,
                        plot_y,
                        plot_width,
                        plot_height,
                    );
                }
            } else {
                // Separate series mode
                self.draw_series_separate(
                    painter,
                    si,
                    this.series_list.len(),
                    s,
                    plot_x,
                    plot_y,
                    plot_width,
                    plot_height,
                );
            }
        }

        // Draw separators between series bands (separate series mode)
        if this.separate_series && this.series_list.len() > 1 {
            let grid_color = QColor::from_rgb(136, 136, 136);
            let n = this.series_list.len();
            let band_height = plot_height / (n as f64);
            let mut sep_pen = QPen::default();
            sep_pen.set_color(&grid_color);
            sep_pen.set_width(0);
            sep_pen.set_style(PenStyle::DashLine);
            painter.as_mut().set_pen(&sep_pen);

            for j in 1..n {
                let y_line = plot_y + band_height * (j as f64);
                self.draw_line(painter, plot_x, y_line, plot_x + plot_width, y_line);
            }
        }
    }

    fn draw_digital_series_combined(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        s: &DataSeries,
        x_min_val: f64,
        x_max_val: f64,
        y_min_val: f64,
        y_max_val: f64,
        plot_x: f64,
        plot_y: f64,
        plot_width: f64,
        plot_height: f64,
    ) {
        for k in 0..(s.data_x.len() - 1) {
            let x_curr = match self.pixel_x(s.data_x[k], x_min_val, x_max_val, plot_x, plot_width) {
                Some(v) => v,
                None => continue,
            };
            let x_next =
                match self.pixel_x(s.data_x[k + 1], x_min_val, x_max_val, plot_x, plot_width) {
                    Some(v) => v,
                    None => continue,
                };
            let y_curr = match self.pixel_y(s.data_y[k], y_min_val, y_max_val, plot_y, plot_height)
            {
                Some(v) => v,
                None => continue,
            };
            let y_next =
                match self.pixel_y(s.data_y[k + 1], y_min_val, y_max_val, plot_y, plot_height) {
                    Some(v) => v,
                    None => continue,
                };

            // horizontal line at y_curr
            self.draw_line(painter, x_curr, y_curr, x_next, y_curr);
            // vertical transition line at x_next
            if (s.data_y[k] - s.data_y[k + 1]).abs() > f64::EPSILON {
                self.draw_line(painter, x_next, y_curr, x_next, y_next);
            }
        }
    }

    fn draw_analog_series_combined(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        s: &DataSeries,
        x_min_val: f64,
        x_max_val: f64,
        y_min_val: f64,
        y_max_val: f64,
        plot_x: f64,
        plot_y: f64,
        plot_width: f64,
        plot_height: f64,
    ) {
        for k in 0..(s.data_x.len() - 1) {
            let x1 = match self.pixel_x(s.data_x[k], x_min_val, x_max_val, plot_x, plot_width) {
                Some(v) => v,
                None => continue,
            };
            let x2 = match self.pixel_x(s.data_x[k + 1], x_min_val, x_max_val, plot_x, plot_width) {
                Some(v) => v,
                None => continue,
            };
            let y1 = match self.pixel_y(s.data_y[k], y_min_val, y_max_val, plot_y, plot_height) {
                Some(v) => v,
                None => continue,
            };
            let y2 = match self.pixel_y(s.data_y[k + 1], y_min_val, y_max_val, plot_y, plot_height)
            {
                Some(v) => v,
                None => continue,
            };
            self.draw_line(painter, x1, y1, x2, y2);
        }
    }

    fn draw_markers_combined(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        s: &DataSeries,
        x_min_val: f64,
        x_max_val: f64,
        y_min_val: f64,
        y_max_val: f64,
        plot_x: f64,
        plot_y: f64,
        plot_width: f64,
        plot_height: f64,
    ) {
        let marker_size = 6.0;
        for k in 0..s.data_x.len() {
            let x_pt = match self.pixel_x(s.data_x[k], x_min_val, x_max_val, plot_x, plot_width) {
                Some(v) => v,
                None => continue,
            };
            let y_pt = match self.pixel_y(s.data_y[k], y_min_val, y_max_val, plot_y, plot_height) {
                Some(v) => v,
                None => continue,
            };
            let rect = QRectF::new(
                x_pt - marker_size / 2.0,
                y_pt - marker_size / 2.0,
                marker_size,
                marker_size,
            );
            painter.as_mut().fill_rect(&rect, &s.color);
        }
    }

    fn draw_series_separate(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        si: usize,
        n_series: usize,
        s: &DataSeries,
        plot_x: f64,
        plot_y: f64,
        plot_width: f64,
        plot_height: f64,
    ) {
        let band_height = plot_height / n_series as f64;
        let band_index = si as f64;
        let band_bottom_y = plot_y + plot_height - band_index * band_height;

        // Compute series local min/max (same logic as original paint())
        let min_val = s.data_y.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = s.data_y.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let local_min = if min_val == max_val {
            min_val - 0.5
        } else {
            min_val
        };
        let local_max = if min_val == max_val {
            max_val + 0.5
        } else {
            max_val
        };

        let state = self.rust();
        let (local_min_val, local_max_val) = if state.y_log_scale {
            if local_max <= 0.0 {
                (0.1, 1.0)
            } else {
                let loc_min = if local_min > 0.0 {
                    local_min
                } else {
                    (local_max / 1e6).max(1e-9)
                };
                (loc_min, local_max)
            }
        } else {
            (local_min, local_max)
        };

        // Use the same effective x-range logic as the original paint() (important for log scale)
        let (x_min_val, x_max_val) = self.effective_x_range();

        if s.data_x.len() > 1 {
            if s.is_digital {
                self.draw_digital_series_separate(
                    painter,
                    s,
                    local_min_val,
                    local_max_val,
                    plot_x,
                    band_bottom_y,
                    plot_width,
                    band_height,
                    x_min_val,
                    x_max_val,
                );
            } else {
                self.draw_analog_series_separate(
                    painter,
                    s,
                    local_min_val,
                    local_max_val,
                    plot_x,
                    band_bottom_y,
                    plot_width,
                    band_height,
                    x_min_val,
                    x_max_val,
                );
            }
        }

        if s.marker {
            self.draw_markers_separate(
                painter,
                s,
                local_min_val,
                local_max_val,
                plot_x,
                band_bottom_y,
                plot_width,
                band_height,
            );
        }
    }

    fn draw_digital_series_separate(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        s: &DataSeries,
        local_min_val: f64,
        local_max_val: f64,
        plot_x: f64,
        band_bottom_y: f64,
        plot_width: f64,
        band_height: f64,
        x_min_val: f64,
        x_max_val: f64,
    ) {
        for k in 0..(s.data_x.len() - 1) {
            let x_curr = match self.pixel_x(s.data_x[k], x_min_val, x_max_val, plot_x, plot_width) {
                Some(v) => v,
                None => continue,
            };
            let x_next =
                match self.pixel_x(s.data_x[k + 1], x_min_val, x_max_val, plot_x, plot_width) {
                    Some(v) => v,
                    None => continue,
                };
            let y_curr = match self.pixel_y_separate(
                s.data_y[k],
                local_min_val,
                local_max_val,
                band_bottom_y,
                band_height,
            ) {
                Some(v) => v,
                None => continue,
            };
            let y_next = match self.pixel_y_separate(
                s.data_y[k + 1],
                local_min_val,
                local_max_val,
                band_bottom_y,
                band_height,
            ) {
                Some(v) => v,
                None => continue,
            };

            self.draw_line(painter, x_curr, y_curr, x_next, y_curr);
            if (s.data_y[k] - s.data_y[k + 1]).abs() > f64::EPSILON {
                self.draw_line(painter, x_next, y_curr, x_next, y_next);
            }
        }
    }

    fn draw_analog_series_separate(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        s: &DataSeries,
        local_min_val: f64,
        local_max_val: f64,
        plot_x: f64,
        band_bottom_y: f64,
        plot_width: f64,
        band_height: f64,
        x_min_val: f64,
        x_max_val: f64,
    ) {
        for k in 0..(s.data_x.len() - 1) {
            let x1 = match self.pixel_x(s.data_x[k], x_min_val, x_max_val, plot_x, plot_width) {
                Some(v) => v,
                None => continue,
            };
            let x2 = match self.pixel_x(s.data_x[k + 1], x_min_val, x_max_val, plot_x, plot_width) {
                Some(v) => v,
                None => continue,
            };
            let y1 = match self.pixel_y_separate(
                s.data_y[k],
                local_min_val,
                local_max_val,
                band_bottom_y,
                band_height,
            ) {
                Some(v) => v,
                None => continue,
            };
            let y2 = match self.pixel_y_separate(
                s.data_y[k + 1],
                local_min_val,
                local_max_val,
                band_bottom_y,
                band_height,
            ) {
                Some(v) => v,
                None => continue,
            };

            self.draw_line(painter, x1, y1, x2, y2);
        }
    }

    fn draw_markers_separate(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        s: &DataSeries,
        local_min_val: f64,
        local_max_val: f64,
        plot_x: f64,
        band_bottom_y: f64,
        plot_width: f64,
        band_height: f64,
    ) {
        let (x_min_val, x_max_val) = self.effective_x_range();
        let marker_size = 6.0;
        for k in 0..s.data_x.len() {
            let x_pt = match self.pixel_x(s.data_x[k], x_min_val, x_max_val, plot_x, plot_width) {
                Some(v) => v,
                None => continue,
            };
            let y_pt = match self.pixel_y_separate(
                s.data_y[k],
                local_min_val,
                local_max_val,
                band_bottom_y,
                band_height,
            ) {
                Some(v) => v,
                None => continue,
            };
            let rect = QRectF::new(
                x_pt - marker_size / 2.0,
                y_pt - marker_size / 2.0,
                marker_size,
                marker_size,
            );
            painter.as_mut().fill_rect(&rect, &s.color);
        }
    }

    fn draw_legend(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        plot_x: f64,
        plot_y: f64,
        plot_width: f64,
        plot_height: f64,
    ) {
        let this = self.rust();
        if !this.legend_visible || this.series_list.is_empty() {
            return;
        }

        let axis_color = if this.dark_mode {
            QColor::from_rgb(255, 255, 255)
        } else {
            QColor::from_rgb(0, 0, 0)
        };

        let legend_padding = 4.0;
        let entry_height = 15.0;
        let mut max_text_width = 0.0;
        for s in &this.series_list {
            let w = s.name.len() as f64 * 7.0;
            if w > max_text_width {
                max_text_width = w;
            }
        }
        let legend_width = max_text_width + 20.0;
        let legend_height = this.series_list.len() as f64 * entry_height + legend_padding * 2.0;

        let (legend_x, legend_y) = match this.legend_position {
            0 => (plot_x + 5.0, plot_y + 5.0),
            1 => (plot_x + plot_width - legend_width - 5.0, plot_y + 5.0),
            2 => (plot_x + 5.0, plot_y + plot_height - legend_height - 5.0),
            3 => (
                plot_x + plot_width - legend_width - 5.0,
                plot_y + plot_height - legend_height - 5.0,
            ),
            _ => (plot_x + plot_width - legend_width - 5.0, plot_y + 5.0),
        };

        let bg_color = if this.dark_mode {
            QColor::from_rgba(30, 30, 30, 200)
        } else {
            QColor::from_rgba(255, 255, 255, 200)
        };
        let bg_rect = QRectF::new(legend_x, legend_y, legend_width, legend_height);
        painter.as_mut().fill_rect(&bg_rect, &bg_color);

        for (idx, s) in this.series_list.iter().enumerate() {
            let text = QString::from(&s.name);

            let mut legend_pen = QPen::default();
            legend_pen.set_color(&s.color);
            legend_pen.set_width(2);
            legend_pen.set_style(PenStyle::SolidLine);
            painter.as_mut().set_pen(&legend_pen);

            let line_y = legend_y + legend_padding + idx as f64 * entry_height + entry_height / 2.0;
            self.draw_line(painter, legend_x + 5.0, line_y, legend_x + 15.0, line_y);

            let mut legend_text_pen = QPen::default();
            legend_text_pen.set_color(&axis_color);
            legend_text_pen.set_width(0);
            legend_text_pen.set_style(PenStyle::SolidLine);
            painter.as_mut().set_pen(&legend_text_pen);

            self.draw_text(
                painter,
                legend_x + 20.0,
                legend_y + legend_padding + idx as f64 * entry_height + 10.0,
                &text,
            );
        }
    }

    fn draw_cursors(
        self: Pin<&Self>,
        painter: &mut Pin<&mut graph_object_qobject::QPainter>,
        plot_x: f64,
        plot_y: f64,
        plot_width: f64,
        plot_height: f64,
        x_min_val: f64,
        x_max_val: f64,
        y_min_val: f64,
        y_max_val: f64,
    ) {
        let this = self.rust();
        let axis_color = if this.dark_mode {
            QColor::from_rgb(255, 255, 255)
        } else {
            QColor::from_rgb(0, 0, 0)
        };

        let mut cursor_pen = QPen::default();
        cursor_pen.set_color(&axis_color);
        cursor_pen.set_width(1);
        cursor_pen.set_style(PenStyle::DashLine);
        painter.as_mut().set_pen(&cursor_pen);

        // Vertical cursors
        for &x_val in &this.cursor_x_positions {
            if x_val >= x_min_val && x_val <= x_max_val && (!this.x_log_scale || x_val > 0.0) {
                if let Some(x_pix) = self.pixel_x(x_val, x_min_val, x_max_val, plot_x, plot_width) {
                    self.draw_line(painter, x_pix, plot_y, x_pix, plot_y + plot_height);
                }
            }
        }

        // Horizontal cursors (only in combined mode)
        if !this.separate_series {
            for &y_val in &this.cursor_y_positions {
                if y_val >= y_min_val && y_val <= y_max_val && (!this.y_log_scale || y_val > 0.0) {
                    if let Some(y_pix) =
                        self.pixel_y(y_val, y_min_val, y_max_val, plot_y, plot_height)
                    {
                        self.draw_line(painter, plot_x, y_pix, plot_x + plot_width, y_pix);
                    }
                }
            }
        }

        // Cursor difference labels
        let mut diff_pen = QPen::default();
        diff_pen.set_color(&axis_color);
        diff_pen.set_width(0);
        diff_pen.set_style(PenStyle::SolidLine);
        painter.as_mut().set_pen(&diff_pen);

        if this.cursor_x_positions.len() == 2 {
            let x1 = this.cursor_x_positions[0];
            let x2 = this.cursor_x_positions[1];
            if !(this.x_log_scale && (x1 <= 0.0 || x2 <= 0.0)) {
                let dx = (x2 - x1).abs();
                let label = QString::from(&format!("ΔX: {}", self.format_value(dx).to_string()));
                let text_width = label.to_string().len() as f64 * 7.0;
                let text_x = plot_x + plot_width / 2.0 - text_width / 2.0;
                let text_y = plot_y + 15.0;
                self.draw_text(painter, text_x, text_y, &label);
            }
        }

        if !this.separate_series && this.cursor_y_positions.len() == 2 {
            let y1 = this.cursor_y_positions[0];
            let y2 = this.cursor_y_positions[1];
            if !(this.y_log_scale && (y1 <= 0.0 || y2 <= 0.0)) {
                let dy = (y2 - y1).abs();
                let label = QString::from(&format!("ΔY: {}", self.format_value(dy).to_string()));

                if let (Some(y1_pix), Some(y2_pix)) = (
                    self.pixel_y(y1, y_min_val, y_max_val, plot_y, plot_height),
                    self.pixel_y(y2, y_min_val, y_max_val, plot_y, plot_height),
                ) {
                    let mid_y = (y1_pix + y2_pix) / 2.0;
                    let text_y = mid_y + 4.0;
                    self.draw_text(painter, plot_x + 10.0, text_y, &label);
                }
            }
        }
    }
    unsafe fn paint(self: Pin<&mut Self>, painter: *mut graph_object_qobject::QPainter) {
        let painter = match unsafe { painter.as_mut() } {
            Some(p) => p,
            None => return,
        };
        let mut pinned_painter = unsafe { Pin::new_unchecked(painter) };

        let this = self.as_ref();
        let size = this.size();
        let width = size.width();
        let height = size.height();

        this.setup_painter(&mut pinned_painter);
        this.draw_background(&mut pinned_painter, width, height);

        let (plot_x, plot_y, plot_width, plot_height, x_axis_y, y_axis_x) =
            this.compute_plot_area(width, height);
        let (x_min_val, x_max_val) = this.effective_x_range();
        let (y_min_val, y_max_val) = this.effective_y_range();

        let x_ref = (x_max_val - x_min_val)
            .abs()
            .max(x_max_val.abs().max(x_min_val.abs()));
        let y_ref = (y_max_val - y_min_val)
            .abs()
            .max(y_max_val.abs().max(y_min_val.abs()));
        let (x_mul, x_prefix) = this.si_scale(x_ref);
        let (y_mul, y_prefix) = this.si_scale(y_ref);

        this.draw_grid_and_axes(
            &mut pinned_painter,
            plot_x,
            plot_y,
            plot_width,
            plot_height,
            x_axis_y,
            y_axis_x,
            x_min_val,
            x_max_val,
            y_min_val,
            y_max_val,
        );
        this.draw_x_axis_ticks_and_labels(
            &mut pinned_painter,
            plot_x,
            x_axis_y,
            plot_width,
            x_min_val,
            x_max_val,
            x_mul,
        );
        this.draw_y_axis_ticks_and_labels(
            &mut pinned_painter,
            plot_x,
            plot_y,
            plot_height,
            y_min_val,
            y_max_val,
            y_mul,
        );
        this.draw_axis_labels(
            &mut pinned_painter,
            plot_x,
            plot_y,
            plot_width,
            plot_height,
            x_axis_y,
            y_axis_x,
            x_prefix,
            y_prefix,
        );
        this.draw_series_data(
            &mut pinned_painter,
            plot_x,
            plot_y,
            plot_width,
            plot_height,
            x_min_val,
            x_max_val,
            y_min_val,
            y_max_val,
        );
        this.draw_legend(&mut pinned_painter, plot_x, plot_y, plot_width, plot_height);
        this.draw_cursors(
            &mut pinned_painter,
            plot_x,
            plot_y,
            plot_width,
            plot_height,
            x_min_val,
            x_max_val,
            y_min_val,
            y_max_val,
        );
    }

    pub fn save_csv(mut self: Pin<&mut Self>, file_path: &QString) {
        use std::io::Write;
        let binding = self.as_ref();
        let this = binding.rust();
        if let Ok(mut file) = std::fs::File::create(std::path::Path::new(&file_path.to_string())) {
            for (i, s) in this.series_list.iter().enumerate() {
                if i > 0 {
                    writeln!(file).ok();
                }
                let header_x = if !this.x_label.to_string().is_empty() {
                    this.x_label.to_string()
                } else {
                    "X".to_owned()
                };
                let header_y = s.name.clone();
                writeln!(file, "{},{}", header_x, header_y).ok();
                for (x, y) in s.data_x.iter().zip(&s.data_y) {
                    writeln!(file, "{:.6},{:.6}", x, y).ok();
                }
            }
        }
    }
    pub fn copy_data(mut self: Pin<&mut Self>) {
        let binding = self.as_ref();
        let this = binding.rust();
        let mut csv = String::new();
        for (i, s) in this.series_list.iter().enumerate() {
            if i > 0 {
                csv.push('\n');
            }
            let header_x = if !this.x_label.to_string().is_empty() {
                this.x_label.to_string()
            } else {
                "X".to_owned()
            };
            csv += &format!("{},{}\n", header_x, s.name);
            for (x, y) in s.data_x.iter().zip(&s.data_y) {
                csv += &format!("{:.6},{:.6}\n", x, y);
            }
        }
        let qstr = QString::from(&csv);
        self.as_mut().request_copy_data(&qstr);
    }
    pub fn copy_image(mut self: Pin<&mut Self>) {
        // QML fogja megcsinálni a grabToImage-ot (async) és clipboard-ra teszi.
        self.as_mut().request_copy_image();
    }

    pub fn save_image(mut self: Pin<&mut Self>, file_path: &graph_object_qobject::QString) {
        // új: normalizáljuk a path-ot, majd QML menti result.saveToFile(...) hívással
        let mut path = file_path.to_string();

        // FileDialog gyakran file:// URL-t ad. QML oldalon is lehet toLocalFile()-t használni,
        // de itt is kezeljük, hogy stabil legyen.
        if let Some(rest) = path.strip_prefix("file://") {
            path = rest.to_string();
            // Windows: "file:///C:/..." -> "/C:/..." (ezt javítjuk)
            if path.starts_with('/') && path.len() > 2 && path.as_bytes()[2] == b':' {
                path = path[1..].to_string();
            }
        }

        // egyszerű kiterjesztés-kezelés (ha nálad máshogy van, itt igazítsd)
        if !path.contains('.') {
            path.push_str(".png");
        }
        let qpath = graph_object_qobject::QString::from(path.as_str());
        self.as_mut().request_save_image(&qpath);
    }

    pub fn place_vertical_cursor(mut self: Pin<&mut Self>, x: f64) {
        let mut this = self.as_mut().rust_mut();
        if this.cursor_x_positions.len() < 2 {
            this.cursor_x_positions.push(x);
        } else {
            this.cursor_x_positions.clear();
            this.cursor_x_positions.push(x);
        }
        self.update();
    }
    pub fn place_horizontal_cursor(mut self: Pin<&mut Self>, y: f64) {
        let mut this = self.as_mut().rust_mut();
        if this.cursor_y_positions.len() < 2 {
            this.cursor_y_positions.push(y);
        } else {
            this.cursor_y_positions.clear();
            this.cursor_y_positions.push(y);
        }
        self.update();
    }
    pub fn clear_cursors(mut self: Pin<&mut Self>) {
        let mut this = self.as_mut().rust_mut();
        this.cursor_x_positions.clear();
        this.cursor_y_positions.clear();
        self.update();
    }

    pub fn request_repaint(mut self: Pin<&mut Self>) {
        self.update();
    }

    pub fn load_oscilloscope_data(mut self: Pin<&mut Self>, channel: i32) {
        let chan: u8 = if channel < 1 {
            1
        } else if channel > 4 {
            4
        } else {
            channel as u8
        };
        let wf = match oscillo_data_provider::fetch_waveform_from_env(chan) {
            Ok(w) => w,
            Err(_) => {
                self.update();
                return;
            }
        };
        if self.as_ref().rust().bode_mode {
            self.as_mut().apply_waveform_bode(chan, wf);
            return;
        }
        let mode = { self.as_ref().rust().mode };
        if mode == 2 {
            // Triggered mode: replace capture directly
            self.as_mut().apply_waveform(chan, wf);
            {
                let mut this = self.as_mut().rust_mut();
                this.live_channel = chan;
                if !this.initial_x_set && !this.series_list.is_empty() {
                    this.initial_x_set = true;
                }
            }
            return;
        }
        // For compress and scroll modes, enqueue data and reuse pumpLive logic
        {
            let mut this = self.as_mut().rust_mut();
            this.live_channel = chan;
        }
        let latest = { self.as_ref().rust().live_latest.clone() };
        if let Ok(mut lock) = latest.lock() {
            *lock = Some(wf);
        }
        self.as_mut().pump_live();
    }
    fn apply_waveform(mut self: Pin<&mut Self>, chan: u8, wf: oscillo_data_provider::Waveform) {
        let series_name = format!("C{}", chan);
        let color = match chan {
            1 => QColor::from_rgb(255, 255, 0), // yellow
            2 => QColor::from_rgb(0, 255, 255), // cyan
            3 => QColor::from_rgb(255, 0, 255), // magenta
            4 => QColor::from_rgb(0, 255, 0),   // green
            _ => QColor::from_rgb(255, 255, 255),
        };

        let q_x_label = QString::from(wf.x_label.as_str()); // pl. "Time"
        let q_y_label = QString::from(wf.y_label.as_str()); // pl. "C1"
        let q_x_unit = QString::from(wf.x_unit.as_str()); // pl. "s"
        let q_y_unit = QString::from(wf.y_unit.as_str()); // pl. "V"

        let mut x_range_update: Option<(f64, f64)> = None;
        let mut y_range_update: Option<(f64, f64)> = None;

        {
            let mut this = self.as_mut().rust_mut();

            // update labels + units (paint fogja összeállítani az SI-prefixes feliratot)
            this.x_label = q_x_label.clone();
            this.y_label = q_y_label.clone();
            this.x_unit = q_x_unit.clone();
            this.y_unit = q_y_unit.clone();

            // Replace or create series
            let idx = match this.series_list.iter().position(|s| s.name == series_name) {
                Some(i) => i,
                None => {
                    this.series_list.push(DataSeries {
                        name: series_name.clone(),
                        is_digital: false,
                        color: color.clone(),
                        thickness: 2.0,
                        line_style: 1,
                        marker: false,
                        data_x: Vec::new(),
                        data_y: Vec::new(),
                        min_y: 0.0,
                        max_y: 0.0,
                    });
                    this.series_list.len() - 1
                }
            };

            {
                let s = &mut this.series_list[idx];
                s.is_digital = false;
                s.color = color;
                s.thickness = 2.0;
                s.line_style = 1;
                s.marker = false;
                s.data_x = wf.x;
                s.data_y = wf.y;

                if s.data_y.is_empty() {
                    s.min_y = 0.0;
                    s.max_y = 1.0;
                } else {
                    let mut mn = f64::INFINITY;
                    let mut mx = f64::NEG_INFINITY;
                    for &v in &s.data_y {
                        if v < mn {
                            mn = v;
                        }
                        if v > mx {
                            mx = v;
                        }
                    }
                    if !mn.is_finite() || !mx.is_finite() {
                        s.min_y = 0.0;
                        s.max_y = 1.0;
                    } else if (mn - mx).abs() < f64::EPSILON {
                        s.min_y = mn - 0.5;
                        s.max_y = mx + 0.5;
                    } else {
                        s.min_y = mn;
                        s.max_y = mx;
                    }
                }
            }
            // global bounds
            let mut xmin_all = f64::INFINITY;
            let mut xmax_all = f64::NEG_INFINITY;
            let mut ymin_all = f64::INFINITY;
            let mut ymax_all = f64::NEG_INFINITY;
            for s2 in &this.series_list {
                if let (Some(first), Some(last)) = (s2.data_x.first(), s2.data_x.last()) {
                    if *first < xmin_all {
                        xmin_all = *first;
                    }
                    if *last > xmax_all {
                        xmax_all = *last;
                    }
                }
                if !s2.data_y.is_empty() {
                    if s2.min_y < ymin_all {
                        ymin_all = s2.min_y;
                    }
                    if s2.max_y > ymax_all {
                        ymax_all = s2.max_y;
                    }
                }
            }
            if !xmin_all.is_finite() || !xmax_all.is_finite() {
                xmin_all = 0.0;
                xmax_all = 1.0;
            }
            if !ymin_all.is_finite() || !ymax_all.is_finite() {
                ymin_all = 0.0;
                ymax_all = 1.0;
            }
            if (xmin_all - xmax_all).abs() < f64::EPSILON {
                xmax_all = xmin_all + 1.0;
            }
            if (ymin_all - ymax_all).abs() < f64::EPSILON {
                ymax_all = ymin_all + 1.0;
            }

            // IMPORTANT: ne kapcsoljuk vissza erőből az auto-range-et,
            // így a user zoom/pan viewportja megmarad (ngscopeclient-szerűen)
            if this.x_auto_range {
                this.x_min = xmin_all;
                this.x_max = xmax_all;
                x_range_update = Some((xmin_all, xmax_all));
            }
            if this.y_auto_range {
                this.y_min = ymin_all;
                this.y_max = ymax_all;
                y_range_update = Some((ymin_all, ymax_all));
            }
        }

        if let Some((xmin, xmax)) = x_range_update {
            self.as_mut().set_x_min(xmin);
            self.as_mut().set_x_max(xmax);
        }
        if let Some((ymin, ymax)) = y_range_update {
            self.as_mut().set_y_min(ymin);
            self.as_mut().set_y_max(ymax);
        }
        self.update();
    }

    fn apply_waveform_bode(
        mut self: Pin<&mut Self>,
        chan: u8,
        wf: oscillo_data_provider::Waveform,
    ) {
        let mut this = self.as_mut().rust_mut();
        // Clear existing series and configure axes for Bode plot
        this.series_list.clear();
        this.x_label = QString::from("Frequency");
        this.y_label = QString::from("");
        this.x_unit = QString::from("Hz");
        this.y_unit = QString::from("");
        this.separate_series = true;
        // Compute FFT of waveform data (magnitude and phase)
        let n = wf.y.len();
        if n < 2 {
            return;
        }
        // Apply Hann window to reduce spectral leakage
        let mut windowed: Vec<f64> =
            wf.y.iter()
                .enumerate()
                .map(|(i, &val)| {
                    let w = 0.5
                        * (1.0 - (2.0 * std::f64::consts::PI * i as f64 / (n - 1) as f64).cos());
                    val * w
                })
                .collect();
        // Perform FFT on windowed data
        let mut planner = FftPlanner::<f64>::new();
        let fft = planner.plan_fft_forward(n);
        let mut buffer: Vec<Complex<f64>> = windowed
            .into_iter()
            .map(|v| Complex { re: v, im: 0.0 })
            .collect();
        fft.process(&mut buffer);
        // Prepare frequency axis values (skip DC = 0 Hz)
        let dt = if wf.x.len() > 1 {
            wf.x[1] - wf.x[0]
        } else {
            1.0
        };
        let fs = if dt > 0.0 { 1.0 / dt } else { 0.0 };
        let half = n / 2;
        let out_len = half + 1;
        let include_nyquist = n % 2 == 0;
        let mut freq_vals = Vec::with_capacity(out_len - 1);
        let mut mag_vals = Vec::with_capacity(out_len - 1);
        let mut phase_vals = Vec::with_capacity(out_len - 1);
        for i in 1..out_len {
            let val = buffer[i];
            // One-sided magnitude (normalize FFT, double non-DC/non-Nyquist)
            let mag = if include_nyquist && i == half {
                (val.re * val.re + val.im * val.im).sqrt() / n as f64
            } else {
                (val.re * val.re + val.im * val.im).sqrt() * 2.0 / n as f64
            };
            let phase = val.im.atan2(val.re) * 180.0 / std::f64::consts::PI;
            freq_vals.push(i as f64 * fs / n as f64);
            mag_vals.push(mag);
            phase_vals.push(phase);
        }
        // Auto-range frequency axis
        let fmin = freq_vals.first().copied().unwrap_or(0.0);
        let fmax = freq_vals.last().copied().unwrap_or(fmin);
        if this.x_auto_range {
            this.x_min = fmin;
            this.x_max = fmax;
            self.as_mut().set_x_min(fmin);
            self.as_mut().set_x_max(fmax);
        }
        if this.y_auto_range {
            this.y_min = 0.0;
            this.y_max = 1.0;
            self.as_mut().set_y_min(0.0);
            self.as_mut().set_y_max(1.0);
        }
        // Create magnitude series (in Volts)
        let series_name_mag = format!("C{} Mag (V)", chan);
        let color_mag = match chan {
            1 => QColor::from_rgb(255, 255, 0),
            2 => QColor::from_rgb(0, 255, 255),
            3 => QColor::from_rgb(255, 0, 255),
            4 => QColor::from_rgb(0, 255, 0),
            _ => QColor::from_rgb(255, 255, 255),
        };
        this.series_list.push(DataSeries {
            name: series_name_mag,
            is_digital: false,
            color: color_mag,
            thickness: 2.0,
            line_style: 1,
            marker: false,
            data_x: freq_vals.clone(),
            data_y: mag_vals,
            min_y: 0.0,
            max_y: 0.0,
        });
        // Compute min/max for magnitude series
        if let Some(s) = this.series_list.last_mut() {
            if s.data_y.is_empty() {
                s.min_y = 0.0;
                s.max_y = 1.0;
            } else {
                let (minv, maxv) = s
                    .data_y
                    .iter()
                    .fold((f64::INFINITY, f64::NEG_INFINITY), |(minv, maxv), &v| {
                        (v.min(minv), v.max(maxv))
                    });
                if !minv.is_finite() || !maxv.is_finite() {
                    s.min_y = 0.0;
                    s.max_y = 1.0;
                } else if (maxv - minv).abs() < f64::EPSILON {
                    s.min_y = minv - 0.5;
                    s.max_y = maxv + 0.5;
                } else {
                    s.min_y = minv;
                    s.max_y = maxv;
                }
            }
        }
        // Create phase series (in degrees)
        let series_name_phase = format!("C{} Phase (deg)", chan);
        let phase_color = if this.dark_mode {
            QColor::from_rgb(255, 255, 255)
        } else {
            QColor::from_rgb(0, 0, 0)
        };
        this.series_list.push(DataSeries {
            name: series_name_phase,
            is_digital: false,
            color: phase_color,
            thickness: 2.0,
            line_style: 2, // dashed line style
            marker: false,
            data_x: freq_vals,
            data_y: phase_vals,
            min_y: -180.0,
            max_y: 180.0,
        });
        // Trigger repaint
        self.update();
    }
    pub fn start_live(mut self: Pin<&mut Self>, channel: i32, period_ms: i32) {
        self.as_mut().stop_live();
        let chan: u8 = channel.clamp(1, 4) as u8;
        let period = period_ms.max(20) as u64;

        let latest = { self.as_ref().rust().live_latest.clone() };
        let stop = Arc::new(AtomicBool::new(false));
        let stop_th = stop.clone();

        let handle = thread::spawn(move || {
            while !stop_th.load(Ordering::Relaxed) {
                if let Ok(wf) = oscillo_data_provider::fetch_waveform_from_env(chan) {
                    if let Ok(mut lock) = latest.lock() {
                        *lock = Some(wf);
                    }
                }
                thread::sleep(Duration::from_millis(period));
            }
        });

        {
            let mut this = self.as_mut().rust_mut();
            this.live_channel = chan;
            this.live_period_ms = period;
            this.live_stop = stop;
            this.live_thread = Some(handle);
        }
    }

    pub fn stop_live(mut self: Pin<&mut Self>) {
        let (stop, handle) = {
            let mut this = self.as_mut().rust_mut();
            let stop = this.live_stop.clone();
            let handle = this.live_thread.take();
            (stop, handle)
        };
        stop.store(true, Ordering::Relaxed);
        if let Some(h) = handle {
            let _ = h.join();
        }
    }

    pub fn pump_live(mut self: Pin<&mut Self>) {
        let mode = { self.as_ref().rust().mode };
        let latest = { self.as_ref().rust().live_latest.clone() };
        let wf_opt = match latest.lock() {
            Ok(mut lock) => lock.take(),
            Err(_) => None,
        };

        let Some(wf) = wf_opt else {
            return;
        };

        let chan = { self.as_ref().rust().live_channel };
        if self.as_ref().rust().bode_mode {
            self.as_mut().apply_waveform_bode(chan, wf);
            {
                let mut this = self.as_mut().rust_mut();
                if !this.initial_x_set && !this.series_list.is_empty() {
                    this.initial_x_set = true;
                }
            }
            return;
        }
        if mode == 2 {
            self.as_mut().apply_waveform(chan, wf);
            {
                let mut this = self.as_mut().rust_mut();
                if !this.initial_x_set && !this.series_list.is_empty() {
                    this.initial_x_set = true;
                }
            }
            return;
        }

        let mut x_range_update: Option<(f64, f64)> = None;
        let mut y_range_update: Option<(f64, f64)> = None;

        {
            let mut this = self.as_mut().rust_mut();

            this.x_label = QString::from(wf.x_label.as_str());
            this.y_label = QString::from(wf.y_label.as_str());
            this.x_unit = QString::from(wf.x_unit.as_str());
            this.y_unit = QString::from(wf.y_unit.as_str());

            let series_name = format!("C{}", chan);
            let color = match chan {
                1 => QColor::from_rgb(255, 255, 0),
                2 => QColor::from_rgb(0, 255, 255),
                3 => QColor::from_rgb(255, 0, 255),
                4 => QColor::from_rgb(0, 255, 0),
                _ => QColor::from_rgb(255, 255, 255),
            };

            let idx = match this.series_list.iter().position(|s| s.name == series_name) {
                Some(i) => i,
                None => {
                    this.series_list.push(DataSeries {
                        name: series_name.clone(),
                        is_digital: false,
                        color: color.clone(),
                        thickness: 2.0,
                        line_style: 1,
                        marker: false,
                        data_x: Vec::new(),
                        data_y: Vec::new(),
                        min_y: 0.0,
                        max_y: 0.0,
                    });
                    this.series_list.len() - 1
                }
            };

            let mut new_x = wf.x;
            let new_y = wf.y;

            if !new_x.is_empty() {
                let first_new_x = new_x[0];
                let dt = if new_x.len() > 1 {
                    new_x[1] - new_x[0]
                } else {
                    0.0
                };

                let last_old_x = this.series_list[idx].data_x.last().copied();
                let mut offset = 0.0;

                if let Some(last_old_x) = last_old_x {
                    if first_new_x <= last_old_x {
                        offset = last_old_x + dt - first_new_x;
                    }
                } else {
                    let mut global_last: Option<f64> = None;
                    for (i, s) in this.series_list.iter().enumerate() {
                        if i == idx {
                            continue;
                        }
                        if let Some(&lx) = s.data_x.last() {
                            global_last = Some(match global_last {
                                Some(g) => g.max(lx),
                                None => lx,
                            });
                        }
                    }
                    if let Some(gl) = global_last {
                        if first_new_x <= gl {
                            offset = gl + dt - first_new_x;
                        }
                    }
                }

                if offset.is_finite() && offset != 0.0 {
                    for xv in &mut new_x {
                        *xv += offset;
                    }
                }
            }

            let mut batch_min = f64::INFINITY;
            let mut batch_max = f64::NEG_INFINITY;
            for &v in &new_y {
                if v < batch_min {
                    batch_min = v;
                }
                if v > batch_max {
                    batch_max = v;
                }
            }

            {
                let series = &mut this.series_list[idx];
                let old_len = series.data_x.len();

                series.is_digital = false;
                series.color = color;
                series.thickness = 2.0;
                series.line_style = 1;
                series.marker = false;

                if old_len == 0 {
                    series.data_x = new_x;
                    series.data_y = new_y;
                    if batch_min.is_finite() && batch_max.is_finite() {
                        series.min_y = batch_min;
                        series.max_y = batch_max;
                    } else {
                        series.min_y = 0.0;
                        series.max_y = 0.0;
                    }
                } else {
                    series.data_x.extend(new_x);
                    series.data_y.extend(new_y);
                    if batch_min.is_finite() && batch_max.is_finite() {
                        if batch_min < series.min_y {
                            series.min_y = batch_min;
                        }
                        if batch_max > series.max_y {
                            series.max_y = batch_max;
                        }
                    }
                }

                if series.data_y.is_empty() {
                    series.min_y = 0.0;
                    series.max_y = 0.0;
                }
            }

            if mode == 1 {
                let buf = this.buffer_size.max(1) as usize;
                for s2 in this.series_list.iter_mut() {
                    if s2.data_x.len() > buf {
                        let drop_count = s2.data_x.len() - buf;
                        s2.data_x.drain(0..drop_count);
                        s2.data_y.drain(0..drop_count);

                        if s2.data_y.is_empty() {
                            s2.min_y = 0.0;
                            s2.max_y = 0.0;
                        } else {
                            let mut s_min = f64::INFINITY;
                            let mut s_max = f64::NEG_INFINITY;
                            for &vy in &s2.data_y {
                                if vy < s_min {
                                    s_min = vy;
                                }
                                if vy > s_max {
                                    s_max = vy;
                                }
                            }
                            if !s_min.is_finite() || !s_max.is_finite() {
                                s2.min_y = 0.0;
                                s2.max_y = 1.0;
                            } else {
                                s2.min_y = s_min;
                                s2.max_y = s_max;
                            }
                        }
                    }
                }
            }

            if this.x_auto_range {
                let mut xmin_all = f64::INFINITY;
                let mut xmax_all = f64::NEG_INFINITY;
                for s2 in &this.series_list {
                    if let (Some(&first), Some(&last)) = (s2.data_x.first(), s2.data_x.last()) {
                        if first < xmin_all {
                            xmin_all = first;
                        }
                        if last > xmax_all {
                            xmax_all = last;
                        }
                    }
                }
                if !xmin_all.is_finite() || !xmax_all.is_finite() {
                    xmin_all = 0.0;
                    xmax_all = 1.0;
                } else if (xmin_all - xmax_all).abs() < f64::EPSILON {
                    xmax_all = xmin_all + 1.0;
                }

                match mode {
                    0 => {
                        if !this.initial_x_set {
                            this.initial_x_set = true;
                            this.x_min = xmin_all;
                        } else if xmin_all < this.x_min {
                            this.x_min = xmin_all;
                        }
                        this.x_max = xmax_all;
                    }
                    1 => {
                        this.initial_x_set = true;
                        this.x_min = xmin_all;
                        this.x_max = xmax_all;
                    }
                    _ => {}
                }
                x_range_update = Some((this.x_min, this.x_max));
            }

            if this.y_auto_range {
                let mut ymin_all = f64::INFINITY;
                let mut ymax_all = f64::NEG_INFINITY;
                for s2 in &this.series_list {
                    if !s2.data_y.is_empty() {
                        if s2.min_y < ymin_all {
                            ymin_all = s2.min_y;
                        }
                        if s2.max_y > ymax_all {
                            ymax_all = s2.max_y;
                        }
                    }
                }
                if !ymin_all.is_finite() || !ymax_all.is_finite() {
                    ymin_all = 0.0;
                    ymax_all = 1.0;
                } else if (ymin_all - ymax_all).abs() < f64::EPSILON {
                    ymax_all = ymin_all + 1.0;
                }
                this.y_min = ymin_all;
                this.y_max = ymax_all;
                y_range_update = Some((ymin_all, ymax_all));
            }
        }

        if let Some((xmin, xmax)) = x_range_update {
            self.as_mut().set_x_min(xmin);
            self.as_mut().set_x_max(xmax);
        }
        if let Some((ymin, ymax)) = y_range_update {
            self.as_mut().set_y_min(ymin);
            self.as_mut().set_y_max(ymax);
        }
        self.update();
    }
    pub fn add_series(
        mut self: Pin<&mut Self>,
        name: &QString,
        series_type: i32,
        color: &QColor,
        thickness: f64,
        line_style: i32,
        marker: bool,
    ) {
        let name_str = name.to_string();

        // E0499: ne hívjunk set_* metódusokat miközben él a rust_mut() kölcsönzés
        let mut y_range_update: Option<(f64, f64)> = None;
        let mut x_range_update: Option<(f64, f64)> = None;

        {
            let mut this = self.as_mut().rust_mut();

            // Ensure unique name by removing any existing series with same name
            this.series_list.retain(|s| s.name != name_str);
            let is_digital = series_type == 2; // assume 0=analog, 1=int (treat as analog), 2=bool digital
            let series = DataSeries {
                name: name_str.clone(),
                is_digital,
                color: color.clone(),
                thickness: if thickness <= 0.0 { 1.0 } else { thickness },
                line_style: if line_style < 0 { 1 } else { line_style },
                marker,
                data_x: Vec::new(),
                data_y: Vec::new(),
                min_y: 0.0,
                max_y: 0.0,
            };
            this.series_list.push(series);
            // Update Y auto-range if enabled
            if this.y_auto_range {
                let mut min_y = f64::INFINITY;
                let mut max_y = f64::NEG_INFINITY;
                for s in &this.series_list {
                    if !s.data_y.is_empty() {
                        if s.min_y < min_y {
                            min_y = s.min_y;
                        }
                        if s.max_y > max_y {
                            max_y = s.max_y;
                        }
                    }
                }
                if !min_y.is_finite() {
                    min_y = 0.0;
                    max_y = 1.0;
                } else if (min_y - max_y).abs() < f64::EPSILON {
                    max_y = min_y + 1.0;
                }
                this.y_min = min_y;
                this.y_max = max_y;
                y_range_update = Some((min_y, max_y));
            }

            // If adding first series with no data, set some defaults for x axis (0..1)
            if this.x_auto_range
                && this.series_list.len() == 1
                && this.series_list[0].data_x.is_empty()
            {
                this.x_min = 0.0;
                this.x_max = 1.0;
                x_range_update = Some((0.0, 1.0));
            }
        }
        if let Some((ymin, ymax)) = y_range_update {
            self.as_mut().set_y_min(ymin);
            self.as_mut().set_y_max(ymax);
        }
        if let Some((xmin, xmax)) = x_range_update {
            self.as_mut().set_x_min(xmin);
            self.as_mut().set_x_max(xmax);
        }

        self.update();
    }

    pub fn remove_series(mut self: Pin<&mut Self>, name: &QString) {
        let name_str = name.to_string();

        let mut x_range_update: Option<(f64, f64)> = None;
        let mut y_range_update: Option<(f64, f64)> = None;

        {
            let mut this = self.as_mut().rust_mut();
            this.series_list.retain(|s| s.name != name_str);

            // Recompute axes if needed
            if this.x_auto_range || this.y_auto_range {
                let mut new_x_min = f64::INFINITY;
                let mut new_x_max = f64::NEG_INFINITY;
                let mut new_y_min = f64::INFINITY;
                let mut new_y_max = f64::NEG_INFINITY;
                for s in &this.series_list {
                    if let (Some(first), Some(last)) = (s.data_x.first(), s.data_x.last()) {
                        if *first < new_x_min {
                            new_x_min = *first;
                        }
                        if *last > new_x_max {
                            new_x_max = *last;
                        }
                    }
                    if !s.data_y.is_empty() {
                        if s.min_y < new_y_min {
                            new_y_min = s.min_y;
                        }
                        if s.max_y > new_y_max {
                            new_y_max = s.max_y;
                        }
                    }
                }
                if !new_x_min.is_finite() || !new_x_max.is_finite() {
                    new_x_min = 0.0;
                    new_x_max = 1.0;
                } else if (new_x_min - new_x_max).abs() < f64::EPSILON {
                    new_x_max = new_x_min + 1.0;
                }

                if !new_y_min.is_finite() || !new_y_max.is_finite() {
                    new_y_min = 0.0;
                    new_y_max = 1.0;
                } else if (new_y_min - new_y_max).abs() < f64::EPSILON {
                    new_y_max = new_y_min + 1.0;
                }

                if this.x_auto_range {
                    this.x_min = new_x_min;
                    this.x_max = new_x_max;
                    x_range_update = Some((new_x_min, new_x_max));
                }
                if this.y_auto_range {
                    this.y_min = new_y_min;
                    this.y_max = new_y_max;
                    y_range_update = Some((new_y_min, new_y_max));
                }
            }
        }
        if let Some((xmin, xmax)) = x_range_update {
            self.as_mut().set_x_min(xmin);
            self.as_mut().set_x_max(xmax);
        }
        if let Some((ymin, ymax)) = y_range_update {
            self.as_mut().set_y_min(ymin);
            self.as_mut().set_y_max(ymax);
        }

        self.update();
    }

    pub fn add_data_point(mut self: Pin<&mut Self>, series_name: &QString, x: f64, y: f64) {
        let series_name_str = series_name.to_string();

        // E0499/E0502: a rust_mut() kölcsönzés alatt ne hívjunk set_* metódusokat és ne tartsunk
        // élő mutable borrow-ot egyetlen series-re, miközben a teljes listát bejárjuk.
        let mut x_range_update: Option<(f64, f64)> = None;
        let mut y_range_update: Option<(f64, f64)> = None;

        {
            let mut this = self.as_mut().rust_mut();

            let idx = match this
                .series_list
                .iter()
                .position(|s| s.name == series_name_str)
            {
                Some(i) => i,
                None => return,
            };

            let px = x;
            let is_digital = this.series_list[idx].is_digital;
            let py = if is_digital {
                if y > 0.5 { 1.0 } else { 0.0 }
            } else {
                y
            };

            // Push the new point
            {
                let series = &mut this.series_list[idx];
                series.data_x.push(px);
                series.data_y.push(py);

                if series.data_y.len() == 1 {
                    series.min_y = py;
                    series.max_y = py;
                } else {
                    if py < series.min_y {
                        series.min_y = py;
                    }
                    if py > series.max_y {
                        series.max_y = py;
                    }
                }
            }

            // X auto range + modes
            if this.x_auto_range {
                if !this.initial_x_set {
                    this.initial_x_set = true;
                    this.x_min = px;
                }

                match this.mode {
                    // Mode 0: compress
                    0 => {
                        if px > this.x_max {
                            this.x_max = px;
                        }
                        if px < this.x_min {
                            this.x_min = px;
                        }
                    }
                    // Mode 1: scroll (keep last buffer_size points)
                    1 => {
                        let buf = this.buffer_size.max(1) as usize;

                        // Trim every series to the last `buf` points
                        for s2 in this.series_list.iter_mut() {
                            if s2.data_x.len() > buf {
                                let drop = s2.data_x.len() - buf;
                                s2.data_x.drain(0..drop);
                                s2.data_y.drain(0..drop);

                                // recompute per-series min/max (dropped points could invalidate cached min/max)
                                if s2.data_y.is_empty() {
                                    s2.min_y = 0.0;
                                    s2.max_y = 0.0;
                                } else {
                                    let mut s_min = f64::INFINITY;
                                    let mut s_max = f64::NEG_INFINITY;
                                    for &vy in &s2.data_y {
                                        if vy < s_min {
                                            s_min = vy;
                                        }
                                        if vy > s_max {
                                            s_max = vy;
                                        }
                                    }
                                    s2.min_y = s_min;
                                    s2.max_y = s_max;
                                }
                            }
                        }

                        // Recompute global x range
                        let mut xmin_all = f64::INFINITY;
                        let mut xmax_all = f64::NEG_INFINITY;
                        for s2 in &this.series_list {
                            if let (Some(first), Some(last)) = (s2.data_x.first(), s2.data_x.last())
                            {
                                if *first < xmin_all {
                                    xmin_all = *first;
                                }
                                if *last > xmax_all {
                                    xmax_all = *last;
                                }
                            }
                        }
                        if !xmin_all.is_finite() {
                            xmin_all = this.x_min;
                        }
                        if !xmax_all.is_finite() {
                            xmax_all = this.x_max;
                        }

                        this.x_min = xmin_all;
                        this.x_max = xmax_all;
                    }
                    // Mode 2: triggered
                    2 => {
                        let buf = this.buffer_size.max(1) as usize;
                        let series_len = this.series_list[idx].data_x.len();

                        if series_len > buf {
                            // span from the (overflowing) frame
                            let first_x = *this.series_list[idx].data_x.first().unwrap_or(&px);
                            let last_x = *this.series_list[idx].data_x.last().unwrap_or(&px);
                            this.last_frame_span = Some(last_x - first_x);

                            // clear all series
                            for s2 in this.series_list.iter_mut() {
                                s2.data_x.clear();
                                s2.data_y.clear();
                                s2.min_y = 0.0;
                                s2.max_y = 0.0;
                            }

                            // start new frame with current point
                            {
                                let series = &mut this.series_list[idx];
                                series.data_x.push(px);
                                series.data_y.push(py);
                                series.min_y = py;
                                series.max_y = py;
                            }
                            let span = this.last_frame_span.unwrap_or(this.buffer_size as f64);
                            this.x_min = px;
                            this.x_max = px + span;
                        } else {
                            if px > this.x_max || this.x_min > this.x_max {
                                this.x_max = px;
                                if !this.initial_x_set {
                                    this.initial_x_set = true;
                                    this.x_min = px;
                                }
                            }
                        }
                    }
                    _ => {}
                }

                x_range_update = Some((this.x_min, this.x_max));
            }
            // Y auto range
            if this.y_auto_range {
                let mut new_y_min = f64::INFINITY;
                let mut new_y_max = f64::NEG_INFINITY;
                for s2 in &this.series_list {
                    if !s2.data_y.is_empty() {
                        if s2.min_y < new_y_min {
                            new_y_min = s2.min_y;
                        }
                        if s2.max_y > new_y_max {
                            new_y_max = s2.max_y;
                        }
                    }
                }
                if !new_y_min.is_finite() {
                    new_y_min = 0.0;
                    new_y_max = 1.0;
                } else if (new_y_min - new_y_max).abs() < f64::EPSILON {
                    new_y_max = new_y_min + 1.0;
                }
                this.y_min = new_y_min;
                this.y_max = new_y_max;
                y_range_update = Some((new_y_min, new_y_max));
            }
        }

        if let Some((xmin, xmax)) = x_range_update {
            self.as_mut().set_x_min(xmin);
            self.as_mut().set_x_max(xmax);
        }
        if let Some((ymin, ymax)) = y_range_update {
            self.as_mut().set_y_min(ymin);
            self.as_mut().set_y_max(ymax);
        }
        self.update();
    }

    pub fn zoom_to_region(mut self: Pin<&mut Self>, x1: f64, x2: f64, y1: f64, y2: f64) {
        let mut x_update: Option<(f64, f64)> = None;
        let mut y_update: Option<(f64, f64)> = None;
        let mut disable_x_auto = false;
        let mut disable_y_auto = false;

        {
            let mut this = self.as_mut().rust_mut();
            if x2 != x1 {
                disable_x_auto = true;
                this.x_auto_range = false;
                this.x_min = x1.min(x2);
                this.x_max = x1.max(x2);
                x_update = Some((this.x_min, this.x_max));
            }
            if y2 != y1 {
                disable_y_auto = true;
                this.y_auto_range = false;
                this.y_min = y1.min(y2);
                this.y_max = y1.max(y2);
                y_update = Some((this.y_min, this.y_max));
            }
        }

        if disable_x_auto {
            self.as_mut().set_x_auto_range(false);
        }
        if let Some((xmin, xmax)) = x_update {
            self.as_mut().set_x_min(xmin);
            self.as_mut().set_x_max(xmax);
        }

        if disable_y_auto {
            self.as_mut().set_y_auto_range(false);
        }
        if let Some((ymin, ymax)) = y_update {
            self.as_mut().set_y_min(ymin);
            self.as_mut().set_y_max(ymax);
        }

        self.update();
    }

    pub fn zoom_x(mut self: Pin<&mut Self>, x1: f64, x2: f64) {
        let mut x_update: Option<(f64, f64)> = None;
        {
            let mut this = self.as_mut().rust_mut();
            if x2 != x1 {
                this.x_auto_range = false;
                this.x_min = x1.min(x2);
                this.x_max = x1.max(x2);
                x_update = Some((this.x_min, this.x_max));
            }
        }

        if let Some((xmin, xmax)) = x_update {
            self.as_mut().set_x_auto_range(false);
            self.as_mut().set_x_min(xmin);
            self.as_mut().set_x_max(xmax);
        }
        self.update();
    }

    pub fn zoom_y(mut self: Pin<&mut Self>, y1: f64, y2: f64) {
        let mut y_update: Option<(f64, f64)> = None;
        {
            let mut this = self.as_mut().rust_mut();
            if y2 != y1 {
                this.y_auto_range = false;
                this.y_min = y1.min(y2);
                this.y_max = y1.max(y2);
                y_update = Some((this.y_min, this.y_max));
            }
        }
        if let Some((ymin, ymax)) = y_update {
            self.as_mut().set_y_auto_range(false);
            self.as_mut().set_y_min(ymin);
            self.as_mut().set_y_max(ymax);
        }
        self.update();
    }

    pub fn zoom_at_point(mut self: Pin<&mut Self>, center_x: f64, center_y: f64, factor: f64) {
        if factor <= 0.0 {
            return;
        }
        let mut disable_auto = false;
        let mut new_ranges: Option<(f64, f64, f64, f64)> = None;

        {
            let mut this = self.as_mut().rust_mut();
            if this.x_auto_range || this.y_auto_range {
                this.x_auto_range = false;
                this.y_auto_range = false;
                disable_auto = true;
            }

            let old_x_min = this.x_min;
            let old_x_max = this.x_max;
            let old_y_min = this.y_min;
            let old_y_max = this.y_max;

            this.x_min = center_x - (center_x - old_x_min) / factor;
            this.x_max = center_x + (old_x_max - center_x) / factor;
            this.y_min = center_y - (center_y - old_y_min) / factor;
            this.y_max = center_y + (old_y_max - center_y) / factor;

            new_ranges = Some((this.x_min, this.x_max, this.y_min, this.y_max));
        }

        if disable_auto {
            self.as_mut().set_x_auto_range(false);
            self.as_mut().set_y_auto_range(false);
        }
        if let Some((xmin, xmax, ymin, ymax)) = new_ranges {
            self.as_mut().set_x_min(xmin);
            self.as_mut().set_x_max(xmax);
            self.as_mut().set_y_min(ymin);
            self.as_mut().set_y_max(ymax);
        }
        self.update();
    }

    pub fn pan(mut self: Pin<&mut Self>, delta_x: f64, delta_y: f64) {
        let mut disable_auto = false;
        let mut new_ranges: Option<(f64, f64, f64, f64)> = None;

        {
            let mut this = self.as_mut().rust_mut();
            if this.x_auto_range || this.y_auto_range {
                this.x_auto_range = false;
                this.y_auto_range = false;
                disable_auto = true;
            }

            this.x_min += delta_x;
            this.x_max += delta_x;
            this.y_min += delta_y;
            this.y_max += delta_y;
            new_ranges = Some((this.x_min, this.x_max, this.y_min, this.y_max));
        }

        if disable_auto {
            self.as_mut().set_x_auto_range(false);
            self.as_mut().set_y_auto_range(false);
        }
        if let Some((xmin, xmax, ymin, ymax)) = new_ranges {
            self.as_mut().set_x_min(xmin);
            self.as_mut().set_x_max(xmax);
            self.as_mut().set_y_min(ymin);
            self.as_mut().set_y_max(ymax);
        }
        self.update();
    }

    pub fn reset_zoom(mut self: Pin<&mut Self>) {
        let mut ranges: Option<(f64, f64, f64, f64)> = None;
        {
            let mut this = self.as_mut().rust_mut();
            this.x_auto_range = true;
            this.y_auto_range = true;

            let mut xmin_all = f64::INFINITY;
            let mut xmax_all = f64::NEG_INFINITY;
            let mut ymin_all = f64::INFINITY;
            let mut ymax_all = f64::NEG_INFINITY;

            for s in &this.series_list {
                if let (Some(first), Some(last)) = (s.data_x.first(), s.data_x.last()) {
                    if *first < xmin_all {
                        xmin_all = *first;
                    }
                    if *last > xmax_all {
                        xmax_all = *last;
                    }
                }
                if !s.data_y.is_empty() {
                    if s.min_y < ymin_all {
                        ymin_all = s.min_y;
                    }
                    if s.max_y > ymax_all {
                        ymax_all = s.max_y;
                    }
                }
            }
            if !xmin_all.is_finite() || !xmax_all.is_finite() {
                xmin_all = 0.0;
                xmax_all = 1.0;
            }
            if !ymin_all.is_finite() || !ymax_all.is_finite() {
                ymin_all = 0.0;
                ymax_all = 1.0;
            }
            if (xmin_all - xmax_all).abs() < f64::EPSILON {
                xmax_all = xmin_all + 1.0;
            }
            if (ymin_all - ymax_all).abs() < f64::EPSILON {
                ymax_all = ymin_all + 1.0;
            }
            this.x_min = xmin_all;
            this.x_max = xmax_all;
            this.y_min = ymin_all;
            this.y_max = ymax_all;

            ranges = Some((xmin_all, xmax_all, ymin_all, ymax_all));
        }

        self.as_mut().set_x_auto_range(true);
        self.as_mut().set_y_auto_range(true);
        if let Some((xmin, xmax, ymin, ymax)) = ranges {
            self.as_mut().set_x_min(xmin);
            self.as_mut().set_x_max(xmax);
            self.as_mut().set_y_min(ymin);
            self.as_mut().set_y_max(ymax);
        }
        self.update();
    }
}
