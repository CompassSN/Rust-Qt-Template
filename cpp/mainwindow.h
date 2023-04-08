#pragma once

#include <QMainWindow>
#include <QLabel>
#include <QAction>
#include <QMessageBox>
#include <iostream>
#include <string>
#include "../target/ui/ui_mainwindow.h"


QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:

    MainWindow(QWidget *parent = nullptr): QMainWindow(parent){
        ui = new Ui::MainWindow;
    
        ui->setupUi(this);
    }
    ~MainWindow(){};

private:
    Ui::MainWindow *ui;
};