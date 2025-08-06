// qml/oscilloscope.qml
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.3
import com.rigol.scope 1.0

ApplicationWindow {
    id: mainWindow
    width: 1000
    height: 700
    visible: true
    title: "Oscilloscope"

    // Determine default address from program arguments or fallback
    property var args: Qt.application.arguments
    property string defaultAddr: (args.length > 1 ? args[1] : "169.254.50.23:5555")

    OscController {
        id: oscController
        address: defaultAddr
        // Start background update thread when UI is completed
        Component.onCompleted: oscController.startUpdate()
    }

    // Background color
    Rectangle {
        anchors.fill: parent
        color: "#1e1e1e"
        z: -1
    }

    // Top toolbar
    RowLayout {
        id: toolbarRow
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        spacing: 8
        Button { text: "ℹ"; onClicked: oscController.info(); }
        Button { text: "⚙"; onClicked: oscController.settings(); }
        Button { text: "Auto"; onClicked: oscController.autoscale(); }
        Button { text: ">_"; onClicked: oscController.console(); }
        Button { text: "💾"; onClicked: oscController.save_config(); }
        Button { text: "↑"; onClicked: oscController.load_config(); }
        Button { text: "📄"; onClicked: {} }
    }

    // Middle area: scope display and side controls
    RowLayout {
        id: middleRow
        anchors.top: toolbarRow.bottom
        anchors.bottom: bottomSection.top
        anchors.left: parent.left
        anchors.right: parent.right
        Layout.fillWidth: true
        Layout.fillHeight: true

        // Scope display area
        Item {
            id: scopeArea
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.preferredWidth: 3
            // Border rectangle
            Rectangle {
                anchors.fill: parent
                color: "transparent"
                border.color: "#888"
                border.width: 1
            }
            // Scope image
            Image {
                anchors.fill: parent
                source: oscController.imageSource
                fillMode: Image.PreserveAspectFit
            }
            // Center vertical reference line
            Rectangle {
                width: 2
                anchors.top: parent.top
                anchors.bottom: parent.bottom
                x: parent.width / 2 - width/2
                color: "#ccc"
            }
            // Magnifier and horizontal adjust buttons at top-right
            Button {
                id: magnifier
                text: "🔍"
                width: 20; height: 20
                anchors.top: parent.top
                anchors.right: parent.right
                anchors.topMargin: 4
                anchors.rightMargin: 4
            }
            Button {
                text: "↔"
                width: 20; height: 20
                anchors.top: parent.top
                anchors.right: magnifier.left
                anchors.topMargin: 4
                anchors.rightMargin: 4
            }
        }

        // Side control panels (Horizontal and Trigger)
        ColumnLayout {
            id: sideControls
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.preferredWidth: 1
            spacing: 8

            GroupBox {
                title: "HORIZ"
                Layout.fillWidth: true
                ColumnLayout {
                    spacing: 4
                    RowLayout {
                        spacing: 4
                        Text { text: "F"; }
                        Slider {
                            id: timebaseSlider
                            from: 1
                            to: 100
                            value: 50
                            onMoved: oscController.timebase(value)
                        }
                        Text { text: "Δ"; }
                    }
                    RowLayout {
                        spacing: 4
                        Text { text: "B"; }
                        Slider {
                            id: offsetSlider
                            from: -100
                            to: 100
                            value: 0
                            onMoved: oscController.time_offset(value)
                        }
                        Text { text: "D"; }
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
                            id: trigSlider
                            orientation: Qt.Vertical
                            from: -100
                            to: 100
                            value: 0
                            onMoved: {
                                trigSpin.value = Math.round(value);
                                oscController.trigger_level(Math.round(value));
                            }
                            Layout.preferredHeight: 100
                        }
                        SpinBox {
                            id: trigSpin
                            from: -100
                            to: 100
                            value: 0
                            onValueChanged: {
                                if (!trigSlider.pressed) {
                                    trigSlider.value = value;
                                    oscController.trigger_level(value);
                                }
                            }
                        }
                    }
                    // Trigger slope buttons
                    RowLayout {
                        spacing: 4
                        Button { text: "↑"; onClicked: oscController.trigger_slope_up(); }
                        Button { text: "↓"; onClicked: oscController.trigger_slope_down(); }
                    }
                    // Trigger source selection
                    ComboBox {
                        id: trigSource
                        model: ["CH1", "CH2", "CH3", "CH4", "EXT"]
                        currentIndex: 0
                        onActivated: oscController.trigger_source(currentText)
                    }
                    // Single and Run/Stop buttons
                    RowLayout {
                        spacing: 4
                        Button { text: "SINGLE"; onClicked: oscController.single(); }
                        Button { text: "RUN/STOP"; onClicked: oscController.run_stop(); }
                    }
                }
            }
        }
    }

    // Bottom section: channel tabs and other tabs
    Item {
        id: bottomSection
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom

        // Channel tabs (CH1-CH4)
        TabView {
            id: channelTabs
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            width: parent.width * 0.67

            Tab {
                title: "CH1"
                ColumnLayout {
                    spacing: 4; anchors.fill: parent; anchors.margins: 4
                    Switch {
                        text: "Enable"
                        onToggled: oscController.ch1_enable(checked)
                    }
                    RowLayout {
                        spacing: 4
                        Text { text: "F"; }
                        Slider {
                            from: 1; to: 100; value: 50
                            onMoved: oscController.ch1_scale(value)
                        }
                        Text { text: "A"; }
                    }
                    RowLayout {
                        spacing: 4
                        Text { text: "B"; }
                        Slider {
                            from: -100; to: 100; value: 0
                            onMoved: oscController.ch1_offset(value)
                        }
                        Text { text: "D"; }
                    }
                    GridLayout {
                        columns: 2; columnSpacing: 4; rowSpacing: 4
                        Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0; }
                        ComboBox {
                            model: ["DC", "AC", "GND"]; currentIndex: 0
                            Layout.row: 0; Layout.column: 1
                            onActivated: oscController.ch1_coupling(currentText)
                        }
                        Label { text: "Probe:"; Layout.row: 1; Layout.column: 0; }
                        ComboBox {
                            model: ["1×", "10×"]; currentIndex: 0
                            Layout.row: 1; Layout.column: 1
                            onActivated: oscController.ch1_probe(currentText)
                        }
                        Label { text: "Current:"; Layout.row: 2; Layout.column: 0; }
                        TextField {
                            text: "0.00"; readOnly: true
                            Layout.row: 2; Layout.column: 1
                        }
                    }
                    CheckBox {
                        text: "AVG"
                        checked: oscController.avgEnabled
                        onClicked: oscController.average(checked)
                    }
                }
            }
            Tab {
                title: "CH2"
                ColumnLayout {
                    spacing: 4; anchors.fill: parent; anchors.margins: 4
                    Switch {
                        text: "Enable"
                        onToggled: oscController.ch2_enable(checked)
                    }
                    RowLayout {
                        spacing: 4
                        Text { text: "F"; }
                        Slider {
                            from: 1; to: 100; value: 50
                            onMoved: oscController.ch2_scale(value)
                        }
                        Text { text: "A"; }
                    }
                    RowLayout {
                        spacing: 4
                        Text { text: "B"; }
                        Slider {
                            from: -100; to: 100; value: 0
                            onMoved: oscController.ch2_offset(value)
                        }
                        Text { text: "D"; }
                    }
                    GridLayout {
                        columns: 2; columnSpacing: 4; rowSpacing: 4
                        Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0; }
                        ComboBox {
                            model: ["DC", "AC", "GND"]; currentIndex: 0
                            Layout.row: 0; Layout.column: 1
                            onActivated: oscController.ch2_coupling(currentText)
                        }
                        Label { text: "Probe:"; Layout.row: 1; Layout.column: 0; }
                        ComboBox {
                            model: ["1×", "10×"]; currentIndex: 0
                            Layout.row: 1; Layout.column: 1
                            onActivated: oscController.ch2_probe(currentText)
                        }
                        Label { text: "Current:"; Layout.row: 2; Layout.column: 0; }
                        TextField {
                            text: "0.00"; readOnly: true
                            Layout.row: 2; Layout.column: 1
                        }
                    }
                    CheckBox {
                        text: "AVG"
                        checked: oscController.avgEnabled
                        onClicked: oscController.average(checked)
                    }
                }
            }
            Tab {
                title: "CH3"
                ColumnLayout {
                    spacing: 4; anchors.fill: parent; anchors.margins: 4
                    Switch {
                        text: "Enable"
                        onToggled: oscController.ch3_enable(checked)
                    }
                    RowLayout {
                        spacing: 4
                        Text { text: "F"; }
                        Slider {
                            from: 1; to: 100; value: 50
                            onMoved: oscController.ch3_scale(value)
                        }
                        Text { text: "A"; }
                    }
                    RowLayout {
                        spacing: 4
                        Text { text: "B"; }
                        Slider {
                            from: -100; to: 100; value: 0
                            onMoved: oscController.ch3_offset(value)
                        }
                        Text { text: "D"; }
                    }
                    GridLayout {
                        columns: 2; columnSpacing: 4; rowSpacing: 4
                        Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0; }
                        ComboBox {
                            model: ["DC", "AC", "GND"]; currentIndex: 0
                            Layout.row: 0; Layout.column: 1
                            onActivated: oscController.ch3_coupling(currentText)
                        }
                        Label { text: "Probe:"; Layout.row: 1; Layout.column: 0; }
                        ComboBox {
                            model: ["1×", "10×"]; currentIndex: 0
                            Layout.row: 1; Layout.column: 1
                            onActivated: oscController.ch3_probe(currentText)
                        }
                        Label { text: "Current:"; Layout.row: 2; Layout.column: 0; }
                        TextField {
                            text: "0.00"; readOnly: true
                            Layout.row: 2; Layout.column: 1
                        }
                    }
                    CheckBox {
                        text: "AVG"
                        checked: oscController.avgEnabled
                        onClicked: oscController.average(checked)
                    }
                }
            }
            Tab {
                title: "CH4"
                ColumnLayout {
                    spacing: 4; anchors.fill: parent; anchors.margins: 4
                    Switch {
                        text: "Enable"
                        onToggled: oscController.ch4_enable(checked)
                    }
                    RowLayout {
                        spacing: 4
                        Text { text: "F"; }
                        Slider {
                            from: 1; to: 100; value: 50
                            onMoved: oscController.ch4_scale(value)
                        }
                        Text { text: "A"; }
                    }
                    RowLayout {
                        spacing: 4
                        Text { text: "B"; }
                        Slider {
                            from: -100; to: 100; value: 0
                            onMoved: oscController.ch4_offset(value)
                        }
                        Text { text: "D"; }
                    }
                    GridLayout {
                        columns: 2; columnSpacing: 4; rowSpacing: 4
                        Label { text: "Coupling:"; Layout.row: 0; Layout.column: 0; }
                        ComboBox {
                            model: ["DC", "AC", "GND"]; currentIndex: 0
                            Layout.row: 0; Layout.column: 1
                            onActivated: oscController.ch4_coupling(currentText)
                        }
                        Label { text: "Probe:"; Layout.row: 1; Layout.column: 0; }
                        ComboBox {
                            model: ["1×", "10×"]; currentIndex: 0
                            Layout.row: 1; Layout.column: 1
                            onActivated: oscController.ch4_probe(currentText)
                        }
                        Label { text: "Current:"; Layout.row: 2; Layout.column: 0; }
                        TextField {
                            text: "0.00"; readOnly: true
                            Layout.row: 2; Layout.column: 1
                        }
                    }
                    CheckBox {
                        text: "AVG"
                        checked: oscController.avgEnabled
                        onClicked: oscController.average(checked)
                    }
                }
            }
        }

        // Measure/Cursor/Math tabs
        TabView {
            id: miscTabs
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            anchors.right: parent.right
            width: parent.width * 0.33

            Tab { title: "Measure"; Rectangle { anchors.fill: parent; color: "transparent"; } }
            Tab { title: "Cursor"; Rectangle { anchors.fill: parent; color: "transparent"; } }
            Tab { title: "Math";   Rectangle { anchors.fill: parent; color: "transparent"; } }
        }
    }
}
