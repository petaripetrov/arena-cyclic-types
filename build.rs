fn main() {
    // Link C++ code, flags go here 
    cxx_build::bridge("src/cpp_arena.rs")
        .file("src/arena.cc")
        .compile("thesis");
}