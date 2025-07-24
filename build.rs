// build.rs
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")               // Link Qt's Network module (Core/Gui/Qml are linked by default)
        .file("src/cxxqt_object.rs")        // Generate C++ from the bridge module
        .qrc("qml/qml.qrc")                // Compile QML resource file
        .build();
}
