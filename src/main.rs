#![feature(slice_as_chunks)]

use std::fs;

#[link(name = "sudoku")]
extern "C" {
    fn dance_solve(sudoku: *mut u8);
}

fn solve_sudoku(sudoku: &mut [u8]) {
    unsafe { dance_solve(sudoku.as_mut_ptr()) }
}

fn split_sudokus<'a>(sudokus: &'a mut str) -> Vec<&'a mut [u8]> {
    unsafe {
        sudokus
            .as_bytes_mut()
            .as_chunks_mut::<82>()
            .0
            .into_iter()
            .map(|s| &mut s[..81])
            .collect()
    }
}

fn batch_print(sudokus: Vec<&mut [u8]>) {
    for sudoku in sudokus {
        let s = std::str::from_utf8(sudoku).unwrap();
        println!("{s}");
    }
}

fn main() {
    let mut sudokus = fs::read_to_string("tests/test50").unwrap();

    let sudokus = split_sudokus(&mut sudokus);

    let v: Vec<&mut [u8]> = sudokus
        .into_iter()
        .map(|s| {
            solve_sudoku(s);
            s
        })
        .collect();

    batch_print(v);
}
