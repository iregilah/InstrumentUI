// qml/awg.qml

import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12
import InstrumentUI 1.0

Window {
    id: awgWindow
    width: 500
    height: 300
    title: qsTr("Function Generator")
    visible: true

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
            Switch { text: qsTr("Enable"); onToggled: awgObject.awg1_enable_changed(checked) }
            GridLayout {
                columns: 2
                columnSpacing: 4
                rowSpacing: 4

                Label { text: qsTr("Waveform:"); Layout.column: 0; Layout.row: 0 }
                ComboBox {
                    id: waveSel1
                    model: ["Sine", "Square", "Pulse", "Ramp", "Noise", "Arb"]
                    currentIndex: 0
                    Layout.column: 1; Layout.row: 0
                    onActivated: awgObject.awg1_waveform_selected(model[index])
                }

                Label { text: qsTr("Frequency [Hz]:"); Layout.column: 0; Layout.row: 1 }
                SpinBox {
                    id: freqSpin1
                    value: 1000
                    from: 1
                    to: 25000000
                    stepSize: 100
                    enabled: awgObject.currentWaveCh1 !== "Noise"
                    Layout.column: 1; Layout.row: 1
                    onValueChanged: awgObject.awg1_freq_changed(value)
                }

                Label { text: qsTr("Amplitude [V]:"); Layout.column: 0; Layout.row: 2 }
                SpinBox {
                    value: 1.0
                    from: 0.0
                    to: 10.0
                    stepSize: 0.1
                    Layout.column: 1; Layout.row: 2
                    onValueChanged: awgObject.awg1_amp_changed(value)
                }

                Label { text: qsTr("Offset [V]:"); Layout.column: 0; Layout.row: 3 }
                SpinBox {
                    value: 0.0
                    from: -5.0
                    to: 5.0
                    stepSize: 0.1
                    Layout.column: 1; Layout.row: 3
                    onValueChanged: awgObject.awg1_offset_changed(value)
                }
            }
            RowLayout {
                spacing: 4
                visible: awgObject.currentWaveCh1 === "Arb"
                Label { text: qsTr("Arb file:"); verticalAlignment: Text.AlignVCenter }
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
            Switch { text: qsTr("Enable"); onToggled: awgObject.awg2_enable_changed(checked) }
            GridLayout {
                columns: 2
                columnSpacing: 4
                rowSpacing: 4

                Label { text: qsTr("Waveform:"); Layout.column: 0; Layout.row: 0 }
                ComboBox {
                    id: waveSel2
                    model: ["Sine", "Square", "Pulse", "Ramp", "Noise", "Arb"]
                    currentIndex: 0
                    Layout.column: 1; Layout.row: 0
                    onActivated: awgObject.awg2_waveform_selected(model[index])
                }

                Label { text: qsTr("Frequency [Hz]:"); Layout.column: 0; Layout.row: 1 }
                SpinBox {
                    id: freqSpin2
                    value: 1000
                    from: 1
                    to: 25000000
                    stepSize: 100
                    enabled: awgObject.currentWaveCh2 !== "Noise"
                    Layout.column: 1; Layout.row: 1
                    onValueChanged: awgObject.awg2_freq_changed(value)
                }

                Label { text: qsTr("Amplitude [V]:"); Layout.column: 0; Layout.row: 2 }
                SpinBox {
                    value: 1.0
                    from: 0.0
                    to: 10.0
                    stepSize: 0.1
                    Layout.column: 1; Layout.row: 2
                    onValueChanged: awgObject.awg2_amp_changed(value)
                }

                Label { text: qsTr("Offset [V]:"); Layout.column: 0; Layout.row: 3 }
                SpinBox {
                    value: 0.0
                    from: -5.0
                    to: 5.0
                    stepSize: 0.1
                    Layout.column: 1; Layout.row: 3
                    onValueChanged: awgObject.awg2_offset_changed(value)
                }
            }
            RowLayout {
                spacing: 4
                visible: awgObject.currentWaveCh2 === "Arb"
                Label { text: qsTr("Arb file:"); verticalAlignment: Text.AlignVCenter }
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

    Component.onCompleted: {
        console.log("AWG window component completed")
    }
}
