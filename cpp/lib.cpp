#include "lib.h"


int init(int argc, size_t argv){
    QApplication app(argc, (char**)argv);
    MainWindow widget = MainWindow();

    widget.show();

    return app.exec();
}