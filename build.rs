use std::{vec, process::Command, fs::{self, create_dir_all}, path::PathBuf, ffi::OsString};

fn main() {
    println!("cargo:rerun-if-changed=./cpp/");
    println!("cargo:rerun-if-changed=./ui/");

    let qt_modules = vec!["Core", "Gui", "Widgets"]
        .iter()
        .map(|m|String::from(*m))
        .collect();
    
    let qtbuild = qt_build_utils::QtBuild::new(qt_modules).unwrap();

    //convert .ui file to .h file using uic
    let mut ui_header_dir_path = PathBuf::from("./target");
    ui_header_dir_path.push("ui");
    create_dir_all(ui_header_dir_path.as_path()).unwrap();
    let uifiles = fs::read_dir("./ui/").unwrap();
    for ui in uifiles{
        let ui_file_path = ui.unwrap().path();
        let mut ui_header_path = ui_header_dir_path.clone();
        ui_header_path.push(format!("ui_{}",ui_file_path.file_stem().unwrap().to_str().unwrap()));
        ui_header_path.set_extension("h");

        Command::new("uic")
            .args([ui_file_path.as_os_str(), OsString::from("-o").as_os_str(), ui_header_path.as_os_str()])
            .spawn()
            .expect("failed to convert .ui to .h");
    }

    cxx_build::bridge("src/main.rs")
        .cpp(true)
        .flag("-std=c++17")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-v")
        .flag("-g")
        .includes(qtbuild.include_paths())
        .include("./target/ui/")
        .include("/usr/include/qt6")
        .file("cpp/lib.cpp")
        .file("cpp/mainwindow.cpp")
        .compile("cppqt");
    
    qtbuild.cargo_link_libraries();
}