extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("sudoku.cpp")
        .compile("libsudoku.a");
}
