#include "lib.h"


#include <Shell>
#include <Window>


int init(int argc, size_t argv){
    LayerShellQt::Shell::useLayerShell();
    QApplication app(argc, (char**)argv);
    //MainWindow widget = MainWindow();
    QWidget widget = QWidget();

    widget.show();

    return app.exec();
}