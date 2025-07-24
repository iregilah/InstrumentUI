// qml/main.qml
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15

// Import the Rust QML module (matches qml_uri and version from Rust):contentReference[oaicite:12]{index=12}
import RigolDemo 1.0

Window {
    visible: true
    width: 640
    height: 480
    title: qsTr("Rigol Demo App")

    // Instantiate the Rust QObject (exposed via cxx-qt) in QML
    Rigol {
        id: rigolController
    }

    Rectangle {
        anchors.fill: parent
        color: "#ececec"
        Button {
            text: qsTr("Run Demo")
            anchors.centerIn: parent
            // On button click, call the Rust invokable method:contentReference[oaicite:13]{index=13}
            onClicked: rigolController.runDemo()
        }
    }
}