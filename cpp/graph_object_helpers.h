// cpp/graph_object_helpers.h

#pragma once

#include <memory>
#include <QtCore/QString>
#include <QtGui/QImage>

namespace graph_object_qobject {
class GraphObject; // forward declare the generated QObject type
}

namespace graph_object_helpers {
std::unique_ptr<QImage> grab_image(graph_object_qobject::GraphObject* item);
bool save_image(graph_object_qobject::GraphObject* item, const QString& file_path);
} // namespace graph_object_helpers
