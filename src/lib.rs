extern crate term_painter;

use std::thread;
use std::time::Duration;
use term_painter::Color::Custom;
use term_painter::ToStyle;

pub struct Sudoku {
    pub columns: [[u32; 9]; 9],
    pub horizontal_lines: [[u32; 9]; 9],
    pub vertical_lines: [[u32; 9]; 9],
}

impl Sudoku {
    pub fn new(horizontal_lines: [[u32; 9]; 9]) -> Sudoku {
        let mut sudoku_instance = Sudoku {
            columns: [[0; 9]; 9],
            horizontal_lines,
            vertical_lines: [[0; 9]; 9],
        };
        sudoku_instance.columns = sudoku_instance.get_columns_by_horizontal_lines();
        sudoku_instance.vertical_lines = sudoku_instance.get_vertical_lines_by_columns();
        return sudoku_instance;
    }

    pub fn solve(&self) {
        loop {
            print_sudoku(self);
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn get_columns_by_horizontal_lines(&self) -> [[u32; 9]; 9] {
        return [[0; 9]; 9];
    }

    fn get_vertical_lines_by_columns(&self) -> [[u32; 9]; 9] {
        return [[0; 9]; 9];
    }

    pub fn get_horizontal_lines_by_vertical_lines(&self) -> [[u32; 9]; 9] {
        return [[0; 9]; 9];
    }
}

pub fn print_sudoku(sudoku: &Sudoku) {
    println!("{}{}{}",
             Custom(1).paint("----------"),
             Custom(2).paint("---------"),
             Custom(3).paint("----------"),
    );
    for (i, line) in sudoku.horizontal_lines.iter().enumerate() {
        match i {
            0 | 1 | 2 => print!("{}", Custom(1).paint("|")),
            3 | 4 | 5 => print!("{}", Custom(4).paint("|")),
            6 | 7 | 8 => print!("{}", Custom(5).paint("|")),
            _ => print!("{}", "|")
        }
        for (index, lines_index) in line.iter().enumerate() {
            match i {
                0 | 1 | 2 => {
                    match index {
                        0 | 1 | 2 => print!(" {} ", Custom(1).paint(lines_index)),
                        3 | 4 | 5 => print!(" {} ", Custom(2).paint(lines_index)),
                        6 | 7 | 8 => print!(" {} ", Custom(3).paint(lines_index)),
                        _ => print!(" {index} ", index = lines_index)
                    }
                }
                3 | 4 | 5 => {
                    match index {
                        0 | 1 | 2 => print!(" {} ", Custom(4).paint(lines_index)),
                        3 | 4 | 5 => print!(" {} ", Custom(7).paint(lines_index)),
                        6 | 7 | 8 => print!(" {} ", Custom(6).paint(lines_index)),
                        _ => print!(" {index} ", index = lines_index)
                    }
                }
                6 | 7 | 8 => {
                    match index {
                        0 | 1 | 2 => print!(" {} ", Custom(5).paint(lines_index)),
                        3 | 4 | 5 => print!(" {} ", Custom(10).paint(lines_index)),
                        6 | 7 | 8 => print!(" {} ", Custom(9).paint(lines_index)),
                        _ => print!(" {index} ", index = lines_index)
                    }
                }
                _ => print!(" {} ", lines_index)
            }
        }
        match i {
            0 | 1 => {
                println!("{}", Custom(3).paint("|"));
                println!("{}{}{}",
                         Custom(1).paint("----------"),
                         Custom(2).paint("---------"),
                         Custom(3).paint("----------"),
                );
            }
            2 | 3 | 4 | 5 => {
                println!("{}", Custom(6).paint("|"));
                println!("{}{}{}",
                         Custom(4).paint("----------"),
                         Custom(7).paint("---------"),
                         Custom(6).paint("----------"),
                );
            }
            6 | 7 | 8 => {
                println!("{}", Custom(9).paint("|"));
                println!("{}{}{}",
                         Custom(5).paint("----------"),
                         Custom(10).paint("---------"),
                         Custom(9).paint("----------"),
                );
            }
            _ => {
                println!("|");
                println!("-----------------------------");
            }
        }
    }
}