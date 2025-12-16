// src/graph_object.rs
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{PenStyle, QPen, QColor, QRectF, QSizeF, QString};
use std::pin::Pin;

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
        include!("cxx-qt-lib/qimage.h");
        type QImage = cxx_qt_lib::QImage;
        include!(<QtQuick/QQuickPaintedItem>);
        type QQuickPaintedItem;
    }
    unsafe extern "C++" {
        include!(<QtGui/QPainter>);
        type QPainter;
        #[rust_name = "draw_line"]
        fn drawLine(self: Pin<&mut QPainter>, x1: f64, y1: f64, x2: f64, y2: f64);
        #[rust_name = "draw_text"]
        fn drawText(self: Pin<&mut QPainter>, x: f64, y: f64, text: &QString);
        #[rust_name = "draw_ellipse"]
        fn drawEllipse(self: Pin<&mut QPainter>, rect: &QRectF);
        #[rust_name = "set_render_hint"]
        fn setRenderHint(self: Pin<&mut QPainter>, hint: i32, enabled: bool);
        #[rust_name = "fill_rect"]
        fn fillRect(self: Pin<&mut QPainter>, rect: &QRectF, color: &QColor);
        #[rust_name = "set_pen_with_pen"]
        fn setPen(self: Pin<&mut QPainter>, pen: &QPen);
        #[rust_name = "save"]
        fn save(self: Pin<&mut QPainter>);
        #[rust_name = "restore"]
        fn restore(self: Pin<&mut QPainter>);
        #[rust_name = "translate"]
        fn translate(self: Pin<&mut QPainter>, dx: f64, dy: f64);
        #[rust_name = "rotate"]
        fn rotate(self: Pin<&mut QPainter>, angle: f64);
    }
    /*
        unsafe extern "C++" {
            include!(<QtGui/QPen>);
            type QPen;
            #[rust_name = "set_pen_with_pen"]
            fn setPen(self: Pin<&mut QPainter>, pen: &QPen);
        }
        unsafe extern "C++" {
            include!(<QtGui/QImage>);
            type QImage;
        }

     */
    unsafe extern "C++" {
        include!(<QtGui/QGuiApplication>);
        include!(<QtGui/QClipboard>);
        type QClipboard;
        #[namespace = "QGuiApplication"]
        #[rust_name = "clipboard"]
        fn clipboard() -> *mut QClipboard;
        #[rust_name = "clipboard_set_text"]
        fn setText(self: Pin<&mut QClipboard>, text: &QString);
        #[rust_name = "clipboard_set_image"]
        fn setImage(self: Pin<&mut QClipboard>, image: &QImage);
    }
    /*
    unsafe extern "C++" {
        #[rust_name = "qpainteditem_to_qquickitem"]
        fn static_cast_QQuickItem(ptr: *mut QQuickPaintedItem) -> *mut QQuickItem;
        #[rust_name = "graphobject_to_qpainteditem"]
        fn static_cast_QQuickPaintedItem(ptr: *mut GraphObject) -> *mut QQuickPaintedItem;
    }
    unsafe extern "C++" {
        include!(<QtQuick/QQuickItemGrabResult>);
        type QQuickItemGrabResult;
        #[rust_name = "grab_to_image_item"]
        fn grabToImage(self: Pin<&mut QQuickPaintedItem>) -> *mut QQuickItemGrabResult;
        #[rust_name = "result_image"]
        fn image(self: &QQuickItemGrabResult) -> QImage;
        #[rust_name = "result_save_to_file"]
        fn saveToFile(self: &QQuickItemGrabResult, file: &QString) -> bool;
    }*/

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
        type GraphObject = super::GraphObjectRust;
    }
    impl cxx_qt::Threading for GraphObject {}
    unsafe extern "C++" {
        include!("graph_object_helpers.h");
        #[namespace = "graph_object_helpers"]
        #[rust_name = "helpers_grab_image"]
        fn grab_image(item: *mut GraphObject) -> UniquePtr<QImage>;
        #[namespace = "graph_object_helpers"]
        #[rust_name = "helpers_save_image"]
        fn save_image(item: *mut GraphObject, file_path: &QString) -> bool;
    }

    extern "RustQt" {
        #[qinvokable]
        fn add_series(self: Pin<&mut GraphObject>, name: &QString, series_type: i32, color: &QColor, thickness: f64, line_style: i32, marker: bool);
        #[qinvokable]
        fn remove_series(self: Pin<&mut GraphObject>, name: &QString);
        #[qinvokable]
        fn add_data_point(self: Pin<&mut GraphObject>, series_name: &QString, x: f64, y: f64);
        #[qinvokable]
        fn zoom_to_region(self: Pin<&mut GraphObject>, x1: f64, x2: f64, y1: f64, y2: f64);
        #[qinvokable]
        fn zoom_x(self: Pin<&mut GraphObject>, x1: f64, x2: f64);
        #[qinvokable]
        fn zoom_y(self: Pin<&mut GraphObject>, y1: f64, y2: f64);
        #[qinvokable]
        fn zoom_at_point(self: Pin<&mut GraphObject>, center_x: f64, center_y: f64, factor: f64);
        #[qinvokable]
        fn pan(self: Pin<&mut GraphObject>, delta_x: f64, delta_y: f64);
        #[qinvokable]
        fn reset_zoom(self: Pin<&mut GraphObject>);
        #[qinvokable]
        fn save_csv(self: Pin<&mut GraphObject>, file_path: &QString);
        #[qinvokable]
        fn save_image(self: Pin<&mut GraphObject>, file_path: &QString);
        #[qinvokable]
        fn copy_data(self: Pin<&mut GraphObject>);
        #[qinvokable]
        fn copy_image(self: Pin<&mut GraphObject>);
        #[qinvokable]
        fn place_vertical_cursor(self: Pin<&mut GraphObject>, x: f64);
        #[qinvokable]
        fn place_horizontal_cursor(self: Pin<&mut GraphObject>, y: f64);
        #[qinvokable]
        fn clear_cursors(self: Pin<&mut GraphObject>);
        #[qinvokable]
        fn request_repaint(self: Pin<&mut GraphObject>);
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
    mode: i32,            // 0: mode1 (compress), 1: mode2 (scroll), 2: mode3 (triggered)
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
        }
    }
}

impl graph_object_qobject::GraphObject {
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

