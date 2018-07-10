#[macro_use(array)]
extern crate ndarray;
extern crate term_painter;

use ndarray::Array2;
use std::thread;
use std::time::Duration;
use term_painter::Color::Custom;
use term_painter::ToStyle;

pub struct Sudoku {
    columns: Array2<u32>,
    horizontal_lines: Array2<u32>,
    vertical_lines: Array2<u32>,
}

impl Sudoku {
    pub fn new(horizontal_lines: Array2<u32>) -> Sudoku {
        let columns: Array2<u32> = array![
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0]
        ];
        let vertical_lines: Array2<u32> = array![
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0]
        ];
        let sudoku_instance = Sudoku {
            columns,
            horizontal_lines,
            vertical_lines,
        };
        //sudoku_instance.columns = sudoku_instance.get_columns_by_horizontal_lines();
        //sudoku_instance.vertical_lines = sudoku_instance.get_vertical_lines_by_columns();
        return sudoku_instance;
    }

    pub fn solve(&self) {
        loop {
            print_sudoku(self);
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn get_columns_by_horizontal_lines(&self) -> Array2<u32> {
        unimplemented!();
    }

    fn get_vertical_lines_by_columns(&self) -> Array2<u32> {
        unimplemented!();
    }

}

pub fn print_sudoku(sudoku: &Sudoku) {
    println!("{}{}{}",
             Custom(1).paint("----------"),
             Custom(2).paint("---------"),
             Custom(3).paint("----------"),
    );
    for (i, line) in sudoku.horizontal_lines.outer_iter().enumerate() {
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