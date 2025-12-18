// src/heatmap_object.rs
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{QColor, QPointF, QRectF, QSizeF, QString};
use std::pin::Pin;

#[cxx_qt::bridge]
pub mod heatmap_qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = cxx_qt_lib::QSizeF;
        include!("cxx-qt-lib/qrectf.h");
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = cxx_qt_lib::QPointF;
        type QRectF = cxx_qt_lib::QRectF;

        include!(<QtQuick/QQuickItem>);
        type QQuickItem;
        include!(<QtQuick/QQuickPaintedItem>);
        type QQuickPaintedItem;
    }
    unsafe extern "C++" {
        include!(<QtGui/QPainter>);
        type QPainter;
        #[rust_name = "fill_rect"]
        fn fillRect(self: Pin<&mut QPainter>, rect: &QRectF, color: &QColor);
        #[rust_name = "draw_text"]
        fn drawText(self: Pin<&mut QPainter>, p: &QPointF, text: &QString);
        #[rust_name = "save"]
        fn save(self: Pin<&mut QPainter>);
        #[rust_name = "restore"]
        fn restore(self: Pin<&mut QPainter>);
        #[rust_name = "translate"]
        fn translate(self: Pin<&mut QPainter>, dx: f64, dy: f64);
        #[rust_name = "rotate"]
        fn rotate(self: Pin<&mut QPainter>, angle: f64);
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QQuickPaintedItem]
        #[qproperty(i32, grid_width, cxx_name = "gridWidth")]
        #[qproperty(i32, grid_height, cxx_name = "gridHeight")]
        #[qproperty(f64, x_min, cxx_name = "xMin")]
        #[qproperty(f64, x_max, cxx_name = "xMax")]
        #[qproperty(f64, y_min, cxx_name = "yMin")]
        #[qproperty(f64, y_max, cxx_name = "yMax")]
        #[qproperty(QString, x_label, cxx_name = "xLabel")]
        #[qproperty(QString, y_label, cxx_name = "yLabel")]
        #[qproperty(bool, dark_mode, cxx_name = "darkMode")]
        type HeatmapObject = super::HeatmapObjectRust;
    }
    impl cxx_qt::Threading for HeatmapObject {}
    extern "RustQt" {
        #[qinvokable]
        fn init_grid(self: Pin<&mut HeatmapObject>, width: i32, height: i32);
        #[qinvokable]
        fn set_value(self: Pin<&mut HeatmapObject>, x_index: i32, y_index: i32, value: f64);
        #[qinvokable]
        fn clear_data(self: Pin<&mut HeatmapObject>);
        #[cxx_override]
        unsafe fn paint(self: Pin<&mut HeatmapObject>, painter: *mut QPainter);
    }
    // Custom constructor declaration (CXX-Qt generálja a C++ ctor-t QQuickItem* parenttel)
    impl cxx_qt::Constructor<(*mut QQuickItem,), BaseArguments = (*mut QQuickItem,)> for HeatmapObject {}

    unsafe extern "RustQt" {
        #[inherit]
        fn update(self: Pin<&mut HeatmapObject>);
        #[inherit]
        fn size(self: &HeatmapObject) -> QSizeF;
        #[inherit]
        #[rust_name = "set_antialiasing"]
        fn setAntialiasing(self: Pin<&mut HeatmapObject>, enabled: bool);
    }
}

// ---- Custom constructor implementation (C++ helper nélkül) ------------------
impl cxx_qt::Constructor<(*mut heatmap_qobject::QQuickItem,)> for heatmap_qobject::HeatmapObject {
    type NewArguments = ();
    type BaseArguments = (*mut heatmap_qobject::QQuickItem,);
    type InitializeArguments = ();

    fn route_arguments(
        args: (*mut heatmap_qobject::QQuickItem,),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        // 1) Rust struct: default
        // 2) Base ctor: QQuickPaintedItem(parent)
        // 3) initialize: nincs extra arg
        ((), args, ())
    }

    fn new((): ()) -> HeatmapObjectRust {
        HeatmapObjectRust::default()
    }

    fn initialize(self: core::pin::Pin<&mut Self>, (): ()) {
        // Ha kell AA (szebb text/line), itt kapcsold be.
        self.set_antialiasing(true);
    }
}

pub struct HeatmapObjectRust {
    grid_width: i32,
    grid_height: i32,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    dark_mode: bool,
    x_label: QString,
    y_label: QString,
    data: Vec<f64>, // flattened grid data
}
impl Default for HeatmapObjectRust {
    fn default() -> Self {
        Self {
            grid_width: 0,
            grid_height: 0,
            x_min: 0.0,
            x_max: 1.0,
            y_min: 0.0,
            y_max: 1.0,
            dark_mode: true,
            x_label: QString::from(""),
            y_label: QString::from(""),
            data: Vec::new(),
        }
    }
}

