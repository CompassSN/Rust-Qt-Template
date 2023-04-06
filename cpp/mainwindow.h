#pragma once

#include <QMainWindow>
#include <QLabel>
#include <QAction>
#include "ui_mainwindow.h"

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

protected:
    void resizeEvent(QResizeEvent*){
        
    };

    QLabel* label;

public:
    MainWindow(QWidget *parent = nullptr): QMainWindow(parent){
        ui->setupUi(this);
    }
    ~MainWindow(){

    };

private:
    Ui::MainWindow *ui;
};
