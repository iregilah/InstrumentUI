// qml/FunctionGeneratorUI.qml
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15
import InstrumentUI 1.0

Window {
    id: awgWindow
    width: 500
    height: 300
    visible: true
    color: "#1e1e1e"
    title: "Function Generator"

    FunctionGeneratorBackend {
        id: awgBackend
    }

    TabView {
        anchors.fill: parent
        Tab {
            title: "CH1"
            Column {
                spacing: 8
                Switch { text: "Enable"; onToggled: awgBackend.awg1_enable_changed(checked) }
                GridLayout {
                    columns: 2; columnSpacing: 4; rowSpacing: 4
                    Label { text: "Waveform:"; Layout.row: 0; Layout.column: 0 }
                    ComboBox {
                        id: waveSel1
                        model: ["Sine", "Square", "Pulse", "Ramp", "Noise", "Arb"]
                        currentIndex: 0
                        Layout.row: 0; Layout.column: 1
                        onActivated: {
                            awgBackend.awg1_waveform_selected(currentText)
                            arbRow1.visible = (currentText === "Arb")
                        }
                    }
                    Label { text: "Frequency [Hz]:"; Layout.row: 1; Layout.column: 0 }
                    SpinBox {
                        from: 1; to: 25000000; stepSize: 100
                        value: 1000
                        enabled: waveSel1.currentText !== "Noise"
                        Layout.row: 1; Layout.column: 1
                        onEditingFinished: awgBackend.awg1_freq_changed(value.toFixed(0))
                    }
                    Label { text: "Amplitude [V]:"; Layout.row: 2; Layout.column: 0 }
                    SpinBox {
                        from: 0.0; to: 10.0; stepSize: 0.1
                        value: 1.0
                        Layout.row: 2; Layout.column: 1
                        onEditingFinished: awgBackend.awg1_amp_changed(value)
                    }
                    Label { text: "Offset [V]:"; Layout.row: 3; Layout.column: 0 }
                    SpinBox {
                        from: -5.0; to: 5.0; stepSize: 0.1
                        value: 0.0
                        Layout.row: 3; Layout.column: 1
                        onEditingFinished: awgBackend.awg1_offset_changed(value)
                    }
                }
                Row {
                    id: arbRow1
                    spacing: 4
                    visible: false
                    Label { text: "Arb file:"; verticalAlignment: Label.AlignVCenter }
                    TextField {
                        id: filePath1
                        placeholderText: "path/to/waveform.csv"
                        Layout.fillWidth: true
                    }
                    Button {
                        text: "Load"
                        onClicked: awgBackend.awg1_load_arb(filePath1.text)
                    }
                }
            }
        }
        Tab {
            title: "CH2"
            Column {
                spacing: 8
                Switch { text: "Enable"; onToggled: awgBackend.awg2_enable_changed(checked) }
                GridLayout {
                    columns: 2; columnSpacing: 4; rowSpacing: 4
                    Label { text: "Waveform:"; Layout.row: 0; Layout.column: 0 }
                    ComboBox {
                        id: waveSel2
                        model: ["Sine", "Square", "Pulse", "Ramp", "Noise", "Arb"]
                        currentIndex: 0
                        Layout.row: 0; Layout.column: 1
                        onActivated: {
                            awgBackend.awg2_waveform_selected(currentText)
                            arbRow2.visible = (currentText === "Arb")
                        }
                    }
                    Label { text: "Frequency [Hz]:"; Layout.row: 1; Layout.column: 0 }
                    SpinBox {
                        from: 1; to: 25000000; stepSize: 100
                        value: 1000
                        enabled: waveSel2.currentText !== "Noise"
                        Layout.row: 1; Layout.column: 1
                        onEditingFinished: awgBackend.awg2_freq_changed(value.toFixed(0))
                    }
                    Label { text: "Amplitude [V]:"; Layout.row: 2; Layout.column: 0 }
                    SpinBox {
                        from: 0.0; to: 10.0; stepSize: 0.1
                        value: 1.0
                        Layout.row: 2; Layout.column: 1
                        onEditingFinished: awgBackend.awg2_amp_changed(value)
                    }
                    Label { text: "Offset [V]:"; Layout.row: 3; Layout.column: 0 }
                    SpinBox {
                        from: -5.0; to: 5.0; stepSize: 0.1
                        value: 0.0
                        Layout.row: 3; Layout.column: 1
                        onEditingFinished: awgBackend.awg2_offset_changed(value)
                    }
                }
                Row {
                    id: arbRow2
                    spacing: 4
                    visible: false
                    Label { text: "Arb file:"; verticalAlignment: Label.AlignVCenter }
                    TextField {
                        id: filePath2
                        placeholderText: "path/to/waveform.csv"
                        Layout.fillWidth: true
                    }
                    Button {
                        text: "Load"
                        onClicked: awgBackend.awg2_load_arb(filePath2.text)
                    }
                }
            }
        }
    }
}
