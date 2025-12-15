// GraphViewWindow.qml
import QtQuick 6.5
import QtQuick.Controls 6.5
import QtQuick.Layouts 1.15
import QtQuick.Dialogs 1.3
import InstrumentUI 1.0

ApplicationWindow {
    id: window
    visible: true
    width: 1000
    height: 600
    title: qsTr("Graph View Test")

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
                onToggled: graph.legendVisible = checked
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
        }

        GraphObject {
            id: graph
            Layout.fillWidth: true
            Layout.fillHeight: true
            legendVisible: false
            legendPosition: 1
            gridVisible: true
            xLabel: qsTr("Time (s)")
            yLabel: qsTr("Value")
            bufferSize: 100

            Component.onCompleted: {
                graph.addSeries(QString("Analog1"), 0, Qt.rgba(1, 0, 0, 1), 2.0, 1, false)
                graph.addSeries(QString("Digital1"), 2, Qt.rgba(0, 1, 0, 1), 2.0, 1, false)
                var t = 0.0
                Timer {
                    interval: 50
                    running: true
                    repeat: true
                    onTriggered: {
                        var analogVal = Math.sin(2 * Math.PI * 1 * t)
                        var digitalVal = (Math.floor(t) % 2 === 0) ? 1 : 0
                        graph.addDataPoint(QString("Analog1"), t, analogVal)
                        graph.addDataPoint(QString("Digital1"), t, digitalVal)
                        t += 0.05
                    }
                }
            }

            MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.AllButtons
                property bool dragging: false
                property real dragStartX: 0
                property real dragStartY: 0

                onPressed: {
                    if (mouse.button === Qt.RightButton) {
                        contextMenu.popup()
                    } else if (mouse.button === Qt.LeftButton) {
                        dragging = true
                        dragStartX = mouse.x
                        dragStartY = mouse.y
                    }
                }
                onReleased: {
                    if (mouse.button === Qt.LeftButton && dragging) {
                        dragging = false
                        var dx = mouse.x - dragStartX
                        var dy = mouse.y - dragStartY
                        if (Math.abs(dx) < 5 && Math.abs(dy) < 5) {
                            // Click -> place cursor
                            if (Qt.shiftModifier & mouse.modifiers) {
                                // Place horizontal cursor (Shift-click)
                                var dataY = graph.yMax - (mouse.y - graph.y) / graph.height * (graph.yMax - graph.yMin)
                                graph.placeHorizontalCursor(dataY)
                            } else {
                                // Place vertical cursor
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
                }
                onPositionChanged: {
                    if (mouse.buttons === Qt.LeftButton && dragging) {
                        // (Optionally draw selection rectangle while dragging)
                    }
                }
                WheelHandler {
                    onWheel: {
                        var zoomFactor = (wheel.angleDelta.y > 0 ? 1.2 : 1/1.2)
                        var px = wheel.x - graph.x
                        var py = wheel.y - graph.y
                        var centerX = graph.xMin + px / graph.width * (graph.xMax - graph.xMin)
                        var centerY = graph.yMax - py / graph.height * (graph.yMax - graph.yMin)
                        graph.zoomAtPoint(centerX, centerY, zoomFactor)
                    }
                }
            }
        }
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
    }

    FileDialog {
        id: fileDialogImage
        title: qsTr("Save Image")
        nameFilters: ["PNG Image (*.png)"]
        onAccepted: {
            graph.grabToImage(function(result) {
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
