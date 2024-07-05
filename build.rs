fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/greeter.cpp")
        .compile("cxx-basics");

    println!("cargo::rerun-if-changed=src/main.rs");
    println!("cargo::rerun-if-changed=src/greeter.cpp");
    println!("cargo::rerun-if-changed=include/greeter.h");
}
