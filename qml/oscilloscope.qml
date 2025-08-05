// qml/oscilloscope.qml
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12
import com.kdab.cxx_qt.demo 1.0

ApplicationWindow {
    id: oscWindow
    visible: true
    width: 1000; height: 700
    title: "Oscilloscope"
    // Instantiate the backend object
    OscilloscopeBackend {
        id: oscBackend
    }
    // Layout the UI
    ColumnLayout {
        anchors.fill: parent
        spacing: 8
        // Top toolbar row
        RowLayout {
            spacing: 8; Layout.fillWidth: true
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
                // optional console log view toggle (not implemented)
            }
        }
        // Main content row: scope display and side panels
        RowLayout {
            spacing: 8; Layout.fillWidth: true; Layout.fillHeight: true
            Rectangle {
                id: scopeDisplay
                Layout.preferredWidth: 750
                Layout.fillHeight: true
                border.color: "#888"; border.width: 1
                // scope image
                Image {
                    anchors.fill: parent
                    source: oscBackend.scopeImageData
                    fillMode: Image.PreserveAspectFit
                }
                // center vertical reference line
                Rectangle {
                    width: 2;
                    anchors.top: parent.top; anchors.bottom: parent.bottom
                    anchors.horizontalCenter: parent.horizontalCenter
                    color: "#ccc"
                }
                // overlay buttons
                Button {
                    text: "🔍"; width: 20; height: 20
                    anchors.top: parent.top; anchors.right: parent.right
                    anchors.topMargin: 4; anchors.rightMargin: 4
                }
                Button {
                    text: "↔"; width: 20; height: 20
                    anchors.top: parent.top; anchors.right: parent.right
                    anchors.topMargin: 4; anchors.rightMargin: 28
                }
            }
            ColumnLayout {
                spacing: 8
                Layout.preferredWidth: 250
                Layout.fillHeight: true
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
                                from: 1; to: 100; value: 50
                                onValueChanged: oscBackend.timebaseChanged(value)
                            }
                            Label { text: "Δ" }
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onValueChanged: oscBackend.timeOffsetChanged(value)
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
                                from: -100; to: 100; value: 0
                                orientation: Qt.Vertical
                                Layout.preferredHeight: 100
                                onValueChanged: oscBackend.triggerLevelChanged(Math.round(value))
                            }
                            SpinBox {
                                from: -100; to: 100
                                value: trigLevelSlider.value
                                onValueChanged: oscBackend.triggerLevelChanged(value)
                            }
                        }
                        RowLayout {
                            spacing: 4
                            Button {
                                text: "↑"
                                onClicked: oscBackend.triggerSlopeUp()
                            }
                            Button {
                                text: "↓"
                                onClicked: oscBackend.triggerSlopeDown()
                            }
                        }
                        ComboBox {
                            model: ["CH1", "CH2", "CH3", "CH4", "EXT"]
                            onActivated: oscBackend.triggerSourceSelected(currentText)
                        }
                        RowLayout {
                            spacing: 4
                            Button {
                                text: "SINGLE"
                                onClicked: oscBackend.singleTriggerClicked()
                            }
                            Button {
                                text: "RUN/STOP"
                                onClicked: oscBackend.runStopClicked()
                            }
                        }
                    }
                }
            }
        }
        // Bottom row: channel controls and measurement tabs
        RowLayout {
            spacing: 8; Layout.fillWidth: true
            // Channel tabs
            TabView {
                id: channelTabs
                Layout.preferredWidth: 680
                Layout.fillHeight: true
                Tab {
                    title: "CH1"
                    ColumnLayout {
                        spacing: 4
                        Switch {
                            text: "Enable"
                            onCheckedChanged: oscBackend.ch1EnableChanged(checked)
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onValueChanged: oscBackend.ch1ScaleChanged(value)
                            }
                            Label { text: "A" }
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onValueChanged: oscBackend.ch1OffsetChanged(value)
                            }
                            Label { text: "D" }
                        }
                        GridLayout {
                            columns: 2; columnSpacing: 4; rowSpacing: 4
                            Label { text: "Coupling:"; Layout.column: 0; Layout.row: 0 }
                            ComboBox {
                                model: ["DC", "AC", "GND"]
                                Layout.column: 1; Layout.row: 0
                                onActivated: oscBackend.ch1CouplingSelected(currentText)
                            }
                            Label { text: "Probe:"; Layout.column: 0; Layout.row: 1 }
                            ComboBox {
                                model: ["1×", "10×"]
                                Layout.column: 1; Layout.row: 1
                                onActivated: oscBackend.ch1ProbeSelected(currentText)
                            }
                            Label { text: "Current:"; Layout.column: 0; Layout.row: 2 }
                            TextField {
                                text: "0.00"; readOnly: true
                                Layout.column: 1; Layout.row: 2
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
                    ColumnLayout {
                        spacing: 4
                        Switch {
                            text: "Enable"
                            onCheckedChanged: oscBackend.ch2EnableChanged(checked)
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onValueChanged: oscBackend.ch2ScaleChanged(value)
                            }
                            Label { text: "A" }
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onValueChanged: oscBackend.ch2OffsetChanged(value)
                            }
                            Label { text: "D" }
                        }
                        GridLayout {
                            columns: 2; columnSpacing: 4; rowSpacing: 4
                            Label { text: "Coupling:"; Layout.column: 0; Layout.row: 0 }
                            ComboBox {
                                model: ["DC", "AC", "GND"]
                                Layout.column: 1; Layout.row: 0
                                onActivated: oscBackend.ch2CouplingSelected(currentText)
                            }
                            Label { text: "Probe:"; Layout.column: 0; Layout.row: 1 }
                            ComboBox {
                                model: ["1×", "10×"]
                                Layout.column: 1; Layout.row: 1
                                onActivated: oscBackend.ch2ProbeSelected(currentText)
                            }
                            Label { text: "Current:"; Layout.column: 0; Layout.row: 2 }
                            TextField {
                                text: "0.00"; readOnly: true
                                Layout.column: 1; Layout.row: 2
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
                    title: "CH3"
                    ColumnLayout {
                        spacing: 4
                        Switch {
                            text: "Enable"
                            onCheckedChanged: oscBackend.ch3EnableChanged(checked)
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onValueChanged: oscBackend.ch3ScaleChanged(value)
                            }
                            Label { text: "A" }
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onValueChanged: oscBackend.ch3OffsetChanged(value)
                            }
                            Label { text: "D" }
                        }
                        GridLayout {
                            columns: 2; columnSpacing: 4; rowSpacing: 4
                            Label { text: "Coupling:"; Layout.column: 0; Layout.row: 0 }
                            ComboBox {
                                model: ["DC", "AC", "GND"]
                                Layout.column: 1; Layout.row: 0
                                onActivated: oscBackend.ch3CouplingSelected(currentText)
                            }
                            Label { text: "Probe:"; Layout.column: 0; Layout.row: 1 }
                            ComboBox {
                                model: ["1×", "10×"]
                                Layout.column: 1; Layout.row: 1
                                onActivated: oscBackend.ch3ProbeSelected(currentText)
                            }
                            Label { text: "Current:"; Layout.column: 0; Layout.row: 2 }
                            TextField {
                                text: "0.00"; readOnly: true
                                Layout.column: 1; Layout.row: 2
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
                    title: "CH4"
                    ColumnLayout {
                        spacing: 4
                        Switch {
                            text: "Enable"
                            onCheckedChanged: oscBackend.ch4EnableChanged(checked)
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onValueChanged: oscBackend.ch4ScaleChanged(value)
                            }
                            Label { text: "A" }
                        }
                        RowLayout {
                            spacing: 4
                            Label { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onValueChanged: oscBackend.ch4OffsetChanged(value)
                            }
                            Label { text: "D" }
                        }
                        GridLayout {
                            columns: 2; columnSpacing: 4; rowSpacing: 4
                            Label { text: "Coupling:"; Layout.column: 0; Layout.row: 0 }
                            ComboBox {
                                model: ["DC", "AC", "GND"]
                                Layout.column: 1; Layout.row: 0
                                onActivated: oscBackend.ch4CouplingSelected(currentText)
                            }
                            Label { text: "Probe:"; Layout.column: 0; Layout.row: 1 }
                            ComboBox {
                                model: ["1×", "10×"]
                                Layout.column: 1; Layout.row: 1
                                onActivated: oscBackend.ch4ProbeSelected(currentText)
                            }
                            Label { text: "Current:"; Layout.column: 0; Layout.row: 2 }
                            TextField {
                                text: "0.00"; readOnly: true
                                Layout.column: 1; Layout.row: 2
                            }
                        }
                        CheckBox {
                            text: "AVG"
                            checked: oscBackend.avgEnabled
                            onToggled: oscBackend.averageToggled(checked)
                        }
                    }
                }
            }
            // Secondary tabs (Measure/Cursor/Math)
            TabView {
                Layout.preferredWidth: 300
                Layout.fillHeight: true
                Tab { title: "Measure"; Rectangle { anchors.fill: parent; color: "transparent" } }
                Tab { title: "Cursor"; Rectangle { anchors.fill: parent; color: "transparent" } }
                Tab { title: "Math"; Rectangle { anchors.fill: parent; color: "transparent" } }
            }
        }
    }
    Component.onCompleted: {
        oscBackend.initialize();
    }
}
