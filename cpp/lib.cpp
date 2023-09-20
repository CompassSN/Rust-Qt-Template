#include "lib.h"

#include <stdio.h>
#include <Shell>
#include <Window>
#include <QWindow>
#include <QRasterWindow>
#include <QList>
#include <QGuiApplication>
#include <QScreen>
#include <QDebug>


int init(int argc, size_t argv){
    QGuiApplication app(argc, (char**)argv);
    //MainWindow widget = MainWindow();
    QRasterWindow widget = QRasterWindow();

    QList<QScreen*> screens = QGuiApplication::screens();

    qDebug() << screens;
    qDebug() << screens.last();

    widget.setScreen(screens.last());

    LayerShellQt::Window *layerShell = LayerShellQt::Window::get(&widget);
    layerShell->setLayer(LayerShellQt::Window::LayerTop);


    widget.show();

    return app.exec();
}