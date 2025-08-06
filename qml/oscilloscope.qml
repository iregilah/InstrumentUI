// qml/oscilloscope.qml
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15
import com.instrument.ui 1.0

ApplicationWindow {
    id: mainWindow
    width: 1000
    height: 700
    visible: true
    color: "#1e1e1e"
    title: "Oscilloscope"
    // Instantiate backend object
    OscilloscopeUI {
        id: backend
    }
    // After window shown, initialize instrument and start image thread
    Component.onCompleted: {
        backend.initialize();
    }
    ColumnLayout {
        anchors.fill: parent
        spacing: 8
        // Top toolbar row
        RowLayout {
            spacing: 8
            Button {
                text: "ℹ"
                onClicked: backend.info_clicked()
            }
            Button {
                text: "⚙"
                onClicked: backend.settings_clicked()
            }
            Button {
                text: "Auto"
                onClicked: backend.autoscale_clicked()
            }
            Button {
                text: ">_"
                onClicked: backend.console_clicked()
            }
            Button {
                text: "💾"
                onClicked: backend.save_config_clicked()
            }
            Button {
                text: "↑"
                onClicked: backend.load_config_clicked()
            }
            Button {
                text: "📄"
                // optional console log view toggle (not implemented)
            }
        }
        // Middle row: scope display and controls
        RowLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 8
            // Scope display area
            Rectangle {
                Layout.preferredWidth: 3
                Layout.fillHeight: true
                border.color: "#888"
                border.width: 1
                // Displayed image
                Image {
                    anchors.fill: parent
                    source: qsTr("screenshot.png?rand=%1").arg(Math.random())
                    fillMode: Image.Stretch
                }
                // Center vertical reference line
                Rectangle {
                    width: 2
                    anchors.top: parent.top
                    anchors.bottom: parent.bottom
                    anchors.horizontalCenter: parent.horizontalCenter
                    color: "#ccc"
                }
                // Zoom and horizontal scale buttons in top-right corner
                Button {
                    id: zoomBtn
                    width: 20; height: 20
                    text: "🔍"
                    anchors.top: parent.top; anchors.right: parent.right
                    anchors.topMargin: 4; anchors.rightMargin: 4
                }
                Button {
                    width: 20; height: 20
                    text: "↔"
                    anchors.top: parent.top
                    anchors.right: zoomBtn.left
                    anchors.topMargin: 4; anchors.rightMargin: 4
                }
            }
            // Right side controls (HORIZ and Trigger)
            ColumnLayout {
                Layout.preferredWidth: 1
                Layout.fillHeight: true
                spacing: 8
                GroupBox {
                    title: "HORIZ"
                    Layout.fillWidth: true
                    Column {
                        anchors.fill: parent
                        anchors.margins: 4
                        spacing: 4
                        Row {
                            spacing: 4
                            Text { text: "F" }
                            Slider {
                                id: timebaseSlider
                                from: 1; to: 100; value: 50
                                onMoved: backend.timebase_changed(value)
                            }
                            Text { text: "Δ" }
                        }
                        Row {
                            spacing: 4
                            Text { text: "B" }
                            Slider {
                                id: timeOffsetSlider
                                from: -100; to: 100; value: 0
                                onMoved: backend.time_offset_changed(value)
                            }
                            Text { text: "D" }
                        }
                    }
                }
                GroupBox {
                    title: "Trigger"
                    Layout.fillWidth: true
                    Column {
                        anchors.fill: parent
                        anchors.margins: 4
                        spacing: 8
                        Row {
                            spacing: 4
                            Slider {
                                id: trigSlider
                                from: -100; to: 100; value: 0
                                orientation: Qt.Vertical
                                onMoved: { backend.trigger_level_changed(Math.round(value)) }
                            }
                            SpinBox {
                                from: -100; to: 100
                                value: trigSlider.value
                                onValueModified: { backend.trigger_level_changed(value) }
                            }
                        }
                        Row {
                            spacing: 4
                            Button {
                                text: "↑"
                                onClicked: backend.trigger_slope_up()
                            }
                            Button {
                                text: "↓"
                                onClicked: backend.trigger_slope_down()
                            }
                        }
                        ComboBox {
                            id: trigSourceBox
                            model: ["CH1", "CH2", "CH3", "CH4", "EXT"]
                            onActivated: backend.trigger_source_selected(text)
                        }
                        Row {
                            spacing: 4
                            Button {
                                text: "SINGLE"
                                onClicked: backend.single_trigger_clicked()
                            }
                            Button {
                                text: "RUN/STOP"
                                onClicked: backend.run_stop_clicked()
                            }
                        }
                    }
                }
            }
        }
        // Bottom row: Channel tabs and Measure/other tabs
        RowLayout {
            spacing: 8
            // Channel tabs
            TabView {
                Layout.preferredWidth: 2
                Layout.fillHeight: true
                Tab {
                    title: "CH1"
                    Column {
                        spacing: 4
                        Switch {
                            text: "Enable"
                            onToggled: backend.ch1_enable_changed(checked)
                        }
                        Row {
                            spacing: 4
                            Text { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onMoved: backend.ch1_scale_changed(value)
                            }
                            Text { text: "A" }
                        }
                        Row {
                            spacing: 4
                            Text { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onMoved: backend.ch1_offset_changed(value)
                            }
                            Text { text: "D" }
                        }
                        GridLayout {
                            columns: 2
                            columnSpacing: 4; rowSpacing: 4
                            Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0 }
                            ComboBox {
                                Layout.row: 0; Layout.column: 1
                                model: ["DC", "AC", "GND"]
                                onActivated: backend.ch1_coupling_selected(text)
                            }
                            Label { text: "Probe:"; Layout.row: 1; Layout.column: 0 }
                            ComboBox {
                                Layout.row: 1; Layout.column: 1
                                model: ["1×", "10×"]
                                onActivated: backend.ch1_probe_selected(text)
                            }
                            Label { text: "Current:"; Layout.row: 2; Layout.column: 0 }
                            TextField {
                                Layout.row: 2; Layout.column: 1
                                text: "0.00"; readOnly: true
                            }
                        }
                        CheckBox {
                            id: avgCheck1
                            text: "AVG"
                            checked: backend.avgEnabled
                            onToggled: backend.average_toggled(checked)
                        }
                    }
                }
                Tab {
                    title: "CH2"
                    Column {
                        spacing: 4
                        Switch {
                            text: "Enable"
                            onToggled: backend.ch2_enable_changed(checked)
                        }
                        Row {
                            spacing: 4
                            Text { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onMoved: backend.ch2_scale_changed(value)
                            }
                            Text { text: "A" }
                        }
                        Row {
                            spacing: 4
                            Text { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onMoved: backend.ch2_offset_changed(value)
                            }
                            Text { text: "D" }
                        }
                        GridLayout {
                            columns: 2
                            columnSpacing: 4; rowSpacing: 4
                            Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0 }
                            ComboBox {
                                Layout.row: 0; Layout.column: 1
                                model: ["DC", "AC", "GND"]
                                onActivated: backend.ch2_coupling_selected(text)
                            }
                            Label { text: "Probe:"; Layout.row: 1; Layout.column: 0 }
                            ComboBox {
                                Layout.row: 1; Layout.column: 1
                                model: ["1×", "10×"]
                                onActivated: backend.ch2_probe_selected(text)
                            }
                            Label { text: "Current:"; Layout.row: 2; Layout.column: 0 }
                            TextField {
                                Layout.row: 2; Layout.column: 1
                                text: "0.00"; readOnly: true
                            }
                        }
                        CheckBox {
                            text: "AVG"
                            checked: backend.avgEnabled
                            onToggled: backend.average_toggled(checked)
                        }
                    }
                }
                Tab {
                    title: "CH3"
                    Column {
                        spacing: 4
                        Switch {
                            text: "Enable"
                            onToggled: backend.ch3_enable_changed(checked)
                        }
                        Row {
                            spacing: 4
                            Text { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onMoved: backend.ch3_scale_changed(value)
                            }
                            Text { text: "A" }
                        }
                        Row {
                            spacing: 4
                            Text { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onMoved: backend.ch3_offset_changed(value)
                            }
                            Text { text: "D" }
                        }
                        GridLayout {
                            columns: 2
                            columnSpacing: 4; rowSpacing: 4
                            Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0 }
                            ComboBox {
                                Layout.row: 0; Layout.column: 1
                                model: ["DC", "AC", "GND"]
                                onActivated: backend.ch3_coupling_selected(text)
                            }
                            Label { text: "Probe:"; Layout.row: 1; Layout.column: 0 }
                            ComboBox {
                                Layout.row: 1; Layout.column: 1
                                model: ["1×", "10×"]
                                onActivated: backend.ch3_probe_selected(text)
                            }
                            Label { text: "Current:"; Layout.row: 2; Layout.column: 0 }
                            TextField {
                                Layout.row: 2; Layout.column: 1
                                text: "0.00"; readOnly: true
                            }
                        }
                        CheckBox {
                            text: "AVG"
                            checked: backend.avgEnabled
                            onToggled: backend.average_toggled(checked)
                        }
                    }
                }
                Tab {
                    title: "CH4"
                    Column {
                        spacing: 4
                        Switch {
                            text: "Enable"
                            onToggled: backend.ch4_enable_changed(checked)
                        }
                        Row {
                            spacing: 4
                            Text { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onMoved: backend.ch4_scale_changed(value)
                            }
                            Text { text: "A" }
                        }
                        Row {
                            spacing: 4
                            Text { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onMoved: backend.ch4_offset_changed(value)
                            }
                            Text { text: "D" }
                        }
                        GridLayout {
                            columns: 2
                            columnSpacing: 4; rowSpacing: 4
                            Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0 }
                            ComboBox {
                                Layout.row: 0; Layout.column: 1
                                model: ["DC", "AC", "GND"]
                                onActivated: backend.ch4_coupling_selected(text)
                            }
                            Label { text: "Probe:"; Layout.row: 1; Layout.column: 0 }
                            ComboBox {
                                Layout.row: 1; Layout.column: 1
                                model: ["1×", "10×"]
                                onActivated: backend.ch4_probe_selected(text)
                            }
                            Label { text: "Current:"; Layout.row: 2; Layout.column: 0 }
                            TextField {
                                Layout.row: 2; Layout.column: 1
                                text: "0.00"; readOnly: true
                            }
                        }
                        CheckBox {
                            text: "AVG"
                            checked: backend.avgEnabled
                            onToggled: backend.average_toggled(checked)
                        }
                    }
                }
            }
            // Right-side tabs (Measure, Cursor, Math)
            TabView {
                Layout.preferredWidth: 1
                Layout.fillHeight: true
                Tab { title: "Measure"; Column { } }
                Tab { title: "Cursor"; Column { } }
                Tab { title: "Math"; Column { } }
            }
        }
    }
    // Timer to periodically refresh the displayed image from file
    Timer {
        interval: 250; running: true; repeat: true
        onTriggered: {
            // update image source with a new query parameter to force reload
            mainWindow.findChild(Image).source = qsTr("screenshot.png?rand=%1").arg(Math.random());
        }
    }
}
