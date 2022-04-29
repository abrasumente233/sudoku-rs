#![feature(slice_as_chunks)]

use std::fs;
use rayon::prelude::*;

#[link(name = "sudoku")]
extern "C" {
    fn dance_solve(sudoku: *mut u8);
}

extern "C" {
    fn write(fd: i32, buf: *const libc::c_void, count: libc::size_t) -> libc::ssize_t;
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

#[cfg(windows)]
fn batch_print(sudokus: Vec<&mut [u8]>) {
    for sudoku in sudokus {
        let s = std::str::from_utf8(sudoku).unwrap();
        println!("{s}");
    }
}

#[cfg(unix)]
fn batch_print(sudokus: Vec<&mut [u8]>) {
    unsafe {
        let buf = sudokus[0].as_ptr() as *const libc::c_void;
        let count = 82 * sudokus.len() - 1;
        write(1, buf, count);
        let buf = b'\n';
        let buf_ptr: *const u8 = &buf;
        write(1, buf_ptr as *const libc::c_void, 1);
    }
}

fn main() {
    let mut sudokus = fs::read_to_string("tests/test1000000").unwrap();

    let sudokus = split_sudokus(&mut sudokus);

    let v: Vec<&mut [u8]> = sudokus
        .into_par_iter()
        .map(|s| {
            solve_sudoku(s);
            s
        })
        .collect();

    batch_print(v);
}
