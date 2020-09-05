#include "mainwindow.h"
#include "ui_mainwindow.h"

#include <QMessageBox>

MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::MainWindow)
{
    ui->setupUi(this);
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::on_actionAbout_Lorenz_triggered()
{
    QMessageBox::about(this, "About Lorenz",
                       "<h2>Lorenz v1.0.0</h2>"
                       "<p>Copyright (C) Arash Hatami 2020</p>"
                       "<p>Licensed under the GNU General Public License v3.0</p>"
                       "<p><a href=\"https://github.com/spieglt/lorenz\">https://github.com/hatamiarash7/lorenz</a></p>"
                       "<p><b>WARNING:</b> if you encrypt a file and lose or forget the password, the file cannot be recovered.</p>"
    );
}
