// qml/hub.qml
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12
import InstrumentUI 1.0
import QtQuick.Controls.Material 2.12

ApplicationWindow {
    id: instrumentWindow
    width: 1000
    height: 600
    title: qsTr("Instrument Manager")
    visible: true
    Material.theme: Material.Dark
    color: "#1e1e1e"    // match main window background

    InstrumentManager {
        id: instrumentManager
        objectName: "instrumentManager"
    }

    function refreshInstrumentList() {
        var raw = instrumentManager.instrumentList
        var arr = []
        try {
            arr = JSON.parse(raw || "[]")
        } catch (e) {
            console.warn("instrumentList JSON parse error:", e, "raw=", raw)
            arr = []
        }
        instrumentListModel.clear()
        for (var i = 0; i < arr.length; ++i) instrumentListModel.append(arr[i])
    }

    // Sidebar panel
    Rectangle {
        id: sidebar
        anchors.top: parent.top
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        width: 280
        color: "#2b2b2b"
        property real collapsedWidth: 32
        property real expandedWidth: width

        // Tab bar for Instruments/Automations
        TabBar {
            id: sidebarTabBar
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            TabButton {
                text: qsTr("Instruments")
            }
            TabButton {
                text: qsTr("Automations")
            }
        }

        // Instruments tab content
        Item {
            id: instrumentsTab
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: sidebarTabBar.bottom
            anchors.bottom: collapseButton.top
            visible: sidebarTabBar.currentIndex === 0

            // Instruments toolbar
            RowLayout {
                id: instrumentToolbar
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.top: parent.top
                spacing: 8
                padding: 4
                Button {
                    text: qsTr("Connect")
                    onClicked: {
                        console.log("Connect clicked")
                        // TODO: implement connection dialog
                    }
                }
                Button {
                    text: qsTr("Refresh")
                    onClicked: instrumentManager.scan()
                }
                Button {
                    text: qsTr("⚙")
                    onClicked: console.log("Instrument settings clicked")
                }
            }

            // Instrument list
            ListView {
                id: instrumentListView
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.top: instrumentToolbar.bottom
                anchors.bottom: parent.bottom
                model: instrumentListModel
                clip: true
                ScrollBar.vertical.policy: ScrollBar.AsNeeded

                delegate: Item {
                    width: instrumentListView.width
                    height: 32
                    RowLayout {
                        anchors.fill: parent
                        anchors.margins: 4
                        spacing: 8
                        Label {
                            text: modelData.name
                            font.pixelSize: 14
                            Layout.alignment: Qt.AlignVCenter
                            color: "#ffffff"
                        }
                        Rectangle {
                            // Communication channel badge
                            color: "#444"
                            radius: 8
                            border.color: "#888"
                            height: 20
                            Layout.alignment: Qt.AlignVCenter
                            Layout.margins: 2
                            implicitWidth: badgeText.implicitWidth + 16
                            width: implicitWidth
                            Text {
                                id: badgeText
                                text: modelData.comm
                                color: "#ffffff"
                                font.pixelSize: 12
                                anchors.centerIn: parent
                            }
                        }
                        Item {
                            Layout.fillWidth: true
                        }
                        Button {
                            text: "⋮"
                            flat: true
                            width: 30
                            Layout.alignment: Qt.AlignVCenter
                            Menu {
                                id: instMenu
                                MenuItem {
                                    text: qsTr("Send to New Window"); onTriggered: console.log("Send to New Window")
                                }
                                MenuItem {
                                    text: qsTr("Configure"); onTriggered: console.log("Configure instrument")
                                }
                                MenuItem {
                                    text: qsTr("Disconnect"); onTriggered: console.log("Disconnect instrument")
                                }
                            }
                            onClicked: instMenu.open()
                        }
                    }
                }
            }
        }

        // Automations tab content
        Item {
            id: automationsTab
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: sidebarTabBar.bottom
            anchors.bottom: collapseButton.top
            visible: sidebarTabBar.currentIndex === 1

            // Automations toolbar
            RowLayout {
                id: automationToolbar
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.top: parent.top
                spacing: 8
                padding: 4
                Button {
                    text: qsTr("New")
                    onClicked: console.log("New automation clicked")
                }
                Button {
                    text: qsTr("Open")
                    menu: Menu {
                        MenuItem {
                            text: qsTr("Open Recent"); enabled: false
                        }
                        // TODO: populate recent items
                    }
                }
                Button {
                    text: qsTr("Import")
                    onClicked: console.log("Import automation clicked")
                }
                Button {
                    text: qsTr("⚙")
                    onClicked: console.log("Automation settings clicked")
                }
            }

            // Automation list
            ListView {
                id: automationListView
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.top: automationToolbar.bottom
                anchors.bottom: parent.bottom
                model: automationListModel
                clip: true
                ScrollBar.vertical.policy: ScrollBar.AsNeeded

                delegate: Item {
                    width: automationListView.width
                    height: 32
                    RowLayout {
                        anchors.fill: parent
                        anchors.margins: 4
                        spacing: 8
                        Label {
                            text: modelData.name
                            font.pixelSize: 14
                            Layout.alignment: Qt.AlignVCenter
                            color: "#ffffff"
                        }
                        Rectangle {
                            color: "#444"
                            radius: 8
                            border.color: "#888"
                            height: 20
                            Layout.alignment: Qt.AlignVCenter
                            Layout.margins: 2
                            implicitWidth: stateText.implicitWidth + 16
                            width: implicitWidth
                            Text {
                                id: stateText
                                text: modelData.state
                                color: "#ffffff"
                                font.pixelSize: 12
                                anchors.centerIn: parent
                            }
                        }
                        Item {
                            Layout.fillWidth: true
                        }
                        Button {
                            text: "⋮"
                            flat: true
                            width: 30
                            Layout.alignment: Qt.AlignVCenter
                            Menu {
                                id: autoMenu
                                MenuItem {
                                    text: qsTr("Edit"); onTriggered: console.log("Edit automation")
                                }
                                MenuItem {
                                    text: qsTr("Rename"); onTriggered: console.log("Rename automation")
                                }
                                MenuItem {
                                    text: qsTr("Send to New Window"); onTriggered: console.log("Open automation in new window")
                                }
                                MenuItem {
                                    text: qsTr("Run"); onTriggered: console.log("Run automation")
                                }
                                MenuItem {
                                    text: qsTr("Export"); onTriggered: console.log("Export automation")
                                }
                                MenuItem {
                                    text: qsTr("Delete"); onTriggered: console.log("Delete automation")
                                }
                            }
                            onClicked: autoMenu.open()
                        }
                    }
                }
            }
        }

        // Collapse/expand sidebar button
        Button {
            id: collapseButton
            anchors.bottom: parent.bottom
            anchors.left: parent.left
            anchors.right: parent.right
            text: "◀"
            onClicked: {
                if (sidebar.width > sidebar.collapsedWidth + 1) {
                    sidebar.expandedWidth = sidebar.width
                    sidebar.width = sidebar.collapsedWidth
                    collapseButton.text = "▶"
                } else {
                    sidebar.width = sidebar.expandedWidth
                    collapseButton.text = "◀"
                }
            }
        }

        // Draggable resize handle
        Rectangle {
            id: resizeHandle
            anchors.top: parent.top
            anchors.bottom: collapseButton.top
            anchors.right: parent.right
            width: 5
            color: "#00000000"
            property real startX: 0
            property real startWidth: 0
            MouseArea {
                anchors.fill: parent
                cursorShape: Qt.SizeHorCursor
                hoverEnabled: true
                onPressed: {
                    resizeHandle.startX = mouse.x
                    resizeHandle.startWidth = sidebar.width
                }
                onPositionChanged: {
                    var dx = mouse.x - resizeHandle.startX
                    var newWidth = resizeHandle.startWidth + dx
                    if (newWidth < sidebar.collapsedWidth) {
                        newWidth = sidebar.collapsedWidth
                    }
                    sidebar.width = newWidth
                }
            }
        }
    }

    // Content area (right panel)
    Rectangle {
        id: contentArea
        anchors.top: parent.top
        anchors.bottom: parent.bottom
        anchors.left: sidebar.right
        anchors.right: parent.right
        color: "#121212"
        Text {
            text: qsTr("Select an instrument or automation to view details.")
            color: "#888"
            anchors.centerIn: parent
        }
    }

    // List models
    ListModel {
        id: instrumentListModel
    }
    ListModel {
        id: automationListModel
    }

    Connections {
        target: instrumentManager
        onInstrumentListChanged: refreshInstrumentList()
    }

    Component.onCompleted: {
        instrumentManager.scan()
        refreshInstrumentList()
        // Populate dummy automations for demonstration
        automationListModel.clear()
        automationListModel.append({"name": "Test Script 1", "state": "idle"})
        automationListModel.append({"name": "Calibration", "state": "running"})
    }
}