    pub fn save_csv(mut self: Pin<&mut Self>, file_path: &QString) {
        use std::io::Write;
        let this = self.as_ref().rust();
        if let Ok(mut file) = std::fs::File::create(std::path::Path::new(&file_path.to_string())) {
            for (i, s) in this.series_list.iter().enumerate() {
                if i > 0 {
                    writeln!(file).ok();
                }
                let header_x = if !this.x_label.to_string().is_empty() { this.x_label.to_string() } else { "X".to_owned() };
                let header_y = s.name.clone();
                writeln!(file, "{},{}", header_x, header_y).ok();
                for (x, y) in s.data_x.iter().zip(&s.data_y) {
                    writeln!(file, "{:.6},{:.6}", x, y).ok();
                }
            }
        }
    }
    pub fn copy_data(mut self: Pin<&mut Self>) {
        let this = self.as_ref().rust();
        let clipboard_ptr = graph_object_qobject::clipboard();
        if !clipboard_ptr.is_null() {
            let mut pinned_cb = unsafe { Pin::new_unchecked(&mut *clipboard_ptr) };
            let mut csv = String::new();
            for (i, s) in this.series_list.iter().enumerate() {
                if i > 0 {
                    csv.push('\n');
                }
                let header_x = if !this.x_label.to_string().is_empty() { this.x_label.to_string() } else { "X".to_owned() };
                csv += &format!("{},{}\n", header_x, s.name);
                for (x, y) in s.data_x.iter().zip(&s.data_y) {
                    csv += &format!("{:.6},{:.6}\n", x, y);
                }
            }
            let qstr = QString::from(&csv);
            pinned_cb.as_mut().clipboard_set_text(&qstr);
        }
    }
    pub fn copy_image(mut self: Pin<&mut Self>) {
        let clipboard_ptr = graph_object_qobject::clipboard();
        if clipboard_ptr.is_null() {
            return;
        }
        /*
        unsafe {
            if let Some(cb) = clipboard_ptr.as_mut() {
                let mut pinned_cb = Pin::new_unchecked(cb);
                // Attempt to grab image from QQuickPaintedItem
                let raw_ptr = graph_object_qobject::static_cast_QQuickPaintedItem(self.as_mut().cpp_mut());
                if !raw_ptr.is_null() {
                    let grab_result = Pin::new_unchecked(&mut *raw_ptr).grab_to_image_item();
                    if !grab_result.is_null() {
                        let img = (*grab_result).image();
                        pinned_cb.as_mut().clipboard_set_image(&img);
                        return;
                    }
                }
                // Fallback: copy placeholder text if image not captured
                let msg = QString::from("[Image Data]");
                pinned_cb.as_mut().clipboard_set_text(&msg);
            }
        }
        */
        let img = graph_object_qobject::helpers_grab_image(self.as_mut().cpp_mut());
        unsafe {
            if let Some(cb) = clipboard_ptr.as_mut() {
                let mut pinned_cb = Pin::new_unchecked(cb);
                if let Some(img_ref) = img.as_ref() {
                    pinned_cb.as_mut().clipboard_set_image(img_ref);
                } else {
                    let msg = QString::from("[Image capture failed]");
                    pinned_cb.as_mut().clipboard_set_text(&msg);
                }
            }
        }
    }

    pub fn save_image(mut self: Pin<&mut Self>, file_path: &QString) {
        /*
        let raw_ptr = graph_object_qobject::static_cast_QQuickPaintedItem(self.as_mut().cpp_mut());
        if raw_ptr.is_null() {
            return;
        }
        unsafe {
            let grab_result = Pin::new_unchecked(&mut *raw_ptr).grab_to_image_item();
            if !grab_result.is_null() {
                let mut save_path = file_path.to_string();
                if !save_path.ends_with(".png") && !save_path.ends_with(".jpg") && !save_path.ends_with(".bmp") {
                    save_path.push_str(".png");
                }
                let qstr = QString::from(&save_path);
                (*grab_result).result_save_to_file(&qstr);
            }
        }
        */
        let mut save_path = file_path.to_string();
        if !save_path.ends_with(".png") && !save_path.ends_with(".jpg") && !save_path.ends_with(".bmp") {
            save_path.push_str(".png");
        }
        let qstr = QString::from(&save_path);
        let _ = graph_object_qobject::helpers_save_image(self.as_mut().cpp_mut(), &qstr);
    }
    pub fn place_vertical_cursor(mut self: Pin<&mut Self>, x: f64) {
        let this = self.as_mut().rust_mut();
        if this.cursor_x_positions.len() < 2 {
            this.cursor_x_positions.push(x);
        } else {
            this.cursor_x_positions.clear();
            this.cursor_x_positions.push(x);
        }
        self.update();
    }
    pub fn place_horizontal_cursor(mut self: Pin<&mut Self>, y: f64) {
        let this = self.as_mut().rust_mut();
        if this.cursor_y_positions.len() < 2 {
            this.cursor_y_positions.push(y);
        } else {
            this.cursor_y_positions.clear();
            this.cursor_y_positions.push(y);
        }
        self.update();
    }
    pub fn clear_cursors(mut self: Pin<&mut Self>) {
        let this = self.as_mut().rust_mut();
        this.cursor_x_positions.clear();
        this.cursor_y_positions.clear();
        self.update();
    }

    pub fn request_repaint(mut self: Pin<&mut Self>) {
        self.update();
    }