impl heatmap_qobject::HeatmapObject {
    fn format_value(&self, val: f64) -> QString {
        if val == 0.0 {
            return QString::from("0");
        }
        if (val - val.round()).abs() < 1e-9 {
            return QString::from(&format!("{:.0}", val));
        }
        let absval = val.abs();
        let formatted = if absval >= 1000.0 || (absval > 0.0 && absval < 0.01) {
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
        if formatted.contains('e') {
            if let Some(e_idx) = formatted.find('e') {
                let (mantissa, exponent) = formatted.split_at(e_idx);
                let mantissa_trim = mantissa.trim_end_matches('0').trim_end_matches('.');
                return QString::from(&format!(
                    "{}{}",
                    if mantissa_trim.is_empty() {
                        "0"
                    } else {
                        mantissa_trim
                    },
                    exponent
                ));
            }
        } else if formatted.contains('.') {
            let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
            return QString::from(trimmed);
        }
        QString::from(&formatted)
    }

    pub fn init_grid(mut self: Pin<&mut Self>, width: i32, height: i32) {
        if width <= 0 || height <= 0 {
            return;
        }
        let mut x_update: Option<(f64, f64)> = None;
        let mut y_update: Option<(f64, f64)> = None;

        {
            let mut this = self.as_mut().rust_mut();
            this.grid_width = width;
            this.grid_height = height;
            this.data = vec![0.0; (width * height) as usize];

            // Set default axes ranges if not set
            if this.x_min >= this.x_max {
                this.x_min = 0.0;
                this.x_max = width as f64;
                x_update = Some((this.x_min, this.x_max));
            }
            if this.y_min >= this.y_max {
                this.y_min = 0.0;
                this.y_max = height as f64;
                y_update = Some((this.y_min, this.y_max));
            }
        }

        if let Some((xmin, xmax)) = x_update {
            self.as_mut().set_x_min(xmin);
            self.as_mut().set_x_max(xmax);
        }
        if let Some((ymin, ymax)) = y_update {
            self.as_mut().set_y_min(ymin);
            self.as_mut().set_y_max(ymax);
        }
        self.update();
    }

    pub fn set_value(mut self: Pin<&mut Self>, x_index: i32, y_index: i32, value: f64) {
        let mut this = self.as_mut().rust_mut();
        if x_index < 0 || y_index < 0 || x_index >= this.grid_width || y_index >= this.grid_height {
            return;
        }
        let idx = (y_index * this.grid_width + x_index) as usize;
        if idx < this.data.len() {
            this.data[idx] = value;
        }
        self.update();
    }

    pub fn clear_data(mut self: Pin<&mut Self>) {
        let mut this = self.as_mut().rust_mut();
        for v in this.data.iter_mut() {
            *v = 0.0;
        }
        self.update();
    }

    unsafe fn paint(self: Pin<&mut Self>, painter: *mut heatmap_qobject::QPainter) {
        if let Some(painter) = painter.as_mut() {
            let mut pinned_painter = Pin::new_unchecked(painter);
            let binding = self.as_ref();
            let this = binding.rust();
            let mut draw_text =
                |p: &mut Pin<&mut heatmap_qobject::QPainter>, x: f64, y: f64, text: &QString| {
                    let pt = QPointF::new(x, y);
                    p.as_mut().draw_text(&pt, text);
                };
           // pinned_painter.as_mut().set_render_hint(1, true);
            let size = self.size();
            let width = size.width();
            let height = size.height();
            // Fill background with appropriate color
            let bg_color = if this.dark_mode {
                QColor::from_rgb(0, 0, 0)
            } else {
                QColor::from_rgb(255, 255, 255)
            };
            pinned_painter
                .as_mut()
                .fill_rect(&QRectF::new(0.0, 0.0, width, height), &bg_color);
            let left_margin = 50.0;
            let right_margin = 20.0;
            let top_margin = 20.0;
            let bottom_margin = 40.0;
            let plot_x = left_margin;
            let plot_y = top_margin;
            let plot_width = (width - left_margin - right_margin).max(1.0);
            let plot_height = (height - top_margin - bottom_margin).max(1.0);
            let x_axis_y = plot_y + plot_height;
            let y_axis_x = plot_x;
            // Compute cell size
            let cols = this.grid_width.max(1) as f64;
            let rows = this.grid_height.max(1) as f64;
            let cell_w = plot_width / cols;
            let cell_h = plot_height / rows;
            // Determine value range for color mapping
            let mut val_min = std::f64::MAX;
            let mut val_max = std::f64::MIN;
            for val in &this.data {
                if *val < val_min {
                    val_min = *val;
                }
                if *val > val_max {
                    val_max = *val;
                }
            }
            if val_min == std::f64::MAX || val_max == std::f64::MIN {
                val_min = 0.0;
                val_max = 1.0;
            }
            if val_min == val_max {
                val_min -= 0.1;
                val_max += 0.1;
            }
            // Draw each cell as colored rectangle
            for yi in 0..this.grid_height {
                for xi in 0..this.grid_width {
                    let idx = (yi * this.grid_width + xi) as usize;
                    if idx >= this.data.len() {
                        continue;
                    }
                    let val = this.data[idx];
                    let frac = if val_max > val_min {
                        (val - val_min) / (val_max - val_min)
                    } else {
                        0.0
                    };
                    let (r, g, b) = if frac <= 0.5 {
                        // blue -> green
                        let t = frac * 2.0;
                        (0u8, (255.0 * t) as u8, (255.0 * (1.0 - t)) as u8)
                    } else {
                        // green -> red
                        let t = (frac - 0.5) * 2.0;
                        ((255.0 * t) as u8, (255.0 * (1.0 - t)) as u8, 0u8)
                    };
                    let color = QColor::from_rgb(r as i32, g as i32, b as i32);
                    let cell_x = plot_x + xi as f64 * cell_w;
                    let cell_y = plot_y + plot_height - (yi as f64 + 1.0) * cell_h;
                    let rect = QRectF::new(cell_x, cell_y, cell_w + 0.5, cell_h + 0.5);
                    pinned_painter.as_mut().fill_rect(&rect, &color);
                }
            }
            // Draw axes lines and tick labels
            let axis_color = if this.dark_mode {
                QColor::from_rgb(255, 255, 255)
            } else {
                QColor::from_rgb(0, 0, 0)
            };
            // Axes lines
            pinned_painter.as_mut().fill_rect(
                &QRectF::new(y_axis_x - 1.0, plot_y, 1.0, plot_height),
                &axis_color,
            );
            pinned_painter
                .as_mut()
                .fill_rect(&QRectF::new(plot_x, x_axis_y, plot_width, 1.0), &axis_color);
            // Tick labels for X and Y
            let num_ticks = 5;
            for i in 0..num_ticks {
                let tx = i as f64 / (num_ticks - 1) as f64;
                let ty = i as f64 / (num_ticks - 1) as f64;
                let data_x_val = this.x_min + tx * (this.x_max - this.x_min);
                let data_y_val = this.y_min + ty * (this.y_max - this.y_min);
                let x_pixel = plot_x + tx * plot_width;
                let y_pixel = plot_y + plot_height - ty * plot_height;
                // X labels
                let label_x = self.as_ref().format_value(data_x_val);
                let label_x_str = label_x.to_string();
                let text_x = if i == 0 {
                    x_pixel
                } else if i == num_ticks - 1 {
                    x_pixel - label_x_str.len() as f64 * 7.0
                } else {
                    x_pixel - label_x_str.len() as f64 * 3.5
                };
                draw_text(&mut pinned_painter, text_x, x_axis_y + 15.0, &label_x);
                // Y labels (skip top to avoid cut)
                if i < num_ticks - 1 {
                    let label_y = self.as_ref().format_value(data_y_val);
                    let label_y_str = label_y.to_string();
                    let text_y = y_pixel + 4.0;
                    let text_x_pos = y_axis_x - label_y_str.len() as f64 * 7.0 - 5.0;
                    draw_text(&mut pinned_painter, text_x_pos, text_y, &label_y);
                }
            }
            // Axis labels
            if !this.x_label.to_string().is_empty() {
                let x_label_str = this.x_label.to_string();
                let label_w = x_label_str.len() as f64 * 7.0;
                let text_x = plot_x + plot_width / 2.0 - label_w / 2.0;
                let text_y = x_axis_y + 30.0;
                draw_text(&mut pinned_painter, text_x, text_y, &this.x_label);
            }
            if !this.y_label.to_string().is_empty() {
                pinned_painter.as_mut().save();
                let center_y = plot_y + plot_height / 2.0;
                pinned_painter.as_mut().translate(plot_x - 40.0, center_y);
                pinned_painter.as_mut().rotate(-90.0);
                draw_text(&mut pinned_painter, 0.0, 0.0, &this.y_label);
                pinned_painter.as_mut().restore();
            }
        }
    }
}
