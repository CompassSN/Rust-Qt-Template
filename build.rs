use std::{vec, process::Command, fs};

fn main() {
    println!("cargo:rerun-if-changed=./cpp/");
    println!("cargo:rerun-if-changed=./ui/");

    fs::create_dir_all("./target/ui").unwrap();
    fs::create_dir_all("./target/moc").unwrap();
    
    let qt_modules = vec!["Core", "Gui", "Widgets"]
        .iter()
        .map(|m|String::from(*m))
        .collect();
    let qtbuild = qt_build_utils::QtBuild::new(qt_modules).unwrap();

    let mut s = String::new();
    for b in qtbuild.include_paths(){
        s += &format!(" -l{}", b.to_string_lossy());
    }

    Command::new("/usr/lib/qt6/uic")
        .args(["./ui/mainwindow.ui", "-o", "./target/ui/ui_mainwindow.h"])
        .output()
        .expect("failed to run uic");
    Command::new("/usr/lib/qt6/moc")
        .args(["./cpp/mainwindow.h", "-o", "./target/moc/moc_mainwindow.cpp"])
        .output()
        .expect("failed to run moc");


    cxx_build::bridge("src/main.rs")
        .cpp(true)
        .flag("-std=c++17")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-v")
        .flag("-g")
        .includes(qtbuild.include_paths())
        .includes(["./target/ui/", "./target/moc/"])
        .file("./target/moc/moc_mainwindow.cpp")
        .file("./cpp/lib.cpp")
        .compile("cppqt");
    
    qtbuild.cargo_link_libraries();
}