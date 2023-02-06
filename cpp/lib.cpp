#include "lib.h"

int init(int argc, size_t argv){
    QApplication app(argc, (char**)argv);
    QWidget widget = QWidget();

    widget.show();

    return app.exec();
}