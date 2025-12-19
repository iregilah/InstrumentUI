// GraphViewWindow.qml
import QtQuick 6.5
import QtQuick.Controls 6.5
import QtQuick.Layouts 1.15
import QtQuick.Dialogs 6.5
import InstrumentUI 1.0

ApplicationWindow {
    id: window
    visible: true
    width: 1000
    height: 600
    title: qsTr("Graph View Test")
    // Live scope view settings
    property int scopeChannel: 1
    property int refreshMs: 200
    property bool live: true

    ColumnLayout {
        anchors.fill: parent
        spacing: 8

        RowLayout {
            id: controlRow
            Layout.fillWidth: true
            Label {
                text: qsTr("Mode:")
            }
            ComboBox {
                id: modeCombo
                model: [qsTr("Compress"), qsTr("Scroll"), qsTr("Triggered")]
                currentIndex: graph.mode
                onCurrentIndexChanged: {
                    graph.mode = currentIndex;
                    graph.resetZoom();
                }
            }
            CheckBox {
                id: legendChk
                text: qsTr("Legend")
                checked: graph.legendVisible
                onToggled: {
                    graph.legendVisible = checked;
                    graph.requestRepaint();
                }
            }
            CheckBox {
                id: separateChk
                text: qsTr("Separate Series")
                checked: graph.separateSeries
                onToggled: {
                    graph.separateSeries = checked;
                    graph.resetZoom();
                }
            }
            Button {
                text: qsTr("Reset Zoom")
                onClicked: graph.resetZoom()
            }
            CheckBox {
                id: gridChk
                text: qsTr("Grid")
                checked: graph.gridVisible
                onToggled: {
                    graph.gridVisible = checked;
                    graph.requestRepaint();
                }
            }
            CheckBox {
                id: darkChk
                text: qsTr("Dark Mode")
                checked: graph.darkMode
                onToggled: {
                    graph.darkMode = checked;
                    graph.requestRepaint();
                }
            }
            CheckBox {
                id: xLogChk
                text: qsTr("Log X")
                checked: graph.xLogScale
                onToggled: {
                    graph.xLogScale = checked;
                    graph.requestRepaint();
                }
            }
            CheckBox {
                id: yLogChk
                text: qsTr("Log Y")
                checked: graph.yLogScale
                onToggled: {
                    graph.yLogScale = checked;
                    graph.requestRepaint();
                }
            }
            Item {
                Layout.fillWidth: true
            } // spacer

            Label {
                text: qsTr("Channel:")
            }
            ComboBox {
                id: channelCombo
                model: [1, 2, 3, 4]
                currentIndex: window.scopeChannel - 1
                onCurrentIndexChanged: {
                    window.scopeChannel = currentIndex + 1
                    // channel váltáskor menjen vissza autoscale-ra
                    graph.xAutoRange = true
                    graph.yAutoRange = true
                    graph.resetZoom()
                    if (window.live) {
                        graph.startLive(window.scopeChannel, window.refreshMs)
                    } else {
                        graph.loadOscilloscopeData(window.scopeChannel)
                    }
                }
            }

            CheckBox {
                id: liveChk
                text: qsTr("Live")
                checked: window.live
                onToggled: {
                    window.live = checked
                    if (checked) {
                        graph.startLive(window.scopeChannel, window.refreshMs)
                    } else {
                        graph.stopLive()
                    }
                }
            }

            Button {
                text: qsTr("Refresh")
                onClicked: graph.loadOscilloscopeData(window.scopeChannel)
            }

        }

        GraphObject {
            id: graph
            Layout.fillWidth: true
            Layout.fillHeight: true
            legendVisible: false
            legendPosition: 1
            gridVisible: true
            xLabel: qsTr("Time (s)")
            // szkóp jellegű rács (Rigol DS1000Z tipikus: 12 osztás vízszint, 8 függőlegesen)
            xDivisions: 12
            yDivisions: 8

            Component.onCompleted: {
                graph.startLive(window.scopeChannel, window.refreshMs)
            }

            MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.AllButtons
                property bool dragging: false
                property bool panning: false
                property int panMode: 0    // 0: both, 1: horizontal-only, 2: vertical-only
                property real dragStartX: 0
                property real dragStartY: 0
                property real dragLastX: 0
                property real dragLastY: 0

                onPressed: {
                    if (mouse.button === Qt.RightButton) {
                        contextMenu.popup()
                    } else if (mouse.button === Qt.MiddleButton) {
                        dragging = true
                        panning = true
                        panMode = 0
                        dragLastX = mouse.x
                        dragLastY = mouse.y
                    } else if (mouse.button === Qt.LeftButton) {
                        if (mouse.x < 60 || mouse.y > graph.height - 50) {
                            dragging = true
                            panning = true
                            dragLastX = mouse.x
                            dragLastY = mouse.y
                            if (mouse.x < 60 && !(mouse.y > graph.height - 50)) {
                                panMode = 2  // vertical only
                            } else if (mouse.y > graph.height - 50 && !(mouse.x < 60)) {
                                panMode = 1  // horizontal only
                            } else {
                                panMode = 0
                            }
                        } else {
                            dragging = true
                            panning = false
                            dragStartX = mouse.x
                            dragStartY = mouse.y
                        }
                    }
                }
                onReleased: {
                    if (mouse.button === Qt.LeftButton && dragging) {
                        if (panning) {
                            dragging = false
                            panning = false
                        } else {
                            dragging = false
                            var dx = mouse.x - dragStartX
                            var dy = mouse.y - dragStartY
                            if (Math.abs(dx) < 5 && Math.abs(dy) < 5) {
                                // Click -> place cursor
                                if (Qt.shiftModifier & mouse.modifiers) {
                                    var dataY = graph.yMax - (mouse.y - graph.y) / graph.height * (graph.yMax - graph.yMin)
                                    graph.placeHorizontalCursor(dataY)
                                } else {
                                    var dataX = graph.xMin + (mouse.x - graph.x) / graph.width * (graph.xMax - graph.xMin)
                                    graph.placeVerticalCursor(dataX)
                                }
                            } else {
                                // Drag -> zoom to selected region
                                var x1 = graph.xMin + (Math.min(dragStartX, mouse.x) - graph.x) / graph.width * (graph.xMax - graph.xMin)
                                var x2 = graph.xMin + (Math.max(dragStartX, mouse.x) - graph.x) / graph.width * (graph.xMax - graph.xMin)
                                var y1 = graph.yMax - (Math.max(dragStartY, mouse.y) - graph.y) / graph.height * (graph.yMax - graph.yMin)
                                var y2 = graph.yMax - (Math.min(dragStartY, mouse.y) - graph.y) / graph.height * (graph.yMax - graph.yMin)
                                if (modeCombo.currentIndex === 0) {
                                    graph.zoomToRegion(x1, x2, y1, y2)
                                } else if (modeCombo.currentIndex === 1) {
                                    graph.zoomX(x1, x2)
                                } else if (modeCombo.currentIndex === 2) {
                                    graph.zoomY(y1, y2)
                                }
                            }
                        }
                    } else if (mouse.button === Qt.MiddleButton && dragging) {
                        dragging = false
                        panning = false
                    }
                }
                onPositionChanged: {
                    if (dragging && panning) {
                        var dx = mouse.x - dragLastX
                        var dy = mouse.y - dragLastY
                        if (panMode === 1) {
                            dy = 0
                        } else if (panMode === 2) {
                            dx = 0
                        }
                        if (dx !== 0 || dy !== 0) {
                            var dx_data = -dx / graph.width * (graph.xMax - graph.xMin)
                            var dy_data = dy / graph.height * (graph.yMax - graph.yMin)
                            graph.pan(dx_data, dy_data)
                            dragLastX = mouse.x
                            dragLastY = mouse.y
                        }
                    } else if (mouse.buttons === Qt.LeftButton && dragging && !panning) {
                        // (Optionally draw selection rectangle while dragging)
                    }
                }
                WheelHandler {
                    onWheel: {
                        var zoomFactor = (wheel.angleDelta.y > 0 ? 1.2 : 1 / 1.2)
                        var px = wheel.x - graph.x
                        var py = wheel.y - graph.y
                        var centerX = graph.xMin + px / graph.width * (graph.xMax - graph.xMin)
                        var centerY = graph.yMax - py / graph.height * (graph.yMax - graph.yMin)
                        graph.zoomAtPoint(centerX, centerY, zoomFactor)
                    }
                }
            }
        }
        Connections {
            target: graph

            function onRequestCopyData(text) {
                // Qt 6+: clipboard elérés QML-ből
                Qt.application.clipboard.setText(text)
            }

            function onRequestCopyImage() {
                graph.grabToImage(function (result) {
                    Qt.application.clipboard.setImage(result.image)
                })
            }

            function onRequestSaveImage(filePath) {
                graph.grabToImage(function (result) {
                    result.saveToFile(filePath)
                })
            }
        }
    }
    // UI-thread pump: csak a legfrissebb már lekért waveform-ot húzza be, nem blokkol IO-val
    Timer {
        interval: 30
        running: window.live
        repeat: true
        onTriggered: graph.pumpLive()
    }

    onClosing: {
        graph.stopLive()
    }

    Menu {
        id: contextMenu
        MenuItem {
            text: qsTr("Save Image")
            onTriggered: fileDialogImage.open()
        }
        MenuItem {
            text: qsTr("Save Data")
            onTriggered: fileDialogData.open()
        }
        MenuItem {
            text: qsTr("Copy Image")
            onTriggered: graph.copyImage()
        }
        MenuItem {
            text: qsTr("Copy Data")
            onTriggered: graph.copyData()
        }
        MenuItem {
            text: qsTr("Clear Cursors")
            onTriggered: graph.clearCursors()
        }
    }

    FileDialog {
        id: fileDialogImage
        title: qsTr("Save Image")
        nameFilters: ["PNG Image (*.png)"]
        onAccepted: {
            graph.grabToImage(function (result) {
                result.saveToFile(fileDialogImage.fileUrl.toLocalFile())
            })
        }
    }
    FileDialog {
        id: fileDialogData
        title: qsTr("Save Data")
        nameFilters: ["CSV File (*.csv)"]
        onAccepted: {
            graph.saveCsv(fileDialogData.fileUrl.toLocalFile())
        }
    }
}
