// qml/main.qml
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15

import RigolDemo 1.0    // A Rust-ban definiált QML modul importja:contentReference[oaicite:10]{index=10}

Window {
    visible: true
    width: 400
    height: 300
    title: qsTr("Rigol Demo")

    // Rust DeviceController objektum a QML-ben
    DeviceController {
        id: deviceController
    }

    Button {
        text: qsTr("Run Basic Demo")
        anchors.centerIn: parent
        onClicked: deviceController.runDemo()   // gombnyomásra Rust metódus hívása:contentReference[oaicite:11]{index=11}
    }
}
