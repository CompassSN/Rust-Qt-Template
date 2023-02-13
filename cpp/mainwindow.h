#include "ui_mainwindow.h"

class MainWindow : public QMainWindow{
    Q_OBJECT

protected:


public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private:
    Ui::MainWindow *ui;
};