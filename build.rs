// build.rs
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .file("src/gui_bridge.rs")      // generate C++ code from the CXX-Qt bridge
        .qrc("qml/qml.qrc")            // include QML resources (main.qml in qrc)
        .build();
}
