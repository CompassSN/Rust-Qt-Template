#include "mainwindow.h"
#include <QPropertyAnimation>
#include <QResizeEvent>
#include <iostream>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow){

        ui->setupUi(this);

}
MainWindow::~MainWindow()
{
    delete ui;
}
