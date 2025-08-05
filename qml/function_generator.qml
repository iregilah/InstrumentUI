// qml/function_generator.qml

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.15
import com.kdab.cxx_qt.scope 1.0

ApplicationWindow {
    visible: true
    width: 500
    height: 300
    title: "Function Generator"
    color: "#1e1e1e"
    Material.theme: Material.Dark

    FunctionGeneratorBackend {
        id: awgBackend
    }

    TabView {
        anchors.fill: parent
        Tab {
            title: "CH1"
            Column {
                spacing: 8
                Switch { text: "Enable"; onToggled: awgBackend.awg1EnableChanged(checked) }
                GridLayout {
                    columns: 2
                    spacing: 4
                    Label { text: "Waveform:"; Layout.row: 0; Layout.column: 0 }
                    ComboBox {
                        id: waveSel1
                        model: ["Sine", "Square", "Pulse", "Ramp", "Noise", "Arb"]
                        Layout.row: 0; Layout.column: 1
                        onActivated: {
                            awgBackend.awg1WaveformSelected(text)
                            fileRow1.visible = (text === "Arb")
                            freqSpin1.enabled = (text !== "Noise")
                        }
                    }
                    Label { text: "Frequency [Hz]:"; Layout.row: 1; Layout.column: 0 }
                    SpinBox {
                        id: freqSpin1
                        from: 1
                        to: 25000000
                        stepSize: 100
                        value: 1000
                        Layout.row: 1; Layout.column: 1
                        enabled: waveSel1.currentText !== "Noise"
                        onEditingFinished: awgBackend.awg1FreqChanged(value)
                    }
                    Label { text: "Amplitude [V]:"; Layout.row: 2; Layout.column: 0 }
                    SpinBox {
                        from: 0.0
                        to: 10.0
                        stepSize: 0.1
                        value: 1.0
                        Layout.row: 2; Layout.column: 1
                        onEditingFinished: awgBackend.awg1AmpChanged(value)
                    }
                    Label { text: "Offset [V]:"; Layout.row: 3; Layout.column: 0 }
                    SpinBox {
                        from: -5.0
                        to: 5.0
                        stepSize: 0.1
                        value: 0.0
                        Layout.row: 3; Layout.column: 1
                        onEditingFinished: awgBackend.awg1OffsetChanged(value)
                    }
                }
                Row {
                    id: fileRow1
                    spacing: 4
                    visible: false
                    Label { text: "Arb file:" }
                    TextField {
                        id: filePath1
                        placeholderText: "path/to/waveform.csv"
                        Layout.fillWidth: true
                    }
                    Button { text: "Load"; onClicked: awgBackend.awg1LoadArb(filePath1.text) }
                }
            }
        }
        Tab {
            title: "CH2"
            Column {
                spacing: 8
                Switch { text: "Enable"; onToggled: awgBackend.awg2EnableChanged(checked) }
                GridLayout {
                    columns: 2
                    spacing: 4
                    Label { text: "Waveform:"; Layout.row: 0; Layout.column: 0 }
                    ComboBox {
                        id: waveSel2
                        model: ["Sine", "Square", "Pulse", "Ramp", "Noise", "Arb"]
                        Layout.row: 0; Layout.column: 1
                        onActivated: {
                            awgBackend.awg2WaveformSelected(text)
                            fileRow2.visible = (text === "Arb")
                            freqSpin2.enabled = (text !== "Noise")
                        }
                    }
                    Label { text: "Frequency [Hz]:"; Layout.row: 1; Layout.column: 0 }
                    SpinBox {
                        id: freqSpin2
                        from: 1
                        to: 25000000
                        stepSize: 100
                        value: 1000
                        Layout.row: 1; Layout.column: 1
                        enabled: waveSel2.currentText !== "Noise"
                        onEditingFinished: awgBackend.awg2FreqChanged(value)
                    }
                    Label { text: "Amplitude [V]:"; Layout.row: 2; Layout.column: 0 }
                    SpinBox {
                        from: 0.0
                        to: 10.0
                        stepSize: 0.1
                        value: 1.0
                        Layout.row: 2; Layout.column: 1
                        onEditingFinished: awgBackend.awg2AmpChanged(value)
                    }
                    Label { text: "Offset [V]:"; Layout.row: 3; Layout.column: 0 }
                    SpinBox {
                        from: -5.0
                        to: 5.0
                        stepSize: 0.1
                        value: 0.0
                        Layout.row: 3; Layout.column: 1
                        onEditingFinished: awgBackend.awg2OffsetChanged(value)
                    }
                }
                Row {
                    id: fileRow2
                    spacing: 4
                    visible: false
                    Label { text: "Arb file:" }
                    TextField {
                        id: filePath2
                        placeholderText: "path/to/waveform.csv"
                        Layout.fillWidth: true
                    }
                    Button { text: "Load"; onClicked: awgBackend.awg2LoadArb(filePath2.text) }
                }
            }
        }
    }
}
