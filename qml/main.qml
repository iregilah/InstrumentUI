// qml/main.qml
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import QtQuick.Layouts 1.15
import com.rust.instrument 1.0

ApplicationWindow {
    id: mainWindow
    width: 1000
    height: 700
    visible: true
    title: "Oscilloscope"
    Material.theme: Material.Dark

    ColumnLayout {
        anchors.fill: parent
        spacing: 8

        // Top toolbar row
        RowLayout {
            Layout.fillWidth: true
            spacing: 8
            Button { text: "ℹ"; onClicked: Backend.infoClicked() }
            Button { text: "⚙"; onClicked: Backend.settingsClicked() }
            Button { text: "Auto"; onClicked: Backend.autoscaleClicked() }
            Button { text: ">_"; onClicked: Backend.consoleClicked() }
            Button { text: "💾"; onClicked: Backend.saveConfig() }
            Button { text: "↑"; onClicked: Backend.loadConfig() }
            Button { text: "📄"; /* optional console log view toggle (no action) */ }
        }

        // Middle section: waveform display (left) and trigger controls (right)
        RowLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true

            // Oscilloscope waveform display area
            Rectangle {
                id: scopeArea
                color: "#1e1e1e"
                border.color: "#888"
                border.width: 1
                Layout.preferredWidth: 750
                Layout.fillHeight: true

                // Oscilloscope image
                Image {
                    anchors.fill: parent
                    source: Backend.scopeImageData
                    fillMode: Image.PreserveAspectFit
                }
                // Vertical center line
                Rectangle {
                    width: 2
                    anchors.top: parent.top
                    anchors.bottom: parent.bottom
                    x: scopeArea.width / 2 - 1
                    color: "#ccc"
                }
                // Top-right corner buttons
                Button {
                    id: magnifierBtn
                    text: "🔍"
                    width: 20; height: 20
                    anchors.top: parent.top; anchors.right: parent.right
                    anchors.topMargin: 4; anchors.rightMargin: 4
                }
                Button {
                    text: "↔"
                    width: 20; height: 20
                    anchors.top: parent.top; anchors.right: magnifierBtn.left
                    anchors.topMargin: 4; anchors.rightMargin: 4
                }
            }

            // Right panel: Horizontal & Trigger controls
            ColumnLayout {
                Layout.preferredWidth: 250
                Layout.fillHeight: true
                spacing: 8

                GroupBox {
                    title: "HORIZ"
                    Layout.fillWidth: true
                    ColumnLayout {
                        spacing: 4
                        // Timebase scale
                        RowLayout {
                            spacing: 4
                            Text { text: "F" }
                            Slider {
                                id: timebaseSlider
                                from: 1; to: 100; value: 50
                                onValueChanged: Backend.timebaseChanged(value)
                            }
                            Text { text: "Δ" }
                        }
                        // Time offset
                        RowLayout {
                            spacing: 4
                            Text { text: "B" }
                            Slider {
                                id: timeOffsetSlider
                                from: -100; to: 100; value: 0
                                onValueChanged: Backend.timeOffsetChanged(value)
                            }
                            Text { text: "D" }
                        }
                    }
                }

                GroupBox {
                    title: "Trigger"
                    Layout.fillWidth: true
                    ColumnLayout {
                        spacing: 8
                        // Trigger level slider + spin
                        RowLayout {
                            spacing: 4
                            Slider {
                                id: trigLevelSlider
                                from: -100; to: 100; value: 0
                                orientation: Qt.Vertical
                                Layout.fillHeight: true
                                onValueChanged: {
                                    trigLevelSpin.value = value
                                    Backend.triggerLevelChanged(Math.round(value))
                                }
                            }
                            SpinBox {
                                id: trigLevelSpin
                                from: -100; to: 100; value: 0
                                Layout.alignment: Qt.AlignVCenter
                                onValueChanged: {
                                    trigLevelSlider.value = value
                                }
                            }
                        }
                        // Trigger slope buttons
                        RowLayout {
                            spacing: 4
                            Button { text: "↑"; onClicked: Backend.triggerSlopeUp() }
                            Button { text: "↓"; onClicked: Backend.triggerSlopeDown() }
                        }
                        // Trigger source selection
                        ComboBox {
                            id: trigSourceCombo
                            model: ["CH1","CH2","CH3","CH4","EXT"]
                            currentIndex: 0
                            onCurrentTextChanged: Backend.triggerSourceSelected(currentText)
                        }
                        // Single and Run/Stop buttons
                        RowLayout {
                            spacing: 4
                            Button { text: "SINGLE"; onClicked: Backend.singleTriggerClicked() }
                            Button { text: "RUN/STOP"; onClicked: Backend.runStopClicked() }
                        }
                    }
                }
            }
        }

        // Bottom section: Channel settings tabs and Measure/Cursor/Math tabs
        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            // Channels TabView (CH1-CH4)
            TabView {
                id: channelsTabView
                Layout.preferredWidth: 600
                Layout.fillHeight: true

                Tab {
                    title: "CH1"
                    ColumnLayout {
                        spacing: 4
                        // Channel 1 enable
                        Switch {
                            text: "Enable"
                            onToggled: Backend.ch1EnableChanged(checked)
                        }
                        // Channel 1 scale
                        RowLayout {
                            spacing: 4
                            Text { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onValueChanged: Backend.ch1ScaleChanged(value)
                            }
                            Text { text: "A" }
                        }
                        // Channel 1 offset
                        RowLayout {
                            spacing: 4
                            Text { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onValueChanged: Backend.ch1OffsetChanged(value)
                            }
                            Text { text: "D" }
                        }
                        // Channel 1 coupling/probe settings
                        GridLayout {
                            columns: 2
                            columnSpacing: 4; rowSpacing: 4
                            Text { text: "Coupling:"; GridLayout.column: 0; GridLayout.row: 0 }
                            ComboBox {
                                id: ch1Coup
                                model: ["DC","AC","GND"]
                                currentIndex: 0
                                GridLayout.column: 1; GridLayout.row: 0
                                onCurrentTextChanged: Backend.ch1CouplingSelected(currentText)
                            }
                            Text { text: "Probe:"; GridLayout.column: 0; GridLayout.row: 1 }
                            ComboBox {
                                id: ch1Probe
                                model: ["1×","10×"]
                                currentIndex: 0
                                GridLayout.column: 1; GridLayout.row: 1
                                onCurrentTextChanged: Backend.ch1ProbeSelected(currentText)
                            }
                            Text { text: "Current:"; GridLayout.column: 0; GridLayout.row: 2 }
                            TextField {
                                text: "0.00"; readOnly: true
                                GridLayout.column: 1; GridLayout.row: 2
                            }
                        }
                        // Channel 1 averaging checkbox
                        CheckBox {
                            text: "AVG"
                            onToggled: Backend.averageToggled(checked)
                            Connections {
                                target: Backend
                                onAvgEnabledChanged: checked = Backend.avgEnabled
                            }
                        }
                    }
                }
                Tab {
                    title: "CH2"
                    ColumnLayout {
                        spacing: 4
                        Switch {
                            text: "Enable"
                            onToggled: Backend.ch2EnableChanged(checked)
                        }
                        RowLayout {
                            spacing: 4
                            Text { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onValueChanged: Backend.ch2ScaleChanged(value)
                            }
                            Text { text: "A" }
                        }
                        RowLayout {
                            spacing: 4
                            Text { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onValueChanged: Backend.ch2OffsetChanged(value)
                            }
                            Text { text: "D" }
                        }
                        GridLayout {
                            columns: 2
                            columnSpacing: 4; rowSpacing: 4
                            Text { text: "Coupling:"; GridLayout.column: 0; GridLayout.row: 0 }
                            ComboBox {
                                model: ["DC","AC","GND"]
                                currentIndex: 0
                                GridLayout.column: 1; GridLayout.row: 0
                                onCurrentTextChanged: Backend.ch2CouplingSelected(currentText)
                            }
                            Text { text: "Probe:"; GridLayout.column: 0; GridLayout.row: 1 }
                            ComboBox {
                                model: ["1×","10×"]
                                currentIndex: 0
                                GridLayout.column: 1; GridLayout.row: 1
                                onCurrentTextChanged: Backend.ch2ProbeSelected(currentText)
                            }
                            Text { text: "Current:"; GridLayout.column: 0; GridLayout.row: 2 }
                            TextField {
                                text: "0.00"; readOnly: true
                                GridLayout.column: 1; GridLayout.row: 2
                            }
                        }
                        CheckBox {
                            text: "AVG"
                            onToggled: Backend.averageToggled(checked)
                            Connections {
                                target: Backend
                                onAvgEnabledChanged: checked = Backend.avgEnabled
                            }
                        }
                    }
                }
                Tab {
                    title: "CH3"
                    ColumnLayout {
                        spacing: 4
                        Switch {
                            text: "Enable"
                            onToggled: Backend.ch3EnableChanged(checked)
                        }
                        RowLayout {
                            spacing: 4
                            Text { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onValueChanged: Backend.ch3ScaleChanged(value)
                            }
                            Text { text: "A" }
                        }
                        RowLayout {
                            spacing: 4
                            Text { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onValueChanged: Backend.ch3OffsetChanged(value)
                            }
                            Text { text: "D" }
                        }
                        GridLayout {
                            columns: 2
                            columnSpacing: 4; rowSpacing: 4
                            Text { text: "Coupling:"; GridLayout.column: 0; GridLayout.row: 0 }
                            ComboBox {
                                model: ["DC","AC","GND"]
                                currentIndex: 0
                                GridLayout.column: 1; GridLayout.row: 0
                                onCurrentTextChanged: Backend.ch3CouplingSelected(currentText)
                            }
                            Text { text: "Probe:"; GridLayout.column: 0; GridLayout.row: 1 }
                            ComboBox {
                                model: ["1×","10×"]
                                currentIndex: 0
                                GridLayout.column: 1; GridLayout.row: 1
                                onCurrentTextChanged: Backend.ch3ProbeSelected(currentText)
                            }
                            Text { text: "Current:"; GridLayout.column: 0; GridLayout.row: 2 }
                            TextField {
                                text: "0.00"; readOnly: true
                                GridLayout.column: 1; GridLayout.row: 2
                            }
                        }
                        CheckBox {
                            text: "AVG"
                            onToggled: Backend.averageToggled(checked)
                            Connections {
                                target: Backend
                                onAvgEnabledChanged: checked = Backend.avgEnabled
                            }
                        }
                    }
                }
                Tab {
                    title: "CH4"
                    ColumnLayout {
                        spacing: 4
                        Switch {
                            text: "Enable"
                            onToggled: Backend.ch4EnableChanged(checked)
                        }
                        RowLayout {
                            spacing: 4
                            Text { text: "F" }
                            Slider {
                                from: 1; to: 100; value: 50
                                onValueChanged: Backend.ch4ScaleChanged(value)
                            }
                            Text { text: "A" }
                        }
                        RowLayout {
                            spacing: 4
                            Text { text: "B" }
                            Slider {
                                from: -100; to: 100; value: 0
                                onValueChanged: Backend.ch4OffsetChanged(value)
                            }
                            Text { text: "D" }
                        }
                        GridLayout {
                            columns: 2
                            columnSpacing: 4; rowSpacing: 4
                            Text { text: "Coupling:"; GridLayout.column: 0; GridLayout.row: 0 }
                            ComboBox {
                                model: ["DC","AC","GND"]
                                currentIndex: 0
                                GridLayout.column: 1; GridLayout.row: 0
                                onCurrentTextChanged: Backend.ch4CouplingSelected(currentText)
                            }
                            Text { text: "Probe:"; GridLayout.column: 0; GridLayout.row: 1 }
                            ComboBox {
                                model: ["1×","10×"]
                                currentIndex: 0
                                GridLayout.column: 1; GridLayout.row: 1
                                onCurrentTextChanged: Backend.ch4ProbeSelected(currentText)
                            }
                            Text { text: "Current:"; GridLayout.column: 0; GridLayout.row: 2 }
                            TextField {
                                text: "0.00"; readOnly: true
                                GridLayout.column: 1; GridLayout.row: 2
                            }
                        }
                        CheckBox {
                            text: "AVG"
                            onToggled: Backend.averageToggled(checked)
                            Connections {
                                target: Backend
                                onAvgEnabledChanged: checked = Backend.avgEnabled
                            }
                        }
                    }
                }
            }

            // Measure/Cursor/Math tabs (empty content)
            TabView {
                Layout.preferredWidth: 300
                Layout.fillHeight: true
                Tab { title: "Measure"; ColumnLayout { } }
                Tab { title: "Cursor"; ColumnLayout { } }
                Tab { title: "Math"; ColumnLayout { } }
            }
        }
    }

    // Start data acquisition thread after UI shown
    Component.onCompleted: Backend.startAcquisition()
}
