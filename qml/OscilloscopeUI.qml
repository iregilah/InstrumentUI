// qml/OscilloscopeUI.qml
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15
import InstrumentUI 1.0

Window {
    id: oscWindow
    width: 1000
    height: 700
    visible: true
    color: "#1e1e1e"
    title: "Oscilloscope"

    OscilloscopeBackend {
        id: backend
    }

    Column {
        anchors.fill: parent
        spacing: 8

        // Top toolbar buttons
        Row {
            spacing: 8
            Button { text: "\u{2139}"; onClicked: backend.info_clicked() }    // ℹ
            Button { text: "\u{2699}"; onClicked: backend.settings_clicked() } // ⚙
            Button { text: "Auto"; onClicked: backend.autoscale_clicked() }
            Button { text: ">_"; onClicked: backend.console_clicked() }
            Button { text: "\u{1F4BE}"; onClicked: backend.save_config_clicked() } // 💾
            Button { text: "\u{2191}"; onClicked: backend.load_config_clicked() }   // ↑
            Button { text: "\u{1F4C4}"; onClicked: {} } // 📄 (optional log toggle)
        }

        // Main content area
        Row {
            spacing: 8
            // Oscilloscope display with image
            Rectangle {
                id: scopeArea
                width: parent.width * 0.75
                anchors.verticalStretch: 1
                border.color: "#888"; border.width: 1
                Image {
                    anchors.fill: parent
                    source: backend.scopeImageUrl
                    fillMode: Image.PreserveAspectFit
                }
                Rectangle {
                    width: 2; height: parent.height
                    anchors.verticalCenter: parent.verticalCenter
                    anchors.horizontalCenter: parent.horizontalCenter
                    color: "#ccc"
                }
                Button {
                    id: zoomBtn
                    width: 20; height: 20; text: "\u{1F50D}"  // 🔍
                    anchors.top: parent.top; anchors.right: parent.right
                    anchors.margins: 4
                }
                Button {
                    width: 20; height: 20; text: "\u{2194}"   // ↔
                    anchors.top: parent.top; anchors.right: zoomBtn.left
                    anchors.topMargin: 4; anchors.rightMargin: 4
                }
            }
            // Right-side control panel (Horiz & Trigger)
            Column {
                anchors.verticalStretch: 1
                spacing: 8
                GroupBox {
                    title: "HORIZ"
                    Column {
                        spacing: 4
                        Row {
                            spacing: 4
                            Label { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onValueChanged: backend.timebase_changed(value)
                            }
                            Label { text: "\u{394}" }  // Δ
                        }
                        Row {
                            spacing: 4
                            Label { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onValueChanged: backend.time_offset_changed(value)
                            }
                            Label { text: "D" }
                        }
                    }
                }
                GroupBox {
                    title: "Trigger"
                    Column {
                        spacing: 8
                        Row {
                            spacing: 4
                            Slider {
                                id: trigLevelSlider
                                orientation: Qt.Vertical
                                from: -100; to: 100; value: 0
                                onValueChanged: backend.trigger_level_changed(value.toFixed(0))
                            }
                            SpinBox {
                                from: -100; to: 100
                                value: trigLevelSlider.value
                                onValueModified: {
                                    backend.trigger_level_changed(value)
                                    trigLevelSlider.value = value
                                }
                            }
                        }
                        Row {
                            spacing: 4
                            Button { text: "↑"; onClicked: backend.trigger_slope_up() }
                            Button { text: "↓"; onClicked: backend.trigger_slope_down() }
                        }
                        ComboBox {
                            model: ["CH1", "CH2", "CH3", "CH4", "EXT"]
                            onActivated: backend.trigger_source_selected(currentText)
                        }
                        Row {
                            spacing: 4
                            Button { text: "SINGLE"; onClicked: backend.single_trigger_clicked() }
                            Button { text: "RUN/STOP"; onClicked: backend.run_stop_clicked() }
                        }
                    }
                }
            }
        }

        // Bottom row: Channel tabs and other tabs
        Row {
            spacing: 8
            // Channel settings tabs (CH1-CH4)
            TabView {
                width: parent.width * 0.66
                Tab {
                    title: "CH1"
                    Column {
                        spacing: 4
                        Switch { text: "Enable"; onToggled: backend.ch1_enable_changed(checked) }
                        Row {
                            spacing: 4
                            Label { text: "F" }
                            Slider { from: 1; to: 100; value: 50; onValueChanged: backend.ch1_scale_changed(value) }
                            Label { text: "A" }
                        }
                        Row {
                            spacing: 4
                            Label { text: "B" }
                            Slider { from: -100; to: 100; value: 0; onValueChanged: backend.ch1_offset_changed(value) }
                            Label { text: "D" }
                        }
                        GridLayout {
                            columns: 2; columnSpacing: 4; rowSpacing: 4
                            Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0 }
                            ComboBox {
                                model: ["DC", "AC", "GND"]; Layout.row: 0; Layout.column: 1
                                onActivated: backend.ch1_coupling_selected(currentText)
                            }
                            Label { text: "Probe:"; Layout.row: 1; Layout.column: 0 }
                            ComboBox {
                                model: ["1×", "10×"]; Layout.row: 1; Layout.column: 1
                                onActivated: backend.ch1_probe_selected(currentText)
                            }
                            Label { text: "Current:"; Layout.row: 2; Layout.column: 0 }
                            Label { text: "0.00"; Layout.row: 2; Layout.column: 1 }
                        }
                        CheckBox {
                            text: "AVG"; checked: backend.avgEnabled
                            onToggled: backend.average_toggled(checked)
                        }
                    }
                }
                Tab {
                    title: "CH2"
                    Column {
                        spacing: 4
                        Switch { text: "Enable"; onToggled: backend.ch2_enable_changed(checked) }
                        Row {
                            spacing: 4
                            Label { text: "F" }
                            Slider { from: 1; to: 100; value: 50; onValueChanged: backend.ch2_scale_changed(value) }
                            Label { text: "A" }
                        }
                        Row {
                            spacing: 4
                            Label { text: "B" }
                            Slider { from: -100; to: 100; value: 0; onValueChanged: backend.ch2_offset_changed(value) }
                            Label { text: "D" }
                        }
                        GridLayout {
                            columns: 2; columnSpacing: 4; rowSpacing: 4
                            Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0 }
                            ComboBox {
                                model: ["DC", "AC", "GND"]; Layout.row: 0; Layout.column: 1
                                onActivated: backend.ch2_coupling_selected(currentText)
                            }
                            Label { text: "Probe:"; Layout.row: 1; Layout.column: 0 }
                            ComboBox {
                                model: ["1×", "10×"]; Layout.row: 1; Layout.column: 1
                                onActivated: backend.ch2_probe_selected(currentText)
                            }
                            Label { text: "Current:"; Layout.row: 2; Layout.column: 0 }
                            Label { text: "0.00"; Layout.row: 2; Layout.column: 1 }
                        }
                        CheckBox { text: "AVG"; checked: backend.avgEnabled; onToggled: backend.average_toggled(checked) }
                    }
                }
                Tab {
                    title: "CH3"
                    Column {
                        spacing: 4
                        Switch { text: "Enable"; onToggled: backend.ch3_enable_changed(checked) }
                        Row {
                            spacing: 4
                            Label { text: "F" }
                            Slider { from: 1; to: 100; value: 50; onValueChanged: backend.ch3_scale_changed(value) }
                            Label { text: "A" }
                        }
                        Row {
                            spacing: 4
                            Label { text: "B" }
                            Slider { from: -100; to: 100; value: 0; onValueChanged: backend.ch3_offset_changed(value) }
                            Label { text: "D" }
                        }
                        GridLayout {
                            columns: 2; columnSpacing: 4; rowSpacing: 4
                            Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0 }
                            ComboBox {
                                model: ["DC", "AC", "GND"]; Layout.row: 0; Layout.column: 1
                                onActivated: backend.ch3_coupling_selected(currentText)
                            }
                            Label { text: "Probe:"; Layout.row: 1; Layout.column: 0 }
                            ComboBox {
                                model: ["1×", "10×"]; Layout.row: 1; Layout.column: 1
                                onActivated: backend.ch3_probe_selected(currentText)
                            }
                            Label { text: "Current:"; Layout.row: 2; Layout.column: 0 }
                            Label { text: "0.00"; Layout.row: 2; Layout.column: 1 }
                        }
                        CheckBox { text: "AVG"; checked: backend.avgEnabled; onToggled: backend.average_toggled(checked) }
                    }
                }
                Tab {
                    title: "CH4"
                    Column {
                        spacing: 4
                        Switch { text: "Enable"; onToggled: backend.ch4_enable_changed(checked) }
                        Row {
                            spacing: 4
                            Label { text: "F" }
                            Slider { from: 1; to: 100; value: 50; onValueChanged: backend.ch4_scale_changed(value) }
                            Label { text: "A" }
                        }
                        Row {
                            spacing: 4
                            Label { text: "B" }
                            Slider { from: -100; to: 100; value: 0; onValueChanged: backend.ch4_offset_changed(value) }
                            Label { text: "D" }
                        }
                        GridLayout {
                            columns: 2; columnSpacing: 4; rowSpacing: 4
                            Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0 }
                            ComboBox {
                                model: ["DC", "AC", "GND"]; Layout.row: 0; Layout.column: 1
                                onActivated: backend.ch4_coupling_selected(currentText)
                            }
                            Label { text: "Probe:"; Layout.row: 1; Layout.column: 0 }
                            ComboBox {
                                model: ["1×", "10×"]; Layout.row: 1; Layout.column: 1
                                onActivated: backend.ch4_probe_selected(currentText)
                            }
                            Label { text: "Current:"; Layout.row: 2; Layout.column: 0 }
                            Label { text: "0.00"; Layout.row: 2; Layout.column: 1 }
                        }
                        CheckBox { text: "AVG"; checked: backend.avgEnabled; onToggled: backend.average_toggled(checked) }
                    }
                }
            }
            // Placeholder tabs for Measure/Cursor/Math (empty content)
            TabView {
                width: parent.width * 0.33
                Tab { title: "Measure"; Column { } }
                Tab { title: "Cursor"; Column { } }
                Tab { title: "Math"; Column { } }
            }
        }
    }
}
