use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        // Link Qt's Network module (Qt Core/Gui/Qml are linked by default features).
        .qt_module("Network")
        // Generate C++ from the bridge module (cxx_qt_object.rs).
        .file("src/cxxqt_object.rs")
        // Include the QML resource file (compile qml.qrc with Qt's rcc tool).
        .qrc("qml/qml.qrc")
        // No explicit setup_linker() call needed in cxx-qt 0.7.2+ (handled internally)
        .build();
}
