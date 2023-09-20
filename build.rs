use std::{vec, process::Command, fs, path::Path};



const UI_DIR: &str = "./ui";
const CPP_DIR: &str = "./cpp";


const QOBJECT_FILES: [&'static str; 1] = ["./cpp/mainwindow.h"];


fn main() {
    let qt6 = true;


    let (UIC_LOC, MOC_LOC) = if qt6{
        std::env::set_var("QMAKE", "qmake6");
        std::env::set_var("QT_VERSION_MAJOR", "6");
        ("/usr/lib/qt6/uic", "/usr/lib/qt6/moc")
    }else {
        std::env::set_var("QMAKE", "qmake");
        std::env::set_var("QT_VERSION_MAJOR", "5");
        ("uic", "moc")
    };

    println!("cargo:rerun-if-changed=./cpp/");
    println!("cargo:rerun-if-changed=./ui/");

    fs::create_dir_all("./target/ui").unwrap();
    fs::create_dir_all("./target/moc").unwrap();
    
    let qt_modules = vec!["Core", "Gui", "Widgets"]
        .iter()
        .map(|m|{String::from(*m)})
        .collect();
    let qtbuild = qt_build_utils::QtBuild::new(qt_modules).unwrap();


    //run uic
    for f in fs::read_dir(UI_DIR).unwrap(){
        let f = f.unwrap();
        Command::new(UIC_LOC)
            .args([f.path().to_str().unwrap(), "-o", format!("./target/ui/ui_{}.h", f.path().file_stem().unwrap().to_str().unwrap()).as_str()])
            .output()
            .expect("failed to run uic");
    }

    let mut includes = vec![];
    for b in qtbuild.include_paths(){
        includes.push(b.to_string_lossy().to_string());
    }
    includes.push("/usr/include/LayerShellQt".to_string());

    //run moc
    let mut files = vec![];
    for f in QOBJECT_FILES{
        let f = Path::new(f);
        let out_file = f.file_stem().unwrap().to_str().unwrap().to_string();
        let out_file =  format!("./target/moc/moc_{}.cpp", &out_file);
        Command::new(MOC_LOC)
            .args([f.to_str().unwrap(), "-o", out_file.as_str()])
            .output()
            .expect("failed to run uic");
        files.push(out_file);
        /*let f = std::fs::File::open(&out_file).unwrap();
        if f.metadata().unwrap().len() == 0{
            drop(f);
            fs::remove_file(&out_file).unwrap_or_else(|_|{
                println!("cargo:warning=不要なファイルの削除に失敗しました。");
                ()
            });
        }*/
    }

    cxx_build::bridge("src/main.rs")
        .cpp(true)
        .flag("-std=c++17")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-v")
        .flag("-g")
        .includes(&includes)
        .file("./cpp/lib.cpp")
        .files(files)
        .compile("cppqt");

    //println!("cargo:rustc-link-search=/usr/lib/qt/plugins/wayland-shell-integration/");
    println!("cargo:rustc-link-lib=LayerShellQtInterface");
    //println!("cargo:rustc-link-lib=layer-shell");

    qtbuild.cargo_link_libraries();
}