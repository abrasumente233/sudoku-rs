#![feature(slice_as_chunks)]

use rayon::prelude::*;
use std::fs;

#[link(name = "sudoku")]
extern "C" {
    fn dance_solve(sudoku: *mut u8);
}

#[cfg(unix)]
extern "C" {
    fn write(fd: i32, buf: *const libc::c_void, count: libc::size_t) -> libc::ssize_t;
}

fn solve_sudoku(sudoku: &mut [u8]) -> &mut [u8] {
    unsafe { dance_solve(sudoku.as_mut_ptr()) }
    sudoku
}

fn split_sudokus(sudokus: &mut str) -> Vec<&mut [u8]> {
    unsafe {
        sudokus
            .as_bytes_mut()
            .as_chunks_mut::<82>()
            .0
            .iter_mut()
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
    loop {
        let mut filename = String::new();
        std::io::stdin().read_line(&mut filename).unwrap();
        let filename = filename.trim_end();

        let mut sudokus = match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(_) => break,
        };

        let v: Vec<&mut [u8]> = split_sudokus(&mut sudokus)
            .into_par_iter()
            .map(solve_sudoku)
            .collect();

        batch_print(v);
    }
}
