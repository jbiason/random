fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/external/aclass.cpp")
        .compile("cxxtest");
}
