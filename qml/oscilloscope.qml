// qml/oscilloscope.qml

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.15
import com.kdab.cxx_qt.scope 1.0

ApplicationWindow {
    visible: true
    width: 1000
    height: 700
    title: "Oscilloscope"
    color: "#1e1e1e"
    Material.theme: Material.Dark

    OscilloscopeBackend {
        id: oscBackend
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 8

        RowLayout {
            spacing: 8
            Button {
                text: "ℹ"
                onClicked: oscBackend.infoClicked()
            }
            Button {
                text: "⚙"
                onClicked: oscBackend.settingsClicked()
            }
            Button {
                text: "Auto"
                onClicked: oscBackend.autoscaleClicked()
            }
            Button {
                text: ">_"
                onClicked: oscBackend.consoleClicked()
            }
            Button {
                text: "💾"
                onClicked: oscBackend.saveConfigClicked()
            }
            Button {
                text: "↑"
                onClicked: oscBackend.loadConfigClicked()
            }
            Button {
                text: "📄"
                /* optional console log view toggle */
            }
        }

        RowLayout {
            spacing: 8
            Layout.fillHeight: true

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                border.color: "#888"
                border.width: 1
                Image {
                    anchors.fill: parent
                    source: oscBackend.scopeImage
                    fillMode: Image.Stretch
                }
                Rectangle {
                    width: 2
                    height: parent.height
                    x: parent.width / 2 - 1
                    color: "#ccc"
                }
                Button {
                    text: "🔍"
                    width: 20
                    height: 20
                    anchors.top: parent.top
                    anchors.right: parent.right
                    anchors.topMargin: 4
                    anchors.rightMargin: 4
                }
                Button {
                    text: "↔"
                    width: 20
                    height: 20
                    anchors.top: parent.top
                    anchors.right: parent.right
                    anchors.topMargin: 4
                    anchors.rightMargin: 28
                }
            }

            ColumnLayout {
                Layout.preferredWidth: implicitWidth
                Layout.fillHeight: true
                spacing: 8

                GroupBox {
                    title: "HORIZ"
                    Layout.fillWidth: true
                    contentItem: ColumnLayout {
                        spacing: 4
                        RowLayout {
                            spacing: 4
                            Label { text: "F" }
                            Slider {
                                id: timebaseSlider
                                from: 1
                                to: 100
                                value: 50
                                onMoved: oscBackend.timebaseChanged(Math.round(value))
                            }
                            Label { text: "Δ" }
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: "B" }
                            Slider {
                                from: -100
                                to: 100
                                value: 0
                                onMoved: oscBackend.timeOffsetChanged(Math.round(value))
                            }
                            Label { text: "D" }
                        }
                    }
                }

                GroupBox {
                    title: "Trigger"
                    Layout.fillWidth: true
                    contentItem: ColumnLayout {
                        spacing: 8
                        RowLayout {
                            spacing: 4
                            Slider {
                                id: trigLevelSlider
                                orientation: Qt.Vertical
                                from: -100
                                to: 100
                                value: 0
                                Layout.fillHeight: true
                                onMoved: oscBackend.triggerLevelChanged(Math.round(value))
                            }
                            SpinBox {
                                from: -100
                                to: 100
                                value: trigLevelSlider.value
                                onEditingFinished: oscBackend.triggerLevelChanged(value)
                            }
                        }
                        RowLayout {
                            spacing: 4
                            Button { text: "↑"; onClicked: oscBackend.triggerSlopeUp() }
                            Button { text: "↓"; onClicked: oscBackend.triggerSlopeDown() }
                        }
                        ComboBox {
                            id: trigSourceCombo
                            model: ["CH1", "CH2", "CH3", "CH4", "EXT"]
                            currentIndex: 0
                            onActivated: oscBackend.triggerSourceSelected(text)
                        }
                        RowLayout {
                            spacing: 4
                            Button { text: "SINGLE"; onClicked: oscBackend.singleTrigger() }
                            Button { text: "RUN/STOP"; onClicked: oscBackend.runStop() }
                        }
                    }
                }
            }
        }

        RowLayout {
            spacing: 8

            TabView {
                Layout.fillWidth: true
                Tab {
                    title: "CH1"
                    Column {
                        spacing: 4
                        Switch {
                            text: "Enable"
                            onToggled: oscBackend.ch1EnableChanged(checked)
                        }
                        Row {
                            spacing: 4
                            Label { text: "F" }
                            Slider {
                                from: 1
                                to: 100
                                value: 50
                                onMoved: oscBackend.ch1ScaleChanged(Math.round(value))
                            }
                            Label { text: "A" }
                        }
                        Row {
                            spacing: 4
                            Label { text: "B" }
                            Slider {
                                from: -100
                                to: 100
                                value: 0
                                onMoved: oscBackend.ch1OffsetChanged(Math.round(value))
                            }
                            Label { text: "D" }
                        }
                        GridLayout {
                            columns: 2
                            spacing: 4
                            Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0 }
                            ComboBox {
                                model: ["DC", "AC", "GND"]
                                currentIndex: 0
                                Layout.row: 0; Layout.column: 1
                                onActivated: oscBackend.ch1CouplingSelected(text)
                            }
                            Label { text: "Probe:"; Layout.row: 1; Layout.column: 0 }
                            ComboBox {
                                model: ["1×", "10×"]
                                currentIndex: 0
                                Layout.row: 1; Layout.column: 1
                                onActivated: oscBackend.ch1ProbeSelected(text)
                            }
                            Label { text: "Current:"; Layout.row: 2; Layout.column: 0 }
                            TextField {
                                text: "0.00"
                                readOnly: true
                                Layout.row: 2; Layout.column: 1
                            }
                        }
                        CheckBox {
                            text: "AVG"
                            checked: oscBackend.avgEnabled
                            onToggled: oscBackend.averageToggled(checked)
                        }
                    }
                }
                Tab {
                    title: "CH2"
                    Column {
                        spacing: 4
                        Switch { text: "Enable"; onToggled: oscBackend.ch2EnableChanged(checked) }
                        Row {
                            spacing: 4
                            Label { text: "F" }
                            Slider { from: 1; to: 100; value: 50; onMoved: oscBackend.ch2ScaleChanged(Math.round(value)) }
                            Label { text: "A" }
                        }
                        Row {
                            spacing: 4
                            Label { text: "B" }
                            Slider { from: -100; to: 100; value: 0; onMoved: oscBackend.ch2OffsetChanged(Math.round(value)) }
                            Label { text: "D" }
                        }
                        GridLayout {
                            columns: 2
                            spacing: 4
                            Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0 }
                            ComboBox { model: ["DC", "AC", "GND"]; currentIndex: 0; Layout.row: 0; Layout.column: 1; onActivated: oscBackend.ch2CouplingSelected(text) }
                            Label { text: "Probe:"; Layout.row: 1; Layout.column: 0 }
                            ComboBox { model: ["1×", "10×"]; currentIndex: 0; Layout.row: 1; Layout.column: 1; onActivated: oscBackend.ch2ProbeSelected(text) }
                            Label { text: "Current:"; Layout.row: 2; Layout.column: 0 }
                            TextField { text: "0.00"; readOnly: true; Layout.row: 2; Layout.column: 1 }
                        }
                        CheckBox { text: "AVG"; checked: oscBackend.avgEnabled; onToggled: oscBackend.averageToggled(checked) }
                    }
                }
                Tab {
                    title: "CH3"
                    Column {
                        spacing: 4
                        Switch { text: "Enable"; onToggled: oscBackend.ch3EnableChanged(checked) }
                        Row {
                            spacing: 4
                            Label { text: "F" }
                            Slider { from: 1; to: 100; value: 50; onMoved: oscBackend.ch3ScaleChanged(Math.round(value)) }
                            Label { text: "A" }
                        }
                        Row {
                            spacing: 4
                            Label { text: "B" }
                            Slider { from: -100; to: 100; value: 0; onMoved: oscBackend.ch3OffsetChanged(Math.round(value)) }
                            Label { text: "D" }
                        }
                        GridLayout {
                            columns: 2
                            spacing: 4
                            Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0 }
                            ComboBox { model: ["DC", "AC", "GND"]; currentIndex: 0; Layout.row: 0; Layout.column: 1; onActivated: oscBackend.ch3CouplingSelected(text) }
                            Label { text: "Probe:"; Layout.row: 1; Layout.column: 0 }
                            ComboBox { model: ["1×", "10×"]; currentIndex: 0; Layout.row: 1; Layout.column: 1; onActivated: oscBackend.ch3ProbeSelected(text) }
                            Label { text: "Current:"; Layout.row: 2; Layout.column: 0 }
                            TextField { text: "0.00"; readOnly: true; Layout.row: 2; Layout.column: 1 }
                        }
                        CheckBox { text: "AVG"; checked: oscBackend.avgEnabled; onToggled: oscBackend.averageToggled(checked) }
                    }
                }
                Tab {
                    title: "CH4"
                    Column {
                        spacing: 4
                        Switch { text: "Enable"; onToggled: oscBackend.ch4EnableChanged(checked) }
                        Row {
                            spacing: 4
                            Label { text: "F" }
                            Slider { from: 1; to: 100; value: 50; onMoved: oscBackend.ch4ScaleChanged(Math.round(value)) }
                            Label { text: "A" }
                        }
                        Row {
                            spacing: 4
                            Label { text: "B" }
                            Slider { from: -100; to: 100; value: 0; onMoved: oscBackend.ch4OffsetChanged(Math.round(value)) }
                            Label { text: "D" }
                        }
                        GridLayout {
                            columns: 2
                            spacing: 4
                            Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0 }
                            ComboBox { model: ["DC", "AC", "GND"]; currentIndex: 0; Layout.row: 0; Layout.column: 1; onActivated: oscBackend.ch4CouplingSelected(text) }
                            Label { text: "Probe:"; Layout.row: 1; Layout.column: 0 }
                            ComboBox { model: ["1×", "10×"]; currentIndex: 0; Layout.row: 1; Layout.column: 1; onActivated: oscBackend.ch4ProbeSelected(text) }
                            Label { text: "Current:"; Layout.row: 2; Layout.column: 0 }
                            TextField { text: "0.00"; readOnly: true; Layout.row: 2; Layout.column: 1 }
                        }
                        CheckBox { text: "AVG"; checked: oscBackend.avgEnabled; onToggled: oscBackend.averageToggled(checked) }
                    }
                }
            }

            TabView {
                Tab { title: "Measure"; Column { } }
                Tab { title: "Cursor"; Column { } }
                Tab { title: "Math"; Column { } }
            }
        }
    }
}
