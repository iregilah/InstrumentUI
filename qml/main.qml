// qml/main.qml
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12
import InstrumentUI 1.0

ApplicationWindow {
    id: mainWindow
    width: 1000
    height: 700
    title: qsTr("Oscilloscope")
    visible: true

    OscilloObject {
        id: oscillo
        objectName: "oscilloObject"
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 8

        /* --------------- felső gombsor --------------- */
        RowLayout {
            spacing: 8
            Button { text: "ℹ";   onClicked: oscillo.info_clicked() }
            Button { text: "⚙";   onClicked: oscillo.settings_clicked() }
            Button { text: "Auto"; onClicked: oscillo.autoscale() }
            Button { text: ">_";   onClicked: oscillo.console_clicked() }
            Button { text: "💾";   onClicked: oscillo.save_config() }
            Button { text: "↑";    onClicked: oscillo.load_config() }
            Button { text: "📄";   onClicked: oscillo.toggle_console_log() }
        }

        /* --------------- kép + trigger + timebase --------------- */
        RowLayout {
            spacing: 8
            Layout.fillWidth: true
            Layout.fillHeight: true

            /* scope‑kép */
            Rectangle {
                Layout.preferredWidth: 600
                Layout.fillHeight: true
                border.color: "#888"
                border.width: 1

                Image { anchors.fill: parent; source: oscillo.scopeImageUrl }

                Rectangle { width: 2; height: parent.height; x: parent.width / 2 - 1; color: "#ccc" }

                Button {
                    id: zoomBtn
                    text: "🔍"
                    width: 20; height: 20
                    anchors.top: parent.top; anchors.topMargin: 4
                    anchors.right: parent.right; anchors.rightMargin: 4
                }
                Button {
                    text: "↔"
                    width: 20; height: 20
                    anchors.top: parent.top; anchors.topMargin: 4
                    anchors.right: zoomBtn.left; anchors.rightMargin: 4
                }
            }

            /* timebase + trigger oszlop */
            ColumnLayout {
                spacing: 8
                Layout.fillHeight: true

                GroupBox {
                    title: qsTr("HORIZ")
                    ColumnLayout {
                        spacing: 4
                        RowLayout {
                            spacing: 4
                            Label { text: qsTr("F") }
                            Slider {
                                from: 1; to: 100; value: 50
                                onValueChanged: oscillo.timebase_changed(value)
                            }
                            Label { text: qsTr("Δ") }
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: qsTr("B") }
                            Slider {
                                from: -100; to: 100; value: 0
                                onValueChanged: oscillo.time_offset_changed(value)
                            }
                            Label { text: qsTr("D") }
                        }
                    }
                }

                GroupBox {
                    title: qsTr("Trigger")
                    ColumnLayout {
                        spacing: 8
                        RowLayout {
                            spacing: 4
                            Slider {
                                id: trigSlider
                                orientation: Qt.Vertical
                                from: -100; to: 100; value: 0
                                onValueChanged: {
                                    let v = Math.round(value)
                                    if (spinLevel.value !== v) spinLevel.value = v
                                    oscillo.trigger_level_changed(v)
                                }
                            }
                            SpinBox {
                                id: spinLevel
                                minimumValue: -100
                                maximumValue: 100
                                value: 0
                                onValueChanged: {
                                    if (trigSlider.value !== value) trigSlider.value = value
                                    oscillo.trigger_level_changed(value)
                                }
                            }
                        }
                        RowLayout {
                            spacing: 4
                            Button { text: "↑"; onClicked: oscillo.trigger_slope_up() }
                            Button { text: "↓"; onClicked: oscillo.trigger_slope_down() }
                        }
                        ComboBox {
                            id: trigSourceBox
                            model: ["CH1","CH2","CH3","CH4","EXT"]
                            onActivated: oscillo.trigger_source_selected(model[index])
                        }
                        RowLayout {
                            spacing: 4
                            Button { text: qsTr("SINGLE");   onClicked: oscillo.single_trigger() }
                            Button { text: qsTr("RUN/STOP"); onClicked: oscillo.run_stop() }
                        }
                    }
                }
            }
        }

        /* --------------- alsó tab‑panelek --------------- */
        RowLayout {
            spacing: 8

            /* CH1‑4 panelek */
            ColumnLayout {
                Layout.preferredWidth: 200
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                spacing: 0

                TabBar {
                    id: channelTabBar
                    Layout.fillWidth: true
                    TabButton { text: qsTr("CH1") }
                    TabButton { text: qsTr("CH2") }
                    TabButton { text: qsTr("CH3") }
                    TabButton { text: qsTr("CH4") }
                }

                StackLayout {
                    id: channelStack
                    Layout.fillWidth: true
                    currentIndex: channelTabBar.currentIndex

                    /* ---------- CH1 ---------- */
                    ColumnLayout {
                        spacing: 4
                        Switch { text: qsTr("Enable"); onToggled: oscillo.ch1_enable_changed(checked) }

                        RowLayout {
                            spacing: 4
                            Label { text: qsTr("F") }
                            Slider { from: 1; to: 100; value: 50; onValueChanged: oscillo.ch1_scale_changed(value) }
                            Label { text: qsTr("A") }
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: qsTr("B") }
                            Slider { from: -100; to: 100; value: 0; onValueChanged: oscillo.ch1_offset_changed(value) }
                            Label { text: qsTr("D") }
                        }

                        GridLayout {
                            columns: 2
                            columnSpacing: 4; rowSpacing: 4

                            Label { text: qsTr("Coupling:"); Layout.column: 0; Layout.row: 0 }
                            ComboBox {
                                model: ["DC","AC","GND"]
                                Layout.column: 1; Layout.row: 0
                                onActivated: oscillo.ch1_coupling_selected(model[index])
                            }

                            Label { text: qsTr("Probe:");    Layout.column: 0; Layout.row: 1 }
                            ComboBox {
                                model: ["1×","10×"]
                                Layout.column: 1; Layout.row: 1
                                onActivated: oscillo.ch1_probe_selected(model[index])
                            }

                            Label { text: qsTr("Current:");  Layout.column: 0; Layout.row: 2 }
                            TextField { text: "0.00"; readOnly: true; Layout.column: 1; Layout.row: 2 }
                        }

                        CheckBox {
                            text: qsTr("AVG")
                            checked: oscillo.avgEnabled
                            onToggled: oscillo.average_toggled(checked)
                        }
                    }

                    /* ---------- CH2 ---------- */
                    ColumnLayout {
                        spacing: 4
                        Switch { text: qsTr("Enable"); onToggled: oscillo.ch2_enable_changed(checked) }

                        RowLayout {
                            spacing: 4
                            Label { text: qsTr("F") }
                            Slider { from: 1; to: 100; value: 50; onValueChanged: oscillo.ch2_scale_changed(value) }
                            Label { text: qsTr("A") }
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: qsTr("B") }
                            Slider { from: -100; to: 100; value: 0; onValueChanged: oscillo.ch2_offset_changed(value) }
                            Label { text: qsTr("D") }
                        }

                        GridLayout {
                            columns: 2
                            columnSpacing: 4; rowSpacing: 4

                            Label { text: qsTr("Coupling:"); Layout.column: 0; Layout.row: 0 }
                            ComboBox {
                                model: ["DC","AC","GND"]
                                Layout.column: 1; Layout.row: 0
                                onActivated: oscillo.ch2_coupling_selected(model[index])
                            }

                            Label { text: qsTr("Probe:");    Layout.column: 0; Layout.row: 1 }
                            ComboBox {
                                model: ["1×","10×"]
                                Layout.column: 1; Layout.row: 1
                                onActivated: oscillo.ch2_probe_selected(model[index])
                            }

                            Label { text: qsTr("Current:");  Layout.column: 0; Layout.row: 2 }
                            TextField { text: "0.00"; readOnly: true; Layout.column: 1; Layout.row: 2 }
                        }

                        CheckBox {
                            text: qsTr("AVG")
                            checked: oscillo.avgEnabled
                            onToggled: oscillo.average_toggled(checked)
                        }
                    }

                    /* ---------- CH3 ---------- */
                    ColumnLayout {
                        spacing: 4
                        Switch { text: qsTr("Enable"); onToggled: oscillo.ch3_enable_changed(checked) }

                        RowLayout {
                            spacing: 4
                            Label { text: qsTr("F") }
                            Slider { from: 1; to: 100; value: 50; onValueChanged: oscillo.ch3_scale_changed(value) }
                            Label { text: qsTr("A") }
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: qsTr("B") }
                            Slider { from: -100; to: 100; value: 0; onValueChanged: oscillo.ch3_offset_changed(value) }
                            Label { text: qsTr("D") }
                        }

                        GridLayout {
                            columns: 2
                            columnSpacing: 4; rowSpacing: 4

                            Label { text: qsTr("Coupling:"); Layout.column: 0; Layout.row: 0 }
                            ComboBox {
                                model: ["DC","AC","GND"]
                                Layout.column: 1; Layout.row: 0
                                onActivated: oscillo.ch3_coupling_selected(model[index])
                            }

                            Label { text: qsTr("Probe:");    Layout.column: 0; Layout.row: 1 }
                            ComboBox {
                                model: ["1×","10×"]
                                Layout.column: 1; Layout.row: 1
                                onActivated: oscillo.ch3_probe_selected(model[index])
                            }

                            Label { text: qsTr("Current:");  Layout.column: 0; Layout.row: 2 }
                            TextField { text: "0.00"; readOnly: true; Layout.column: 1; Layout.row: 2 }
                        }

                        CheckBox {
                            text: qsTr("AVG")
                            checked: oscillo.avgEnabled
                            onToggled: oscillo.average_toggled(checked)
                        }
                    }

                    /* ---------- CH4 ---------- */
                    ColumnLayout {
                        spacing: 4
                        Switch { text: qsTr("Enable"); onToggled: oscillo.ch4_enable_changed(checked) }

                        RowLayout {
                            spacing: 4
                            Label { text: qsTr("F") }
                            Slider { from: 1; to: 100; value: 50; onValueChanged: oscillo.ch4_scale_changed(value) }
                            Label { text: qsTr("A") }
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: qsTr("B") }
                            Slider { from: -100; to: 100; value: 0; onValueChanged: oscillo.ch4_offset_changed(value) }
                            Label { text: qsTr("D") }
                        }

                        GridLayout {
                            columns: 2
                            columnSpacing: 4; rowSpacing: 4

                            Label { text: qsTr("Coupling:"); Layout.column: 0; Layout.row: 0 }
                            ComboBox {
                                model: ["DC","AC","GND"]
                                Layout.column: 1; Layout.row: 0
                                onActivated: oscillo.ch4_coupling_selected(model[index])
                            }

                            Label { text: qsTr("Probe:");    Layout.column: 0; Layout.row: 1 }
                            ComboBox {
                                model: ["1×","10×"]
                                Layout.column: 1; Layout.row: 1
                                onActivated: oscillo.ch4_probe_selected(model[index])
                            }

                            Label { text: qsTr("Current:");  Layout.column: 0; Layout.row: 2 }
                            TextField { text: "0.00"; readOnly: true; Layout.column: 1; Layout.row: 2 }
                        }

                        CheckBox {
                            text: qsTr("AVG")
                            checked: oscillo.avgEnabled
                            onToggled: oscillo.average_toggled(checked)
                        }
                    }
                }
            }

            /* Measure / Cursor / Math */
            ColumnLayout {
                Layout.preferredWidth: 100
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                spacing: 0

                TabBar {
                    id: miscTabBar
                    Layout.fillWidth: true
                    TabButton { text: qsTr("Measure") }
                    TabButton { text: qsTr("Cursor") }
                    TabButton { text: qsTr("Math") }
                }

                StackLayout {
                    id: miscStack
                    Layout.fillWidth: true
                    currentIndex: miscTabBar.currentIndex
                    ColumnLayout { }   /* Measure */
                    ColumnLayout { }   /* Cursor  */
                    ColumnLayout { }   /* Math    */
                }
            }
        }
    }

    Component.onCompleted: oscillo.start_capture()
}
