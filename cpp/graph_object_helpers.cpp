// cpp/graph_object_helpers.cpp

#include "graph_object_helpers.h"

#include <QtCore/QEventLoop>
#include <QtCore/QObject>
#include <QtCore/QTimer>
#include <QtQuick/QQuickItemGrabResult>

// IMPORTANT:
// This include path assumes your include_prefix is the crate name: "Instrument_UI".
// This matches your build output include dirs.
//
// If this include fails, see the note below about finding the exact generated path.
#include "Instrument_UI/src/graph_object.cxxqt.h"

namespace graph_object_helpers {

std::unique_ptr<QImage> grab_image(graph_object_qobject::GraphObject* item) {
    if (!item) {
        return {};
    }

    // QQuickItem::grabToImage() returns a QSharedPointer<QQuickItemGrabResult>
    auto result = item->grabToImage();
    if (result.isNull()) {
        return {};
    }

    // Wait until the grab result is ready (synchronous wait).
    // Example patterns connect to result.data() and the ready signal. :contentReference[oaicite:1]{index=1}
    QEventLoop loop;
    QObject::connect(result.data(), &QQuickItemGrabResult::ready,
                     &loop, &QEventLoop::quit);

    // Safety timeout so you don't deadlock forever if ready never fires.
    QTimer timeout;
    timeout.setSingleShot(true);
    QObject::connect(&timeout, &QTimer::timeout, &loop, &QEventLoop::quit);
    timeout.start(2000);

    loop.exec();

    // Even if timeout happened, result->image() will just be empty.
    return std::make_unique<QImage>(result->image());
}

bool save_image(graph_object_qobject::GraphObject* item, const QString& file_path) {
    auto img = grab_image(item);
    if (!img) {
        return false;
    }
    return img->save(file_path);
}

} // namespace graph_object_helpers
