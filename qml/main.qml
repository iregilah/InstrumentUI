// qml/main.qml
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12
import InstrumentUI 1.0
import QtQuick.Controls.Material 2.12

ApplicationWindow {
    id: mainWindow
    width: 1000
    height: 700
    minimumWidth: 900
    minimumHeight: 560
    title: qsTr("Oscilloscope")
    visible: true
    color: "#1e1e1e"
    Material.theme: oscillo.darkMode ? Material.Dark : Material.Light

    OscilloObject {
        id: oscillo
        objectName: "oscilloObject"
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 8

        /* Top toolbar buttons */
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
                    settingsPopup.open()
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

        /* Scope image and trigger/timebase controls */
        RowLayout {
            id: middleRow
            spacing: 8
            Layout.fillWidth: true
            Layout.fillHeight: true

            // Scope area grows with the window and keeps aspect ratio
            Rectangle {
                id: scopeFrame
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumWidth: 400
                border.color: "#888"
                border.width: 1
                clip: true

                Image {
                    id: scopeImage
                    anchors.fill: parent
                    source: oscillo.scopeImageUrl
                    fillMode: Image.PreserveAspectFit
                    asynchronous: true
                    smooth: true
                }

                // Center line stays centered on the actual painted image, not the letterbox
                Rectangle {
                    width: 2
                    height: scopeImage.paintedHeight
                    x: (parent.width - scopeImage.paintedWidth)/2 + scopeImage.paintedWidth/2 - 1
                    y: (parent.height - scopeImage.paintedHeight)/2
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

            // Right-side controls are anchored to the right and scroll when needed
            ScrollView {
                id: rightPanel
                Layout.fillHeight: true
                Layout.preferredWidth: 320
                Layout.minimumWidth: 260
                Layout.maximumWidth: 480
                clip: true
                ScrollBar.vertical.policy: ScrollBar.AsNeeded

                ColumnLayout {
                    id: rightCol
                    width: rightPanel.width
                    spacing: 8

                    GroupBox {
                        title: qsTr("HORIZ")
                        Layout.fillWidth: true
                        ColumnLayout {
                            spacing: 4
                            RowLayout {
                                spacing: 4
                                Label { text: qsTr("F") }
                                Slider {
                                    Layout.fillWidth: true
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
                                    Layout.fillWidth: true
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
                        Layout.fillWidth: true
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
                                    Layout.preferredHeight: 160
                                    onValueChanged: {
                                        let v = Math.round(value)
                                        if (spinLevel.value !== v) spinLevel.value = v
                                        oscillo.trigger_level_changed(v)
                                    }
                                }
                                SpinBox {
                                    id: spinLevel
                                    from: -100
                                    to: 100
                                    value: 0
                                    onValueChanged: {
                                        if (trigSlider.value !== value) trigSlider.value = value
                                        oscillo.trigger_level_changed(value)
                                    }
                                }
                            }
                            RowLayout {
                                spacing: 4
                                Button {
                                    text: "↑"
                                    onClicked: oscillo.trigger_slope_up()
                                }
                                Button {
                                    text: "↓"
                                    onClicked: oscillo.trigger_slope_down()
                                }
                            }
                            ComboBox {
                                id: trigSourceBox
                                Layout.fillWidth: true
                                model: ["CH1", "CH2", "CH3", "CH4", "EXT"]
                                onActivated: oscillo.trigger_source_selected(model[index])
                            }
                            RowLayout {
                                spacing: 4
                                Button {
                                    text: qsTr("SINGLE")
                                    onClicked: oscillo.single_trigger()
                                }
                                Button {
                                    text: qsTr("RUN/STOP")
                                    onClicked: oscillo.run_stop()
                                }
                            }
                        }
                    }
                }
            }
        }

        /* Bottom panels (Channels and Misc tabs) */
        RowLayout {
            id: bottomRow
            spacing: 8
            Layout.fillWidth: true

            // Channels pane (left) — consistent width with its tab bar; scrollable content
            ColumnLayout {
                id: channelsPane
                Layout.fillWidth: true
                Layout.preferredWidth: 520
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

                    /* Channel 1 controls */
                    ScrollView {
                        Layout.fillWidth: true
                        clip: true
                        ScrollBar.vertical.policy: ScrollBar.AsNeeded
                        ColumnLayout {
                            width: parent.width
                            spacing: 4
                            Switch {
                                text: qsTr("Enable")
                                onToggled: oscillo.ch1_enable_changed(checked)
                            }
                            RowLayout {
                                spacing: 4
                                Label { text: qsTr("F") }
                                Slider {
                                    Layout.fillWidth: true
                                    from: 1
                                    to: 100
                                    value: 50
                                    onValueChanged: oscillo.ch1_scale_changed(value)
                                }
                                Label { text: qsTr("A") }
                            }
                            RowLayout {
                                spacing: 4
                                Label { text: qsTr("B") }
                                Slider {
                                    Layout.fillWidth: true
                                    from: -100
                                    to: 100
                                    value: 0
                                    onValueChanged: oscillo.ch1_offset_changed(value)
                                }
                                Label { text: qsTr("D") }
                            }
                            GridLayout {
                                columns: 2
                                columnSpacing: 4
                                rowSpacing: 4
                                Layout.fillWidth: true

                                Label { text: qsTr("Coupling:"); Layout.column: 0; Layout.row: 0 }
                                ComboBox {
                                    Layout.column: 1; Layout.row: 0
                                    Layout.fillWidth: true
                                    model: ["DC", "AC", "GND"]
                                    onActivated: oscillo.ch1_coupling_selected(model[index])
                                }
                                Label { text: qsTr("Probe:"); Layout.column: 0; Layout.row: 1 }
                                ComboBox {
                                    Layout.column: 1; Layout.row: 1
                                    Layout.fillWidth: true
                                    model: ["1×", "10×"]
                                    onActivated: oscillo.ch1_probe_selected(model[index])
                                }
                                Label { text: qsTr("Current:"); Layout.column: 0; Layout.row: 2 }
                                TextField {
                                    Layout.column: 1; Layout.row: 2
                                    Layout.fillWidth: true
                                    text: "0.00"
                                    readOnly: true
                                }
                            }
                            CheckBox {
                                text: qsTr("AVG")
                                checked: oscillo.avgEnabled
                                onToggled: oscillo.average_toggled(checked)
                            }
                        }
                    }

                    /* Channel 2 controls */
                    ScrollView {
                        Layout.fillWidth: true
                        clip: true
                        ScrollBar.vertical.policy: ScrollBar.AsNeeded
                        ColumnLayout {
                            width: parent.width
                            spacing: 4
                            Switch {
                                text: qsTr("Enable")
                                onToggled: oscillo.ch2_enable_changed(checked)
                            }
                            RowLayout {
                                spacing: 4
                                Label { text: qsTr("F") }
                                Slider {
                                    Layout.fillWidth: true
                                    from: 1
                                    to: 100
                                    value: 50
                                    onValueChanged: oscillo.ch2_scale_changed(value)
                                }
                                Label { text: qsTr("A") }
                            }
                            RowLayout {
                                spacing: 4
                                Label { text: qsTr("B") }
                                Slider {
                                    Layout.fillWidth: true
                                    from: -100
                                    to: 100
                                    value: 0
                                    onValueChanged: oscillo.ch2_offset_changed(value)
                                }
                                Label { text: qsTr("D") }
                            }
                            GridLayout {
                                columns: 2
                                columnSpacing: 4
                                rowSpacing: 4
                                Layout.fillWidth: true

                                Label { text: qsTr("Coupling:"); Layout.column: 0; Layout.row: 0 }
                                ComboBox {
                                    Layout.column: 1; Layout.row: 0
                                    Layout.fillWidth: true
                                    model: ["DC", "AC", "GND"]
                                    onActivated: oscillo.ch2_coupling_selected(model[index])
                                }
                                Label { text: qsTr("Probe:"); Layout.column: 0; Layout.row: 1 }
                                ComboBox {
                                    Layout.column: 1; Layout.row: 1
                                    Layout.fillWidth: true
                                    model: ["1×", "10×"]
                                    onActivated: oscillo.ch2_probe_selected(model[index])
                                }
                                Label { text: qsTr("Current:"); Layout.column: 0; Layout.row: 2 }
                                TextField {
                                    Layout.column: 1; Layout.row: 2
                                    Layout.fillWidth: true
                                    text: "0.00"
                                    readOnly: true
                                }
                            }
                            CheckBox {
                                text: qsTr("AVG")
                                checked: oscillo.avgEnabled
                                onToggled: oscillo.average_toggled(checked)
                            }
                        }
                    }

                    /* Channel 3 controls */
                    ScrollView {
                        Layout.fillWidth: true
                        clip: true
                        ScrollBar.vertical.policy: ScrollBar.AsNeeded
                        ColumnLayout {
                            width: parent.width
                            spacing: 4
                            Switch {
                                text: qsTr("Enable")
                                onToggled: oscillo.ch3_enable_changed(checked)
                            }
                            RowLayout {
                                spacing: 4
                                Label { text: qsTr("F") }
                                Slider {
                                    Layout.fillWidth: true
                                    from: 1
                                    to: 100
                                    value: 50
                                    onValueChanged: oscillo.ch3_scale_changed(value)
                                }
                                Label { text: qsTr("A") }
                            }
                            RowLayout {
                                spacing: 4
                                Label { text: qsTr("B") }
                                Slider {
                                    Layout.fillWidth: true
                                    from: -100
                                    to: 100
                                    value: 0
                                    onValueChanged: oscillo.ch3_offset_changed(value)
                                }
                                Label { text: qsTr("D") }
                            }
                            GridLayout {
                                columns: 2
                                columnSpacing: 4
                                rowSpacing: 4
                                Layout.fillWidth: true

                                Label { text: qsTr("Coupling:"); Layout.column: 0; Layout.row: 0 }
                                ComboBox {
                                    Layout.column: 1; Layout.row: 0
                                    Layout.fillWidth: true
                                    model: ["DC", "AC", "GND"]
                                    onActivated: oscillo.ch3_coupling_selected(model[index])
                                }
                                Label { text: qsTr("Probe:"); Layout.column: 0; Layout.row: 1 }
                                ComboBox {
                                    Layout.column: 1; Layout.row: 1
                                    Layout.fillWidth: true
                                    model: ["1×", "10×"]
                                    onActivated: oscillo.ch3_probe_selected(model[index])
                                }
                                Label { text: qsTr("Current:"); Layout.column: 0; Layout.row: 2 }
                                TextField {
                                    Layout.column: 1; Layout.row: 2
                                    Layout.fillWidth: true
                                    text: "0.00"
                                    readOnly: true
                                }
                            }
                            CheckBox {
                                text: qsTr("AVG")
                                checked: oscillo.avgEnabled
                                onToggled: oscillo.average_toggled(checked)
                            }
                        }
                    }

                    /* Channel 4 controls */
                    ScrollView {
                        Layout.fillWidth: true
                        clip: true
                        ScrollBar.vertical.policy: ScrollBar.AsNeeded
                        ColumnLayout {
                            width: parent.width
                            spacing: 4
                            Switch {
                                text: qsTr("Enable")
                                onToggled: oscillo.ch4_enable_changed(checked)
                            }
                            RowLayout {
                                spacing: 4
                                Label { text: qsTr("F") }
                                Slider {
                                    Layout.fillWidth: true
                                    from: 1
                                    to: 100
                                    value: 50
                                    onValueChanged: oscillo.ch4_scale_changed(value)
                                }
                                Label { text: qsTr("A") }
                            }
                            RowLayout {
                                spacing: 4
                                Label { text: qsTr("B") }
                                Slider {
                                    Layout.fillWidth: true
                                    from: -100
                                    to: 100
                                    value: 0
                                    onValueChanged: oscillo.ch4_offset_changed(value)
                                }
                                Label { text: qsTr("D") }
                            }
                            GridLayout {
                                columns: 2
                                columnSpacing: 4
                                rowSpacing: 4
                                Layout.fillWidth: true

                                Label { text: qsTr("Coupling:"); Layout.column: 0; Layout.row: 0 }
                                ComboBox {
                                    Layout.column: 1; Layout.row: 0
                                    Layout.fillWidth: true
                                    model: ["DC", "AC", "GND"]
                                    onActivated: oscillo.ch4_coupling_selected(model[index])
                                }
                                Label { text: qsTr("Probe:"); Layout.column: 0; Layout.row: 1 }
                                ComboBox {
                                    Layout.column: 1; Layout.row: 1
                                    Layout.fillWidth: true
                                    model: ["1×", "10×"]
                                    onActivated: oscillo.ch4_probe_selected(model[index])
                                }
                                Label { text: qsTr("Current:"); Layout.column: 0; Layout.row: 2 }
                                TextField {
                                    Layout.column: 1; Layout.row: 2
                                    Layout.fillWidth: true
                                    text: "0.00"
                                    readOnly: true
                                }
                            }
                            CheckBox {
                                text: qsTr("AVG")
                                checked: oscillo.avgEnabled
                                onToggled: oscillo.average_toggled(checked)
                            }
                        }
                    }
                }
            }

            // Misc pane (right) — stays aligned to the right side
            ColumnLayout {
                id: miscPane
                Layout.fillWidth: true
                Layout.preferredWidth: 280
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

                    ColumnLayout { Layout.fillWidth: true }   /* Measure */
                    ColumnLayout { Layout.fillWidth: true }   /* Cursor */
                    ColumnLayout { Layout.fillWidth: true }   /* Math */
                }
            }
        }
    }

    Popup {
        id: settingsPopup
        modal: true
        anchors.centerIn: parent
        ColumnLayout {
            spacing: 8

            Switch {
                text: qsTr("Dark theme")
                checked: oscillo.darkMode
                onToggled: oscillo.darkMode = checked
            }
            RowLayout {
                spacing: 4
                Label { text: qsTr("Style:") }
                ComboBox {
                    id: styleCombo
                    model: ["Fusion", "Imagine", "Material"]
                    Component.onCompleted: {
                        var idx = model.indexOf(oscillo.currentStyle);
                        if (idx >= 0) currentIndex = idx;
                    }
                    onActivated: oscillo.set_style(styleCombo.currentText)
                }
            }
            Label {
                text: qsTr("* Style change takes effect on next launch")
                font.pointSize: 8
                color: "#888"
            }
            Button {
                text: qsTr("Close")
                Layout.alignment: Qt.AlignHCenter
                onClicked: settingsPopup.close()
            }
        }
    }
    Component.onCompleted: {
        oscillo.init_from_env()
        oscillo.start_capture()
    }
}