    pub fn add_series(mut self: Pin<&mut Self>, name: &QString, series_type: i32, color: &QColor, thickness: f64, line_style: i32, marker: bool) {
        let mut this = self.as_mut().rust_mut();
        // Ensure unique name by removing any existing series with same name
        this.series_list.retain(|s| s.name != name.to_string());
        let is_digital = series_type == 2; // assume 0=analog, 1=int (treat as analog), 2=bool digital
        let series = DataSeries {
            name: name.to_string(),
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
        // Maybe update axes if auto-range (especially y-axis if adding digital series adjusts y range? For bool, 0-1)
        if this.y_auto_range {
            // Recompute global y range from all series data (likely empty new series so just consider existing)
            let mut min_y = std::f64::MAX;
            let mut max_y = std::f64::MIN;
            for s in &this.series_list {
                if !s.data_y.is_empty() {
                    if s.min_y < min_y { min_y = s.min_y; }
                    if s.max_y > max_y { max_y = s.max_y; }
                }
            }
            if min_y == std::f64::MAX {
                // no data in any series yet, just reset to 0..1 default
                min_y = 0.0;
                max_y = 1.0;
            } else if min_y == max_y {
                max_y = min_y + 1.0;
            }
            this.y_min = min_y;
            this.y_max = max_y;
            self.as_mut().set_y_min(min_y);
            self.as_mut().set_y_max(max_y);
        }
        // If adding first series with no data, set some defaults for x axis as well (just dummy 0..1)
        if this.x_auto_range && this.series_list.len() == 1 && this.series_list[0].data_x.is_empty() {
            this.x_min = 0.0;
            this.x_max = 1.0;
            self.as_mut().set_x_min(0.0);
            self.as_mut().set_x_max(1.0);
        }
        self.update();  // trigger repaint
    }

    pub fn remove_series(mut self: Pin<&mut Self>, name: &QString) {
        let mut this = self.as_mut().rust_mut();
        this.series_list.retain(|s| s.name != name.to_string());
        // Recompute axes if needed
        if this.x_auto_range || this.y_auto_range {
            let mut new_x_min = std::f64::MAX;
            let mut new_x_max = std::f64::MIN;
            let mut new_y_min = std::f64::MAX;
            let mut new_y_max = std::f64::MIN;
            for s in &this.series_list {
                if !s.data_x.is_empty() {
                    let sx_min = *s.data_x.first().unwrap();
                    let sx_max = *s.data_x.last().unwrap();
                    if sx_min < new_x_min { new_x_min = sx_min; }
                    if sx_max > new_x_max { new_x_max = sx_max; }
                }
                if !s.data_y.is_empty() {
                    if s.min_y < new_y_min { new_y_min = s.min_y; }
                    if s.max_y > new_y_max { new_y_max = s.max_y; }
                }
            }
            if new_x_min == std::f64::MAX {
                // no series or no data: reset to 0..1
                new_x_min = 0.0;
                new_x_max = 1.0;
            } else if new_x_min == new_x_max {
                new_x_max = new_x_min + 1.0;
            }
            if new_y_min == std::f64::MAX {
                new_y_min = 0.0;
                new_y_max = 1.0;
            } else if new_y_min == new_y_max {
                new_y_max = new_y_min + 1.0;
            }
            if this.x_auto_range {
                this.x_min = new_x_min;
                this.x_max = new_x_max;
                self.as_mut().set_x_min(new_x_min);
                self.as_mut().set_x_max(new_x_max);
            }
            if this.y_auto_range {
                this.y_min = new_y_min;
                this.y_max = new_y_max;
                self.as_mut().set_y_min(new_y_min);
                self.as_mut().set_y_max(new_y_max);
            }
        }
        self.update();
    }

    pub fn add_data_point(mut self: Pin<&mut Self>, series_name: &QString, x: f64, y: f64) {
        let mut this = self.as_mut().rust_mut();
        if let Some(series) = this.series_list.iter_mut().find(|s| s.name == series_name.to_string()) {
            let px = x;
            let py = if series.is_digital {
                // For digital, treat value >0 as 1, else 0 (just in case)
                if y > 0.5 { 1.0 } else { 0.0 }
            } else {
                y
            };
            // Append new data point
            series.data_x.push(px);
            series.data_y.push(py);
            // Update series min/max for this series
            if series.data_y.len() == 1 {
                series.min_y = py;
                series.max_y = py;
            } else {
                if py < series.min_y { series.min_y = py; }
                if py > series.max_y { series.max_y = py; }
            }
            // If x_auto_range:
            if this.x_auto_range {
                if !this.initial_x_set {
                    // first data point in graph
                    this.initial_x_set = true;
                    this.x_min = px;
                }
                if this.mode == 0 {
                    // Mode1: compress, keep earliest x_min, extend x_max
                    if px > this.x_max {
                        this.x_max = px;
                    }
                } else if this.mode == 1 {
                    // Mode2: scrolling window
                    // Remove old points if beyond buffer
                    let buf = if this.buffer_size < 1 { 1 } else { this.buffer_size } as usize;
                    if series.data_x.len() > buf {
                        // How many to drop? just 1 (we can drop one at a time)
                        let drop_count = series.data_x.len() - buf;
                        // Remove from front of this series
                        series.data_x.drain(0..drop_count);
                        series.data_y.drain(0..drop_count);
                        // Also for all other series, if they have more points than buf, drop same count to align
                        for s2 in this.series_list.iter_mut() {
                            if s2.name != series.name && s2.data_x.len() > buf {
                                let remove = if s2.data_x.len() >= drop_count { drop_count } else { s2.data_x.len() };
                                s2.data_x.drain(0..remove);
                                s2.data_y.drain(0..remove);
                            }
                        }
                    }
                    // After dropping, update x range to show last buffer_size points
                    // Determine global min and max x from series
                    let mut xmin_all = std::f64::MAX;
                    let mut xmax_all = std::f64::MIN;
                    for s2 in &this.series_list {
                        if !s2.data_x.is_empty() {
                            let sxmin = *s2.data_x.first().unwrap();
                            let sxmax = *s2.data_x.last().unwrap();
                            if sxmin < xmin_all { xmin_all = sxmin; }
                            if sxmax > xmax_all { xmax_all = sxmax; }
                        }
                    }
                    if xmin_all == std::f64::MAX {
                        xmin_all = this.x_min;
                    }
                    if xmax_all == std::f64::MIN {
                        xmax_all = this.x_max;
                    }
                    this.x_min = xmin_all;
                    this.x_max = xmax_all;
                } else if this.mode == 2 {
                    // Mode3: triggered - when buffer full, reset (clear data)
                    let buf = if this.buffer_size < 1 { 1 } else { this.buffer_size } as usize;
                    if series.data_x.len() > buf {
                        // Trigger event: buffer full
                        let new_point_x = px;
                        let new_point_y = py;
                        // Calculate last frame span if we had one
                        if series.data_x.len() >= buf {
                            // span = x_range of this series for full frame
                            let first_x = series.data_x.first().cloned().unwrap_or(new_point_x);
                            let last_x = series.data_x.last().cloned().unwrap_or(new_point_x);
                            this.last_frame_span = Some(last_x - first_x);
                        }
                        // Clear all series data
                        for s2 in this.series_list.iter_mut() {
                            s2.data_x.clear();
                            s2.data_y.clear();
                            s2.min_y = 0.0;
                            s2.max_y = 0.0;
                        }
                        // Start new frame with the current point for this series
                        series.data_x.push(new_point_x);
                        series.data_y.push(new_point_y);
                        series.min_y = new_point_y;
                        series.max_y = new_point_y;
                        // Adjust x axis range to new frame
                        if let Some(span) = this.last_frame_span {
                            this.x_min = new_point_x;
                            this.x_max = new_point_x + span;
                        } else {
                            // if unknown span, just set a default range of buffer_size points
                            this.x_min = new_point_x;
                            this.x_max = new_point_x + this.buffer_size as f64;
                        }
                    } else {
                        // If buffer not yet full, update x_max normally
                        if px > this.x_max || this.x_min > this.x_max {
                            // if x_min not set properly or px beyond
                            this.x_max = px;
                            if !this.initial_x_set {
                                this.initial_x_set = true;
                                this.x_min = px;
                            }
                        }
                    }
                }
                // Update Q_PROPERTY values
                self.as_mut().set_x_min(this.x_min);
                self.as_mut().set_x_max(this.x_max);
            }
            // Update y axis range if auto
            if this.y_auto_range {
                // For global y range, find min/max across all series
                let mut new_y_min = std::f64::MAX;
                let mut new_y_max = std::f64::MIN;
                for s2 in &this.series_list {
                    if !s2.data_y.is_empty() {
                        if s2.min_y < new_y_min { new_y_min = s2.min_y; }
                        if s2.max_y > new_y_max { new_y_max = s2.max_y; }
                    }
                }
                if new_y_min == std::f64::MAX {
                    new_y_min = 0.0;
                    new_y_max = 1.0;
                } else if new_y_min == new_y_max {
                    new_y_max = new_y_min + 1.0;
                }
                this.y_min = new_y_min;
                this.y_max = new_y_max;
                self.as_mut().set_y_min(new_y_min);
                self.as_mut().set_y_max(new_y_max);
            }
        }
        // Trigger repaint
        self.update();
    }

    pub fn zoom_to_region(mut self: Pin<&mut Self>, x1: f64, x2: f64, y1: f64, y2: f64) {
        let mut this = self.as_mut().rust_mut();
        if x2 != x1 {
            this.x_auto_range = false;
            this.x_min = x1.min(x2);
            this.x_max = x1.max(x2);
            self.as_mut().set_x_auto_range(false);
            self.as_mut().set_x_min(this.x_min);
            self.as_mut().set_x_max(this.x_max);
        }
        if y2 != y1 {
            this.y_auto_range = false;
            this.y_min = y1.min(y2);
            this.y_max = y1.max(y2);
            self.as_mut().set_y_auto_range(false);
            self.as_mut().set_y_min(this.y_min);
            self.as_mut().set_y_max(this.y_max);
        }
        self.update();
    }

    pub fn zoom_x(mut self: Pin<&mut Self>, x1: f64, x2: f64) {
        let mut this = self.as_mut().rust_mut();
        if x2 != x1 {
            this.x_auto_range = false;
            this.x_min = x1.min(x2);
            this.x_max = x1.max(x2);
            self.as_mut().set_x_auto_range(false);
            self.as_mut().set_x_min(this.x_min);
            self.as_mut().set_x_max(this.x_max);
        }
        self.update();
    }

    pub fn zoom_y(mut self: Pin<&mut Self>, y1: f64, y2: f64) {
        let mut this = self.as_mut().rust_mut();
        if y2 != y1 {
            this.y_auto_range = false;
            this.y_min = y1.min(y2);
            this.y_max = y1.max(y2);
            self.as_mut().set_y_auto_range(false);
            self.as_mut().set_y_min(this.y_min);
            self.as_mut().set_y_max(this.y_max);
        }
        self.update();
    }

    pub fn zoom_at_point(mut self: Pin<&mut Self>, center_x: f64, center_y: f64, factor: f64) {
        if factor <= 0.0 {
            return;
        }
        let mut this = self.as_mut().rust_mut();
        // Turn off auto range when manual zoom
        if this.x_auto_range || this.y_auto_range {
            this.x_auto_range = false;
            this.y_auto_range = false;
            self.as_mut().set_x_auto_range(false);
            self.as_mut().set_y_auto_range(false);
        }
        let fx = factor;
        let fy = factor;
        let old_x_min = this.x_min;
        let old_x_max = this.x_max;
        let old_y_min = this.y_min;
        let old_y_max = this.y_max;
        // center_x, center_y in data coordinates around which to zoom
        this.x_min = center_x - (center_x - old_x_min) / fx;
        this.x_max = center_x + (old_x_max - center_x) / fx;
        this.y_min = center_y - (center_y - old_y_min) / fy;
        this.y_max = center_y + (old_y_max - center_y) / fy;
        self.as_mut().set_x_min(this.x_min);
        self.as_mut().set_x_max(this.x_max);
        self.as_mut().set_y_min(this.y_min);
        self.as_mut().set_y_max(this.y_max);
        self.update();
    }

    pub fn pan(mut self: Pin<&mut Self>, delta_x: f64, delta_y: f64) {
        let mut this = self.as_mut().rust_mut();
        if this.x_auto_range || this.y_auto_range {
            // if auto, disable auto when user pans
            this.x_auto_range = false;
            this.y_auto_range = false;
            self.as_mut().set_x_auto_range(false);
            self.as_mut().set_y_auto_range(false);
        }
        // Shift view by delta in data units
        this.x_min += delta_x;
        this.x_max += delta_x;
        this.y_min += delta_y;
        this.y_max += delta_y;
        self.as_mut().set_x_min(this.x_min);
        self.as_mut().set_x_max(this.x_max);
        self.as_mut().set_y_min(this.y_min);
        self.as_mut().set_y_max(this.y_max);
        self.update();
    }

    pub fn reset_zoom(mut self: Pin<&mut Self>) {
        let mut this = self.as_mut().rust_mut();
        // Re-enable auto range and recompute full data extents
        this.x_auto_range = true;
        this.y_auto_range = true;
        self.as_mut().set_x_auto_range(true);
        self.as_mut().set_y_auto_range(true);
        // Compute overall min/max from all data
        let mut xmin_all = std::f64::MAX;
        let mut xmax_all = std::f64::MIN;
        let mut ymin_all = std::f64::MAX;
        let mut ymax_all = std::f64::MIN;
        for s in &this.series_list {
            if !s.data_x.is_empty() {
                let sxmin = *s.data_x.first().unwrap();
                let sxmax = *s.data_x.last().unwrap();
                if sxmin < xmin_all { xmin_all = sxmin; }
                if sxmax > xmax_all { xmax_all = sxmax; }
            }
            if !s.data_y.is_empty() {
                if s.min_y < ymin_all { ymin_all = s.min_y; }
                if s.max_y > ymax_all { ymax_all = s.max_y; }
            }
        }
        if xmin_all == std::f64::MAX || xmax_all == std::f64::MIN {
            xmin_all = 0.0;
            xmax_all = 1.0;
        }
        if ymin_all == std::f64::MAX || ymax_all == std::f64::MIN {
            ymin_all = 0.0;
            ymax_all = 1.0;
        }
        if xmin_all == xmax_all {
            xmax_all = xmin_all + 1.0;
        }
        if ymin_all == ymax_all {
            ymax_all = ymin_all + 1.0;
        }
        this.x_min = xmin_all;
        this.x_max = xmax_all;
        this.y_min = ymin_all;
        this.y_max = ymax_all;
        self.as_mut().set_x_min(xmin_all);
        self.as_mut().set_x_max(xmax_all);
        self.as_mut().set_y_min(ymin_all);
        self.as_mut().set_y_max(ymax_all);
        self.update();
    }

    unsafe fn paint(self: Pin<&mut Self>, painter: *mut graph_object_qobject::QPainter) {
        if let Some(painter) = painter.as_mut() {
            let mut pinned_painter = Pin::new_unchecked(painter);
            let this = self.as_ref().rust();
            // Enable antialiasing for smoother lines and text
            pinned_painter.as_mut().set_render_hint(1, true);
            // Determine dimensions
            let size = self.size();
            let width = size.width();
            let height = size.height();
            // Fill background
            let bg_color = if this.dark_mode { QColor::from_rgb(0, 0, 0) } else { QColor::from_rgb(255, 255, 255) };
            pinned_painter.as_mut().fill_rect(&QRectF::new(0.0, 0.0, width, height), &bg_color);
            // Define margins for axes and legend
            let left_margin: f64 = 60.0;
            let right_margin: f64 = 10.0;
            let top_margin: f64 = if this.legend_visible && this.legend_position < 2 { 20.0 } else { 10.0 };
            let bottom_margin: f64 = 50.0;
            // Compute drawing area for data
            let plot_x = left_margin;
            let plot_y = top_margin;
            let plot_width = (width - left_margin - right_margin).max(1.0);
            let plot_height = (height - top_margin - bottom_margin).max(1.0);
            // Axes positions
            let x_axis_y = plot_y + plot_height;  // y coordinate of X axis line (bottom of plot area)
            let y_axis_x = plot_x;                // x coordinate of Y axis line (left of plot area)
            // Determine effective min/max for log scales (avoid <=0)
            let (x_min_val, x_max_val) = if this.x_log_scale {
                if this.x_max <= 0.0 {
                    (0.1, 1.0)
                } else {
                    let min_val = if this.x_min > 0.0 { this.x_min } else { (this.x_max / 1e6).max(1e-9) };
                    (min_val, this.x_max)
                }
            } else {
                (this.x_min, this.x_max)
            };
            let (y_min_val, y_max_val) = if this.y_log_scale {
                if this.y_max <= 0.0 {
                    (0.1, 1.0)
                } else {
                    let min_val = if this.y_min > 0.0 { this.y_min } else { (this.y_max / 1e6).max(1e-9) };
                    (min_val, this.y_max)
                }
            } else {
                (this.y_min, this.y_max)
            };
            // Draw grid (if enabled) and axes lines
            // Set pen for grid and axis lines (color gray for grid, white/black for axes)
            let axis_color = if this.dark_mode { QColor::from_rgb(255, 255, 255) } else { QColor::from_rgb(0, 0, 0) };
            let grid_color = if this.dark_mode { QColor::from_rgb(136, 136, 136) } else { QColor::from_rgb(136, 136, 136) };
            // Draw vertical grid lines and vertical (Y) axis line
            let mut grid_pen = QPen::default();
            grid_pen.set_color(&grid_color);
            grid_pen.set_width(0);
            let grid_pen_style = if this.grid_visible { PenStyle::DashLine } else { PenStyle::SolidLine };
            grid_pen.set_style(grid_pen_style);
            pinned_painter.as_mut().set_pen_with_pen(&grid_pen);
            let num_x_ticks = 5;
            for i in 0..num_x_ticks {
                let t = i as f64 / (num_x_ticks - 1) as f64;
                let data_x_val = if this.x_log_scale {
                    let log_min = x_min_val.log10();
                    let log_max = x_max_val.log10();
                    (10.0_f64).powf(log_min + t * (log_max - log_min))
                } else {
                    x_min_val + t * (x_max_val - x_min_val)
                };
                let x_pixel = if this.x_log_scale {
                    let log_min = x_min_val.log10();
                    let log_max = x_max_val.log10();
                    plot_x + ((data_x_val.log10() - log_min) / (log_max - log_min)) * plot_width
                } else {
                    plot_x + ((data_x_val - x_min_val) / (x_max_val - x_min_val)) * plot_width
                };
                if this.grid_visible {
                    // vertical grid line
                    pinned_painter.as_mut().draw_line(x_pixel, plot_y, x_pixel, plot_y + plot_height);
                }
            }
            // Vertical axis line (left)
            let mut axis_pen = QPen::default();
            axis_pen.set_color(&axis_color);
            axis_pen.set_width(0);
            axis_pen.set_style(PenStyle::SolidLine);
            pinned_painter.as_mut().set_pen_with_pen(&axis_pen);
            pinned_painter.as_mut().draw_line(y_axis_x, plot_y, y_axis_x, plot_y + plot_height);
            // Draw horizontal grid lines and horizontal (X) axis line
            let mut grid_pen2 = QPen::default();
            grid_pen2.set_color(&grid_color);
            grid_pen2.set_width(0);
            let grid_pen2_style = if this.grid_visible { PenStyle::DashLine } else { PenStyle::SolidLine };
            grid_pen2.set_style(grid_pen2_style);
            pinned_painter.as_mut().set_pen_with_pen(&grid_pen2);
            let num_y_ticks = 5;
            for j in 0..num_y_ticks {
                let t = j as f64 / (num_y_ticks - 1) as f64;
                let data_y_val = if this.y_log_scale {
                    let log_min = y_min_val.log10();
                    let log_max = y_max_val.log10();
                    (10.0_f64).powf(log_min + t * (log_max - log_min))
                } else {
                    y_min_val + t * (y_max_val - y_min_val)
                };
                let y_pixel = plot_y + plot_height - (if this.y_log_scale {
                    let log_min = y_min_val.log10();
                    let log_max = y_max_val.log10();
                    ((data_y_val.log10() - log_min) / (log_max - log_min)) * plot_height
                } else {
                    ((data_y_val - y_min_val) / (y_max_val - y_min_val)) * plot_height
                });
                if this.grid_visible && !this.separate_series {
                    // horizontal grid line
                    pinned_painter.as_mut().draw_line(plot_x, y_pixel, plot_x + plot_width, y_pixel);
                }
            }
            // Horizontal axis line (bottom)
            let mut axis_pen2 = QPen::default();
            axis_pen2.set_color(&axis_color);
            axis_pen2.set_width(0);
            axis_pen2.set_style(PenStyle::SolidLine);
            pinned_painter.as_mut().set_pen_with_pen(&axis_pen2);
            pinned_painter.as_mut().draw_line(plot_x, x_axis_y, plot_x + plot_width, x_axis_y);
            // Draw tick marks and labels
            let mut axis_pen3 = QPen::default();
            axis_pen3.set_color(&axis_color);
            axis_pen3.set_width(0);
            axis_pen3.set_style(PenStyle::SolidLine);
            pinned_painter.as_mut().set_pen_with_pen(&axis_pen3);
            // X-axis ticks and labels
            for i in 0..num_x_ticks {
                let t = i as f64 / (num_x_ticks - 1) as f64;
                let data_x_val = if this.x_log_scale {
                    let log_min = x_min_val.log10();
                    let log_max = x_max_val.log10();
                    (10.0_f64).powf(log_min + t * (log_max - log_min))
                } else {
                    x_min_val + t * (x_max_val - x_min_val)
                };
                let x_pixel = if this.x_log_scale {
                    let log_min = x_min_val.log10();
                    let log_max = x_max_val.log10();
                    plot_x + ((data_x_val.log10() - log_min) / (log_max - log_min)) * plot_width
                } else {
                    plot_x + ((data_x_val - x_min_val) / (x_max_val - x_min_val)) * plot_width
                };
                // tick mark (vertical small line)
                let tick_len = 5.0;
                pinned_painter.as_mut().draw_line(x_pixel, x_axis_y, x_pixel, x_axis_y - tick_len);
                // label
                let label = self.as_ref().format_value(data_x_val);
                // Position label: center except at ends
                let label_str = label.to_string();
                let mut text_x = x_pixel;
                if i == 0 {
                    // leftmost align to left
                    text_x = x_pixel;
                } else if i == num_x_ticks - 1 {
                    // rightmost align to right by subtracting approximate text width
                    let approx_width = label_str.len() as f64 * 7.0;
                    text_x = x_pixel - approx_width;
                } else {
                    // center align
                    let approx_width = label_str.len() as f64 * 7.0;
                    text_x = x_pixel - approx_width / 2.0;
                }
                let text_y = x_axis_y + 15.0;
                pinned_painter.as_mut().draw_text(text_x, text_y, &label);
            }
            // Y-axis ticks and labels (if not in separate series mode)
            if !this.separate_series {
                for j in 0..num_y_ticks {
                    let t = j as f64 / (num_y_ticks - 1) as f64;
                    let data_y_val = if this.y_log_scale {
                        let log_min = y_min_val.log10();
                        let log_max = y_max_val.log10();
                        (10.0_f64).powf(log_min + t * (log_max - log_min))
                    } else {
                        y_min_val + t * (y_max_val - y_min_val)
                    };
                    let y_pixel = plot_y + plot_height - (if this.y_log_scale {
                        let log_min = y_min_val.log10();
                        let log_max = y_max_val.log10();
                        ((data_y_val.log10() - log_min) / (log_max - log_min)) * plot_height
                    } else {
                        ((data_y_val - y_min_val) / (y_max_val - y_min_val)) * plot_height
                    });
                    // tick mark (horizontal small line)
                    let tick_len = 5.0;
                    pinned_painter.as_mut().draw_line(y_axis_x, y_pixel, y_axis_x + tick_len, y_pixel);
                    // label, skip top label to avoid cut off
                    if j == num_y_ticks - 1 {
                        continue;
                    }
                    let label = self.as_ref().format_value(data_y_val);
                    let label_str = label.to_string();
                    // Right-align label to just left of axis line
                    let approx_width = label_str.len() as f64 * 7.0;
                    let text_x = y_axis_x - approx_width - 2.0;
                    let text_y = y_pixel + 4.0;  // baseline at tick (approx center)
                    pinned_painter.as_mut().draw_text(text_x, text_y, &label);
                }
            }
            // Axis labels (units or names)
            // X-axis label (centered at bottom margin area)
            if !this.x_label.to_string().is_empty() {
                let x_label_str = this.x_label.to_string();
                // Center horizontally in plot area
                let label_width = x_label_str.len() as f64 * 7.0;
                let text_x = plot_x + plot_width / 2.0 - label_width / 2.0;
                let text_y = x_axis_y + 35.0;
                pinned_painter.as_mut().draw_text(text_x, text_y, &this.x_label);
            }
            // Y-axis label (rotated vertical)
            if !this.y_label.to_string().is_empty() {
                // Save painter state
                pinned_painter.as_mut().save();
                // Determine position roughly center left, rotated -90
                let center_y = plot_y + plot_height / 2.0;
                let text_x = 15.0;
                let text_y = center_y;
                // Translate and rotate
                pinned_painter.as_mut().translate(text_x, text_y);
                pinned_painter.as_mut().rotate(-90.0);
                pinned_painter.as_mut().draw_text(0.0, 0.0, &this.y_label);
                // Restore painter state
                pinned_painter.as_mut().restore();
            }
            // Draw series data
            for (si, s) in this.series_list.iter().enumerate() {
                if s.data_x.is_empty() {
                    continue;
                }
                // Choose pen for series (color, thickness, style)
                let style_val = if s.line_style <= 0 { 1 } else { s.line_style };
                let mut pen = QPen::default();
                pen.set_color(&s.color);
                let width_i = if s.thickness <= 0.0 { 1 } else { s.thickness.round() as i32 };
                pen.set_width(width_i);
                let pen_style = match style_val {
                    2 => PenStyle::DashLine,
                    3 => PenStyle::DotLine,
                    4 => PenStyle::DashDotLine,
                    5 => PenStyle::DashDotDotLine,
                    _ => PenStyle::SolidLine,
                };
                pen.set_style(pen_style);
                pinned_painter.as_mut().set_pen_with_pen(&pen);
                if !this.separate_series {
                    // Draw connecting lines
                    if s.data_x.len() > 1 {
                        if s.is_digital {
                            // digital: draw steps
                            for k in 0..(s.data_x.len() - 1) {
                                let x_curr = if this.x_log_scale {
                                    if s.data_x[k] <= 0.0 { continue; }
                                    let log_min = x_min_val.log10();
                                    let log_max = x_max_val.log10();
                                    plot_x + ((s.data_x[k].log10() - log_min) / (log_max - log_min)) * plot_width
                                } else {
                                    plot_x + ((s.data_x[k] - x_min_val) / (x_max_val - x_min_val)) * plot_width
                                };
                                let x_next = if this.x_log_scale {
                                    if s.data_x[k + 1] <= 0.0 { continue; }
                                    let log_min = x_min_val.log10();
                                    let log_max = x_max_val.log10();
                                    plot_x + ((s.data_x[k + 1].log10() - log_min) / (log_max - log_min)) * plot_width
                                } else {
                                    plot_x + ((s.data_x[k + 1] - x_min_val) / (x_max_val - x_min_val)) * plot_width
                                };
                                let y_curr = if this.y_log_scale {
                                    if s.data_y[k] <= 0.0 { continue; }
                                    let log_min = y_min_val.log10();
                                    let log_max = y_max_val.log10();
                                    plot_y + plot_height - ((s.data_y[k].log10() - log_min) / (log_max - log_min)) * plot_height
                                } else {
                                    plot_y + plot_height - ((s.data_y[k] - y_min_val) / (y_max_val - y_min_val)) * plot_height
                                };
                                let y_next = if this.y_log_scale {
                                    if s.data_y[k + 1] <= 0.0 { continue; }
                                    let log_min = y_min_val.log10();
                                    let log_max = y_max_val.log10();
                                    plot_y + plot_height - ((s.data_y[k + 1].log10() - log_min) / (log_max - log_min)) * plot_height
                                } else {
                                    plot_y + plot_height - ((s.data_y[k + 1] - y_min_val) / (y_max_val - y_min_val)) * plot_height
                                };
                                // horizontal line at y_curr
                                pinned_painter.as_mut().draw_line(x_curr, y_curr, x_next, y_curr);
                                // vertical transition line at x_next
                                if (s.data_y[k] - s.data_y[k + 1]).abs() > f64::EPSILON {
                                    pinned_painter.as_mut().draw_line(x_next, y_curr, x_next, y_next);
                                }
                            }
                        } else {
                            // analog: straight lines between points
                            for k in 0..(s.data_x.len() - 1) {
                                if this.x_log_scale && (s.data_x[k] <= 0.0 || s.data_x[k + 1] <= 0.0) {
                                    continue;
                                }
                                if this.y_log_scale && (s.data_y[k] <= 0.0 || s.data_y[k + 1] <= 0.0) {
                                    continue;
                                }
                                let x1 = if this.x_log_scale {
                                    let log_min = x_min_val.log10();
                                    let log_max = x_max_val.log10();
                                    plot_x + ((s.data_x[k].log10() - log_min) / (log_max - log_min)) * plot_width
                                } else {
                                    plot_x + ((s.data_x[k] - x_min_val) / (x_max_val - x_min_val)) * plot_width
                                };
                                let x2 = if this.x_log_scale {
                                    let log_min = x_min_val.log10();
                                    let log_max = x_max_val.log10();
                                    plot_x + ((s.data_x[k + 1].log10() - log_min) / (log_max - log_min)) * plot_width
                                } else {
                                    plot_x + ((s.data_x[k + 1] - x_min_val) / (x_max_val - x_min_val)) * plot_width
                                };
                                let y1 = if this.y_log_scale {
                                    let log_min = y_min_val.log10();
                                    let log_max = y_max_val.log10();
                                    plot_y + plot_height - ((s.data_y[k].log10() - log_min) / (log_max - log_min)) * plot_height
                                } else {
                                    plot_y + plot_height - ((s.data_y[k] - y_min_val) / (y_max_val - y_min_val)) * plot_height
                                };
                                let y2 = if this.y_log_scale {
                                    let log_min = y_min_val.log10();
                                    let log_max = y_max_val.log10();
                                    plot_y + plot_height - ((s.data_y[k + 1].log10() - log_min) / (log_max - log_min)) * plot_height
                                } else {
                                    plot_y + plot_height - ((s.data_y[k + 1] - y_min_val) / (y_max_val - y_min_val)) * plot_height
                                };
                                pinned_painter.as_mut().draw_line(x1, y1, x2, y2);
                            }
                        }
                    }
                    // Draw markers if enabled
                    if s.marker {
                        let marker_size = 6.0;
                        for k in 0..s.data_x.len() {
                            if this.x_log_scale && s.data_x[k] <= 0.0 { continue; }
                            if this.y_log_scale && s.data_y[k] <= 0.0 { continue; }
                            let x_pt = if this.x_log_scale {
                                let log_min = x_min_val.log10();
                                let log_max = x_max_val.log10();
                                plot_x + ((s.data_x[k].log10() - log_min) / (log_max - log_min)) * plot_width
                            } else {
                                plot_x + ((s.data_x[k] - x_min_val) / (x_max_val - x_min_val)) * plot_width
                            };
                            let y_pt = if this.y_log_scale {
                                let log_min = y_min_val.log10();
                                let log_max = y_max_val.log10();
                                plot_y + plot_height - ((s.data_y[k].log10() - log_min) / (log_max - log_min)) * plot_height
                            } else {
                                plot_y + plot_height - ((s.data_y[k] - y_min_val) / (y_max_val - y_min_val)) * plot_height
                            };
                            let rect = QRectF::new(x_pt - marker_size / 2.0, y_pt - marker_size / 2.0, marker_size, marker_size);
                            pinned_painter.as_mut().draw_ellipse(&rect);
                        }
                    }
                } else {
                    // Separate series mode: each series in its own vertical band
                    let n = this.series_list.len() as f64;
                    let band_height = plot_height / n;
                    let band_index = si as f64;
                    let base_y_top = plot_y; // top of overall plot
                    // Each series band spans y from (axis_y - (band_index+1)*band_height) to (axis_y - band_index*band_height)
                    // Here axis_y is overall bottom of plot
                    // Calculate offset for bottom of this band:
                    let band_bottom_y = plot_y + plot_height - band_index * band_height;
                    // band_top_y = band_bottom_y - band_height
                    // Compute series local min/max
                    let min_val = s.data_y.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    let max_val = s.data_y.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                    let local_min = if min_val == max_val { min_val - 0.5 } else { min_val };
                    let local_max = if min_val == max_val { max_val + 0.5 } else { max_val };
                    let (local_min_val, local_max_val) = if this.y_log_scale {
                        if local_max <= 0.0 {
                            (0.1, 1.0)
                        } else {
                            let loc_min = if local_min > 0.0 { local_min } else { (local_max / 1e6).max(1e-9) };
                            (loc_min, local_max)
                        }
                    } else {
                        (local_min, local_max)
                    };
                    // Scale factors
                    let x_scale = plot_width / (this.x_max - this.x_min);
                    let y_scale_local = if this.y_log_scale {
                        (band_height / ((local_max_val).log10() - (local_min_val).log10()).max(1e-9))
                    } else {
                        band_height / (local_max_val - local_min_val)
                    };
                    if s.data_x.len() > 1 {
                        if s.is_digital {
                            for k in 0..(s.data_x.len() - 1) {
                                if this.x_log_scale && (s.data_x[k] <= 0.0 || s.data_x[k + 1] <= 0.0) {
                                    continue;
                                }
                                if this.y_log_scale && (s.data_y[k] <= 0.0 || s.data_y[k + 1] <= 0.0) {
                                    continue;
                                }
                                let x_curr = if this.x_log_scale {
                                    let log_min = x_min_val.log10();
                                    let log_max = x_max_val.log10();
                                    plot_x + ((s.data_x[k].log10() - log_min) / (log_max - log_min)) * plot_width
                                } else {
                                    plot_x + ((s.data_x[k] - this.x_min) / (this.x_max - this.x_min)) * plot_width
                                };
                                let x_next = if this.x_log_scale {
                                    let log_min = x_min_val.log10();
                                    let log_max = x_max_val.log10();
                                    plot_x + ((s.data_x[k + 1].log10() - log_min) / (log_max - log_min)) * plot_width
                                } else {
                                    plot_x + ((s.data_x[k + 1] - this.x_min) / (this.x_max - this.x_min)) * plot_width
                                };
                                let y_curr = if this.y_log_scale {
                                    band_bottom_y - ((s.data_y[k].log10() - local_min_val.log10()) * y_scale_local)
                                } else {
                                    band_bottom_y - ((s.data_y[k] - local_min_val) * y_scale_local)
                                };
                                let y_next = if this.y_log_scale {
                                    band_bottom_y - ((s.data_y[k + 1].log10() - local_min_val.log10()) * y_scale_local)
                                } else {
                                    band_bottom_y - ((s.data_y[k + 1] - local_min_val) * y_scale_local)
                                };
                                pinned_painter.as_mut().draw_line(x_curr, y_curr, x_next, y_curr);
                                if (s.data_y[k] - s.data_y[k + 1]).abs() > f64::EPSILON {
                                    pinned_painter.as_mut().draw_line(x_next, y_curr, x_next, y_next);
                                }
                            }
                        } else {
                            for k in 0..(s.data_x.len() - 1) {
                                if this.x_log_scale && (s.data_x[k] <= 0.0 || s.data_x[k + 1] <= 0.0) {
                                    continue;
                                }
                                if this.y_log_scale && (s.data_y[k] <= 0.0 || s.data_y[k + 1] <= 0.0) {
                                    continue;
                                }
                                let x1 = if this.x_log_scale {
                                    let log_min = x_min_val.log10();
                                    let log_max = x_max_val.log10();
                                    plot_x + ((s.data_x[k].log10() - log_min) / (log_max - log_min)) * plot_width
                                } else {
                                    plot_x + ((s.data_x[k] - this.x_min) / (this.x_max - this.x_min)) * plot_width
                                };
                                let x2 = if this.x_log_scale {
                                    let log_min = x_min_val.log10();
                                    let log_max = x_max_val.log10();
                                    plot_x + ((s.data_x[k + 1].log10() - log_min) / (log_max - log_min)) * plot_width
                                } else {
                                    plot_x + ((s.data_x[k + 1] - this.x_min) / (this.x_max - this.x_min)) * plot_width
                                };
                                let y1 = if this.y_log_scale {
                                    band_bottom_y - ((s.data_y[k].log10() - local_min_val.log10()) * y_scale_local)
                                } else {
                                    band_bottom_y - ((s.data_y[k] - local_min_val) * y_scale_local)
                                };
                                let y2 = if this.y_log_scale {
                                    band_bottom_y - ((s.data_y[k + 1].log10() - local_min_val.log10()) * y_scale_local)
                                } else {
                                    band_bottom_y - ((s.data_y[k + 1] - local_min_val) * y_scale_local)
                                };
                                pinned_painter.as_mut().draw_line(x1, y1, x2, y2);
                            }
                        }
                    }
                    if s.marker {
                        let marker_size = 6.0;
                        for k in 0..s.data_x.len() {
                            if this.x_log_scale && s.data_x[k] <= 0.0 { continue; }
                            if this.y_log_scale && s.data_y[k] <= 0.0 { continue; }
                            let x_pt = if this.x_log_scale {
                                let log_min = x_min_val.log10();
                                let log_max = x_max_val.log10();
                                plot_x + ((s.data_x[k].log10() - log_min) / (log_max - log_min)) * plot_width
                            } else {
                                plot_x + ((s.data_x[k] - this.x_min) / (this.x_max - this.x_min)) * plot_width
                            };
                            let y_pt = if this.y_log_scale {
                                band_bottom_y - ((s.data_y[k].log10() - local_min_val.log10()) * y_scale_local)
                            } else {
                                band_bottom_y - ((s.data_y[k] - local_min_val) * y_scale_local)
                            };
                            let rect = QRectF::new(x_pt - marker_size / 2.0, y_pt - marker_size / 2.0, marker_size, marker_size);
                            pinned_painter.as_mut().draw_ellipse(&rect);
                        }
                    }
                }
            }
            // Draw legend if visible
            if this.legend_visible && !this.series_list.is_empty() {
                // Determine legend placement
                // We'll render each series name with colored line sample
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
                    0 => (plot_x + 5.0, plot_y + 5.0), // top-left
                    1 => (plot_x + plot_width - legend_width - 5.0, plot_y + 5.0), // top-right
                    2 => (plot_x + 5.0, plot_y + plot_height - legend_height - 5.0), // bottom-left
                    3 => (plot_x + plot_width - legend_width - 5.0, plot_y + plot_height - legend_height - 5.0), // bottom-right
                    _ => (plot_x + plot_width - legend_width - 5.0, plot_y + 5.0),
                };
                // Background for legend (semi-transparent)
                let bg_color = if this.dark_mode {
                    QColor::from_rgba(30, 30, 30, 200)
                } else {
                    QColor::from_rgba(255, 255, 255, 200)
                };
                //pinned_painter.as_mut().set_pen(&bg_color, 0.0, 1);
                let bg_rect = QRectF::new(legend_x, legend_y, legend_width, legend_height);
                pinned_painter.as_mut().fill_rect(&bg_rect, &bg_color);
                // Draw each entry
                for (idx, s) in this.series_list.iter().enumerate() {
                    let text = QString::from(&s.name);
                    // Color sample (line or square)
                    let mut legend_pen = QPen::default();
                    legend_pen.set_color(&s.color);
                    legend_pen.set_width(2);
                    legend_pen.set_style(PenStyle::SolidLine);
                    pinned_painter.as_mut().set_pen_with_pen(&legend_pen);
                    let line_y = legend_y + legend_padding + idx as f64 * entry_height + entry_height / 2.0;
                    pinned_painter.as_mut().draw_line(legend_x + 5.0, line_y, legend_x + 15.0, line_y);
                    // Text
                    let mut legend_text_pen = QPen::default();
                    legend_text_pen.set_color(&axis_color);
                    legend_text_pen.set_width(0);
                    legend_text_pen.set_style(PenStyle::SolidLine);
                    pinned_painter.as_mut().set_pen_with_pen(&legend_text_pen);
                    pinned_painter.as_mut().draw_text(legend_x + 20.0, legend_y + legend_padding + idx as f64 * entry_height + 10.0, &text);
                }
            }
            // Draw cursor lines and differences
            let mut cursor_pen = QPen::default();
            cursor_pen.set_color(&axis_color);
            cursor_pen.set_width(1);
            cursor_pen.set_style(PenStyle::DashLine);
            pinned_painter.as_mut().set_pen_with_pen(&cursor_pen);
            // Vertical cursors
            for x_val in &this.cursor_x_positions {
                if *x_val >= x_min_val && *x_val <= x_max_val && (!this.x_log_scale || *x_val > 0.0) {
                    let x_pix = if this.x_log_scale {
                        let log_min = x_min_val.log10();
                        let log_max = x_max_val.log10();
                        plot_x + ((*x_val).log10() - log_min) / (log_max - log_min) * plot_width
                    } else {
                        plot_x + ((*x_val - x_min_val) / (x_max_val - x_min_val)) * plot_width
                    };
                    pinned_painter.as_mut().draw_line(x_pix, plot_y, x_pix, plot_y + plot_height);
                }
            }
            // Horizontal cursors (if not separate series)
            if !this.separate_series {
                for y_val in &this.cursor_y_positions {
                    if *y_val >= y_min_val && *y_val <= y_max_val && (!this.y_log_scale || *y_val > 0.0) {
                        let y_pix = plot_y + plot_height - (if this.y_log_scale {
                            let log_min = y_min_val.log10();
                            let log_max = y_max_val.log10();
                            ((*y_val).log10() - log_min) / (log_max - log_min) * plot_height
                        } else {
                            ((*y_val - y_min_val) / (y_max_val - y_min_val)) * plot_height
                        });
                        pinned_painter.as_mut().draw_line(plot_x, y_pix, plot_x + plot_width, y_pix);
                    }
                }
            }
            // Cursor difference labels
            let mut diff_pen = QPen::default();
            diff_pen.set_color(&axis_color);
            diff_pen.set_width(0);
            diff_pen.set_style(PenStyle::SolidLine);
            pinned_painter.as_mut().set_pen_with_pen(&diff_pen);
            if this.cursor_x_positions.len() == 2 {
                let x1 = this.cursor_x_positions[0];
                let x2 = this.cursor_x_positions[1];
                if !(this.x_log_scale && (x1 <= 0.0 || x2 <= 0.0)) {
                    let dx = (x2 - x1).abs();
                    let label = QString::from(&format!("ΔX: {}", self.as_ref().format_value(dx).to_string()));
                    let text_width = label.to_string().len() as f64 * 7.0;
                    let text_x = plot_x + plot_width / 2.0 - text_width / 2.0;
                    let text_y = plot_y + 15.0;
                    pinned_painter.as_mut().draw_text(text_x, text_y, &label);
                }
            }
            if !this.separate_series && this.cursor_y_positions.len() == 2 {
                let y1 = this.cursor_y_positions[0];
                let y2 = this.cursor_y_positions[1];
                if !(this.y_log_scale && (y1 <= 0.0 || y2 <= 0.0)) {
                    let dy = (y2 - y1).abs();
                    let label = QString::from(&format!("ΔY: {}", self.as_ref().format_value(dy).to_string()));
                    let text_x = plot_x + 10.0;
                    let mid_y = {
                        let y1_pix = plot_y + plot_height - (if this.y_log_scale {
                            let log_min = y_min_val.log10();
                            let log_max = y_max_val.log10();
                            ((y1).log10() - log_min) / (log_max - log_min) * plot_height
                        } else {
                            ((y1 - y_min_val) / (y_max_val - y_min_val)) * plot_height
                        });
                        let y2_pix = plot_y + plot_height - (if this.y_log_scale {
                            let log_min = y_min_val.log10();
                            let log_max = y_max_val.log10();
                            ((y2).log10() - log_min) / (log_max - log_min) * plot_height
                        } else {
                            ((y2 - y_min_val) / (y_max_val - y_min_val)) * plot_height
                        });
                        (y1_pix + y2_pix) / 2.0
                    };
                    let text_y = mid_y + 4.0;
                    pinned_painter.as_mut().draw_text(text_x, text_y, &label);
                }
            }
        }
    }
}
