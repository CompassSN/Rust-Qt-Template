use std::vec;

fn main() {
    println!("cargo:rerun-if-changed=./cpp/lib.cpp");
    println!("cargo:rerun-if-changed=./cpp/lib.h");

    let qt_modules = vec!["Core", "Gui", "Widgets"]
        .iter()
        .map(|m|String::from(*m))
        .collect();
    
    let qtbuild = qt_build_utils::QtBuild::new(qt_modules).unwrap();

    cxx_build::bridge("src/main.rs")
        .cpp(true)
        .flag("-std=c++17")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-v")
        .flag("-g")
        .includes(qtbuild.include_paths())
        .file("cpp/lib.cpp")
        .compile("cppqt");
    
    qtbuild.cargo_link_libraries();
}