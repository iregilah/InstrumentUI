// qml/main.qml
import QtQuick 2.15
import QtQuick.Controls 2.15
import RigolApp 1.0    // import the module from Rust (qml_uri "RigolApp", version 1.0)

Window {
    visible: true
    width: 400
    height: 300
    title: qsTr("Rigol SCPI Demo")

    // Instantiate the Rust backend object
    Backend {
        id: backend
    }

    Column {
        anchors.centerIn: parent
        spacing: 20

        Button {
            text: qsTr("Run Basic Demo")
            onClicked: backend.run_demo()
        }
    }
}