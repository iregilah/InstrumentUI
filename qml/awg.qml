// qml/awg.qml
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12
import InstrumentUI 1.0
import QtQuick.Controls.Material 2.12

Window {
    id: awgWindow
    width: 500
    height: 300
    title: qsTr("Function Generator")
    visible: true
    color: "#1e1e1e"
    Material.theme: Material.Dark

    AwgObject {
        id: awgObject
        objectName: "awgObject"
    }

    TabBar {
        id: awgTabBar
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        TabButton { text: qsTr("CH1") }
        TabButton { text: qsTr("CH2") }
    }

    StackLayout {
        id: awgStack
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: awgTabBar.bottom
        anchors.bottom: parent.bottom
        currentIndex: awgTabBar.currentIndex

        ColumnLayout {
            spacing: 8
            Switch {
                text: qsTr("Enable")
                onToggled: awgObject.awg1_enable_changed(checked)
            }
            GridLayout {
                columns: 2
                columnSpacing: 4
                rowSpacing: 4

                Label {
                    text: qsTr("Waveform:")
                    Layout.column: 0
                    Layout.row: 0
                }
                ComboBox {
                    id: waveSel1
                    model: ["Sine", "Square", "Pulse", "Ramp", "Noise", "Arb"]
                    currentIndex: 0
                    Layout.column: 1
                    Layout.row: 0
                    onActivated: awgObject.awg1_waveform_selected(model[index])
                }

                Label {
                    text: qsTr("Frequency [Hz]:")
                    Layout.column: 0
                    Layout.row: 1
                }
                SpinBox {
                    id: freqSpin1
                    value: 1000
                    from: 1
                    to: 25000000
                    stepSize: 100
                    enabled: awgObject.currentWaveCh1 !== "Noise"
                    Layout.column: 1
                    Layout.row: 1
                    onValueChanged: awgObject.awg1_freq_changed(value)
                }

                Label {
                    text: qsTr("Amplitude [V]:")
                    Layout.column: 0
                    Layout.row: 2
                }
                SpinBox {
                    id: ampSpin1
                    from: 0
                    to: 100
                    value: 10
                    stepSize: 1
                    Layout.column: 1
                    Layout.row: 2
                    property int decimals: 1
                    validator: DoubleValidator {
                        bottom: Math.min(ampSpin1.from, ampSpin1.to)
                        top: Math.max(ampSpin1.from, ampSpin1.to)
                    }
                    textFromValue: function(value, locale) {
                        return Number(value / 10).toLocaleString(locale, 'f', ampSpin1.decimals)
                    }
                    valueFromText: function(text, locale) {
                        return Number.fromLocaleString(locale, text) * 10
                    }
                    onValueChanged: awgObject.awg1_amp_changed(value / 10.0)
                }

                Label {
                    text: qsTr("Offset [V]:")
                    Layout.column: 0
                    Layout.row: 3
                }
                SpinBox {
                    id: offsetSpin1
                    from: -50
                    to: 50
                    value: 0
                    stepSize: 1
                    Layout.column: 1
                    Layout.row: 3
                    property int decimals: 1
                    validator: DoubleValidator {
                        bottom: Math.min(offsetSpin1.from, offsetSpin1.to)
                        top: Math.max(offsetSpin1.from, offsetSpin1.to)
                    }
                    textFromValue: function(value, locale) {
                        return Number(value / 10).toLocaleString(locale, 'f', offsetSpin1.decimals)
                    }
                    valueFromText: function(text, locale) {
                        return Number.fromLocaleString(locale, text) * 10
                    }
                    onValueChanged: awgObject.awg1_offset_changed(value / 10.0)
                }
            }

            RowLayout {
                spacing: 4
                visible: awgObject.currentWaveCh1 === "Arb"
                Label {
                    text: qsTr("Arb file:")
                    verticalAlignment: Text.AlignVCenter
                }
                TextField {
                    id: filePath1
                    placeholderText: qsTr("path/to/waveform.csv")
                    Layout.fillWidth: true
                }
                Button {
                    text: qsTr("Load")
                    onClicked: awgObject.awg1_load_arb(filePath1.text)
                }
            }
        }

        ColumnLayout {
            spacing: 8
            Switch {
                text: qsTr("Enable")
                onToggled: awgObject.awg2_enable_changed(checked)
            }
            GridLayout {
                columns: 2
                columnSpacing: 4
                rowSpacing: 4

                Label {
                    text: qsTr("Waveform:")
                    Layout.column: 0
                    Layout.row: 0
                }
                ComboBox {
                    id: waveSel2
                    model: ["Sine", "Square", "Pulse", "Ramp", "Noise", "Arb"]
                    currentIndex: 0
                    Layout.column: 1
                    Layout.row: 0
                    onActivated: awgObject.awg2_waveform_selected(model[index])
                }

                Label {
                    text: qsTr("Frequency [Hz]:")
                    Layout.column: 0
                    Layout.row: 1
                }
                SpinBox {
                    id: freqSpin2
                    value: 1000
                    from: 1
                    to: 25000000
                    stepSize: 100
                    enabled: awgObject.currentWaveCh2 !== "Noise"
                    Layout.column: 1
                    Layout.row: 1
                    onValueChanged: awgObject.awg2_freq_changed(value)
                }

                Label {
                    text: qsTr("Amplitude [V]:")
                    Layout.column: 0
                    Layout.row: 2
                }
                SpinBox {
                    id: ampSpin2
                    from: 0
                    to: 100
                    value: 10
                    stepSize: 1
                    Layout.column: 1
                    Layout.row: 2
                    property int decimals: 1
                    validator: DoubleValidator {
                        bottom: Math.min(ampSpin2.from, ampSpin2.to)
                        top: Math.max(ampSpin2.from, ampSpin2.to)
                    }
                    textFromValue: function(value, locale) {
                        return Number(value / 10).toLocaleString(locale, 'f', ampSpin2.decimals)
                    }
                    valueFromText: function(text, locale) {
                        return Number.fromLocaleString(locale, text) * 10
                    }
                    onValueChanged: awgObject.awg2_amp_changed(value / 10.0)
                }

                Label {
                    text: qsTr("Offset [V]:")
                    Layout.column: 0
                    Layout.row: 3
                }
                SpinBox {
                    id: offsetSpin2
                    from: -50
                    to: 50
                    value: 0
                    stepSize: 1
                    Layout.column: 1
                    Layout.row: 3
                    property int decimals: 1
                    validator: DoubleValidator {
                        bottom: Math.min(offsetSpin2.from, offsetSpin2.to)
                        top: Math.max(offsetSpin2.from, offsetSpin2.to)
                    }
                    textFromValue: function(value, locale) {
                        return Number(value / 10).toLocaleString(locale, 'f', offsetSpin2.decimals)
                    }
                    valueFromText: function(text, locale) {
                        return Number.fromLocaleString(locale, text) * 10
                    }
                    onValueChanged: awgObject.awg2_offset_changed(value / 10.0)
                }
            }

            RowLayout {
                spacing: 4
                visible: awgObject.currentWaveCh2 === "Arb"
                Label {
                    text: qsTr("Arb file:")
                    verticalAlignment: Text.AlignVCenter
                }
                TextField {
                    id: filePath2
                    placeholderText: qsTr("path/to/waveform.csv")
                    Layout.fillWidth: true
                }
                Button {
                    text: qsTr("Load")
                    onClicked: awgObject.awg2_load_arb(filePath2.text)
                }
            }
        }
    }
}
