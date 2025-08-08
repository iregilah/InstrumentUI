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

        RowLayout {
            spacing: 8
            Button {
                text: "ℹ"
                onClicked: oscillo.info_clicked()
            }
            Button {
                text: "⚙"
                onClicked: {
                    oscillo.settings_clicked()
                    settingsDialog.open()
                }
            }
            Button {
                text: "Auto"
                onClicked: oscillo.autoscale()
            }
            Button {
                text: ">_"
                onClicked: oscillo.console_clicked()
            }
            Button {
                text: "💾"
                onClicked: oscillo.save_config()
            }
            Button {
                text: "↑"
                onClicked: oscillo.load_config()
            }
            Button {
                text: "📄"
                onClicked: oscillo.toggle_console_log()
            }
        }

        RowLayout {
            spacing: 8
            Layout.fillWidth: true
            Layout.fillHeight: true

            Rectangle {
                Layout.preferredWidth: 600
                Layout.fillHeight: true
                border.color: "#888"
                border.width: 1

                Image {
                    anchors.fill: parent
                    source: oscillo.scopeImageUrl
                }

                Rectangle {
                    width: 2
                    height: parent.height
                    x: parent.width / 2 - 1
                    color: "#ccc"
                }

                Button {
                    id: zoomBtn
                    text: "🔍"
                    width: 20
                    height: 20
                    anchors.top: parent.top
                    anchors.topMargin: 4
                    anchors.right: parent.right
                    anchors.rightMargin: 4
                }
                Button {
                    text: "↔"
                    width: 20
                    height: 20
                    anchors.top: parent.top
                    anchors.topMargin: 4
                    anchors.right: zoomBtn.left
                    anchors.rightMargin: 4
                }
            }

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
                                from: 1
                                to: 100
                                value: 50
                                onValueChanged: oscillo.timebase_changed(value)
                            }
                            Label { text: qsTr("Δ") }
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: qsTr("B") }
                            Slider {
                                from: -100
                                to: 100
                                value: 0
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
                                from: -100
                                to: 100
                                value: 0
                                onValueChanged: {
                                    let v = Math.round(value)
                                    if (spinLevel.value !== v)
                                        spinLevel.value = v
                                    oscillo.trigger_level_changed(v)
                                }
                            }
                            SpinBox {
                                id: spinLevel
                                from: -100
                                to: 100
                                value: 0
                                onValueChanged: {
                                    if (trigSlider.value !== value)
                                        trigSlider.value = value
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
                            model: ["CH1", "CH2", "CH3", "CH4", "EXT"]
                            onActivated: oscillo.trigger_source_selected(model[index])
                        }
                        RowLayout {
                            spacing: 4
                            Button { text: qsTr("SINGLE"); onClicked: oscillo.single_trigger() }
                            Button { text: qsTr("RUN/STOP"); onClicked: oscillo.run_stop() }
                        }
                    }
                }
            }
        }

        RowLayout {
            spacing: 8

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
                                model: ["DC", "AC", "GND"]
                                Layout.column: 1; Layout.row: 0
                                onActivated: oscillo.ch1_coupling_selected(model[index])
                            }
                            Label { text: qsTr("Probe:"); Layout.column: 0; Layout.row: 1 }
                            ComboBox {
                                model: ["1×", "10×"]
                                Layout.column: 1; Layout.row: 1
                                onActivated: oscillo.ch1_probe_selected(model[index])
                            }
                            Label { text: qsTr("Current:"); Layout.column: 0; Layout.row: 2 }
                            TextField { text: "0.00"; readOnly: true; Layout.column: 1; Layout.row: 2 }
                        }
                        CheckBox {
                            text: qsTr("AVG")
                            checked: oscillo.avgEnabled
                            onToggled: oscillo.average_toggled(checked)
                        }
                    }

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
                                model: ["DC", "AC", "GND"]
                                Layout.column: 1; Layout.row: 0
                                onActivated: oscillo.ch2_coupling_selected(model[index])
                            }
                            Label { text: qsTr("Probe:"); Layout.column: 0; Layout.row: 1 }
                            ComboBox {
                                model: ["1×", "10×"]
                                Layout.column: 1; Layout.row: 1
                                onActivated: oscillo.ch2_probe_selected(model[index])
                            }
                            Label { text: qsTr("Current:"); Layout.column: 0; Layout.row: 2 }
                            TextField { text: "0.00"; readOnly: true; Layout.column: 1; Layout.row: 2 }
                        }
                        CheckBox {
                            text: qsTr("AVG")
                            checked: oscillo.avgEnabled
                            onToggled: oscillo.average_toggled(checked)
                        }
                    }

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
                                model: ["DC", "AC", "GND"]
                                Layout.column: 1; Layout.row: 0
                                onActivated: oscillo.ch3_coupling_selected(model[index])
                            }
                            Label { text: qsTr("Probe:"); Layout.column: 0; Layout.row: 1 }
                            ComboBox {
                                model: ["1×", "10×"]
                                Layout.column: 1; Layout.row: 1
                                onActivated: oscillo.ch3_probe_selected(model[index])
                            }
                            Label { text: qsTr("Current:"); Layout.column: 0; Layout.row: 2 }
                            TextField { text: "0.00"; readOnly: true; Layout.column: 1; Layout.row: 2 }
                        }
                        CheckBox {
                            text: qsTr("AVG")
                            checked: oscillo.avgEnabled
                            onToggled: oscillo.average_toggled(checked)
                        }
                    }

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
                                model: ["DC", "AC", "GND"]
                                Layout.column: 1; Layout.row: 0
                                onActivated: oscillo.ch4_coupling_selected(model[index])
                            }
                            Label { text: qsTr("Probe:"); Layout.column: 0; Layout.row: 1 }
                            ComboBox {
                                model: ["1×", "10×"]
                                Layout.column: 1; Layout.row: 1
                                onActivated: oscillo.ch4_probe_selected(model[index])
                            }
                            Label { text: qsTr("Current:"); Layout.column: 0; Layout.row: 2 }
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
                    ColumnLayout { }
                    ColumnLayout { }
                    ColumnLayout { }
                }
            }
        }
    }

    Dialog {
        id: settingsDialog
        title: qsTr("Settings")
        modality: Qt.ApplicationModal
        visible: false
        width: 300
        height: contentItem.implicitHeight + 20
        ColumnLayout {
            anchors.fill: parent
            spacing: 8
            padding: 8
            RowLayout {
                spacing: 4
                Label { text: qsTr("Theme:") }
                ComboBox {
                    id: themeCombo
                    model: ["Light", "Dark"]
                    currentIndex: 1
                    onActivated: {
                        console.log("Theme changed to " + model[index])
                        oscillo.setTheme(index === 1)
                        // Note: Material theme can be applied dynamically if style is Material.
                        if (styleCombo.currentText === "Material") {
                            console.log("Material theme selection: " + model[index])
                        }
                    }
                }
            }
            RowLayout {
                spacing: 4
                Label { text: qsTr("Style:") }
                ComboBox {
                    id: styleCombo
                    model: ["Fusion", "Imagine", "Material"]
                    currentIndex: 2
                    onActivated: {
                        console.log("Style changed to " + model[index])
                        oscillo.setStyle(model[index])
                    }
                }
            }
            Button {
                text: qsTr("Close")
                Layout.alignment: Qt.AlignRight
                onClicked: settingsDialog.close()
            }
        }
    }

    Component.onCompleted: {
        oscillo.start_capture()
        // Load the AWG (Function Generator) window
        var comp = Qt.createComponent("qrc:/qt/qml/InstrumentUI/qml/awg.qml")
        if (comp.status === Component.Ready) {
            var obj = comp.createObject(null)
            if (obj !== null) {
                console.log("AWG window loaded successfully")
            } else {
                console.log("Failed to create AWG window")
            }
        } else if (comp.status === Component.Error) {
            console.log("Error loading AWG component: " + comp.errorString())
        } else {
            comp.statusChanged.connect(function() {
                if (comp.status === Component.Ready) {
                    var obj2 = comp.createObject(null)
                    console.log("AWG window loaded (delayed), success: " + (obj2 !== null))
                } else if (comp.status === Component.Error) {
                    console.log("Error loading AWG component: " + comp.errorString())
                }
            })
        }
    }
}
