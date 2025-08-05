// qml/function_generator.qml
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12
import com.kdab.cxx_qt.demo 1.0

ApplicationWindow {
    id: awgWindow
    visible: true
    width: 500; height: 300
    title: "Function Generator"
    // Instantiate backend object
    FunctionGeneratorBackend {
        id: awgBackend
    }
    // Use dark theme palette
    Component.onCompleted: {
        Qt.callLater(function() {
            // Ensure dark palette for controls if needed (QtQuick Controls 2 uses System palette by default)
        });
    }
    TabView {
        anchors.fill: parent
        Tab {
            title: "CH1"
            ColumnLayout {
                spacing: 8; anchors.fill: parent; anchors.margins: 8
                Switch {
                    text: "Enable"
                    onCheckedChanged: awgBackend.awg1EnableChanged(checked)
                }
                GridLayout {
                    columns: 2; columnSpacing: 4; rowSpacing: 4
                    Label { text: "Waveform:"; Layout.column: 0; Layout.row: 0; verticalAlignment: Label.AlignVCenter }
                    ComboBox {
                        model: ["Sine", "Square", "Pulse", "Ramp", "Noise", "Arb"]
                        Layout.column: 1; Layout.row: 0
                        onActivated: {
                            awgBackend.awg1WaveformSelected(currentText);
                        }
                    }
                    Label { text: "Frequency [Hz]:"; Layout.column: 0; Layout.row: 1; verticalAlignment: Label.AlignVCenter }
                    SpinBox {
                        from: 1; to: 25000000; stepSize: 100; value: 1000
                        enabled: comboWave1.currentText !== "Noise"
                        Layout.column: 1; Layout.row: 1
                        onValueChanged: awgBackend.awg1FreqChanged(value)
                        Component.onCompleted: comboWave1.Keys.onPressed = function(e) { /* no-op to allow arrow keys? */ }
                    }
                    Label { text: "Amplitude [V]:"; Layout.column: 0; Layout.row: 2; verticalAlignment: Label.AlignVCenter }
                    SpinBox {
                        from: 0.0; to: 10.0; stepSize: 0.1; value: 1.0
                        Layout.column: 1; Layout.row: 2
                        onValueChanged: awgBackend.awg1AmpChanged(value)
                    }
                    Label { text: "Offset [V]:"; Layout.column: 0; Layout.row: 3; verticalAlignment: Label.AlignVCenter }
                    SpinBox {
                        from: -5.0; to: 5.0; stepSize: 0.1; value: 0.0
                        Layout.column: 1; Layout.row: 3
                        onValueChanged: awgBackend.awg1OffsetChanged(value)
                    }
                }
                RowLayout {
                    spacing: 4
                    visible: comboWave1.currentText === "Arb"
                    Label { text: "Arb file:"; verticalAlignment: Label.AlignVCenter }
                    TextField {
                        id: filePath1
                        Layout.fillWidth: true
                        placeholderText: "path/to/waveform.csv"
                    }
                    Button {
                        text: "Load"
                        onClicked: awgBackend.awg1LoadArb(filePath1.text)
                    }
                }
            }
            // The waveform ComboBox must be defined after it is referenced (or give it id earlier). We define it invisibly here for id reference.
            ComboBox { id: comboWave1; visible: false }
        }
        Tab {
            title: "CH2"
            ColumnLayout {
                spacing: 8; anchors.fill: parent; anchors.margins: 8
                Switch {
                    text: "Enable"
                    onCheckedChanged: awgBackend.awg2EnableChanged(checked)
                }
                GridLayout {
                    columns: 2; columnSpacing: 4; rowSpacing: 4
                    Label { text: "Waveform:"; Layout.column: 0; Layout.row: 0; verticalAlignment: Label.AlignVCenter }
                    ComboBox {
                        model: ["Sine", "Square", "Pulse", "Ramp", "Noise", "Arb"]
                        Layout.column: 1; Layout.row: 0
                        onActivated: {
                            awgBackend.awg2WaveformSelected(currentText);
                        }
                    }
                    Label { text: "Frequency [Hz]:"; Layout.column: 0; Layout.row: 1; verticalAlignment: Label.AlignVCenter }
                    SpinBox {
                        from: 1; to: 25000000; stepSize: 100; value: 1000
                        enabled: comboWave2.currentText !== "Noise"
                        Layout.column: 1; Layout.row: 1
                        onValueChanged: awgBackend.awg2FreqChanged(value)
                        Component.onCompleted: comboWave2.Keys.onPressed = function(e) {}
                    }
                    Label { text: "Amplitude [V]:"; Layout.column: 0; Layout.row: 2; verticalAlignment: Label.AlignVCenter }
                    SpinBox {
                        from: 0.0; to: 10.0; stepSize: 0.1; value: 1.0
                        Layout.column: 1; Layout.row: 2
                        onValueChanged: awgBackend.awg2AmpChanged(value)
                    }
                    Label { text: "Offset [V]:"; Layout.column: 0; Layout.row: 3; verticalAlignment: Label.AlignVCenter }
                    SpinBox {
                        from: -5.0; to: 5.0; stepSize: 0.1; value: 0.0
                        Layout.column: 1; Layout.row: 3
                        onValueChanged: awgBackend.awg2OffsetChanged(value)
                    }
                }
                RowLayout {
                    spacing: 4
                    visible: comboWave2.currentText === "Arb"
                    Label { text: "Arb file:"; verticalAlignment: Label.AlignVCenter }
                    TextField {
                        id: filePath2
                        Layout.fillWidth: true
                        placeholderText: "path/to/waveform.csv"
                    }
                    Button {
                        text: "Load"
                        onClicked: awgBackend.awg2LoadArb(filePath2.text)
                    }
                }
            }
            ComboBox { id: comboWave2; visible: false }
        }
    }
}
