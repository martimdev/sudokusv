extern crate term_painter;

use std::thread;
use std::time::Duration;
use term_painter::Color::Custom;
use term_painter::ToStyle;

pub struct Sudoku {
    columns: [[u32; 9]; 9],
    horizontal_lines: [[u32; 9]; 9],
    vertical_lines: [[u32; 9]; 9],
}

impl Sudoku {
    pub fn new(horizontal_lines: [[u32; 9]; 9]) -> Sudoku {
        let mut sudoku_instance = Sudoku {
            columns: [[0; 9]; 9],
            horizontal_lines,
            vertical_lines: [[0; 9]; 9],
        };
        sudoku_instance.update_by_horizontal_lines();
        return sudoku_instance;
    }

    pub fn solve(&self) {
        while self.is_correct() {
            print_sudoku(self);
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn is_correct(&self) -> bool {
        fn verify_array(array2: [[u32; 9]; 9]) -> bool {
            for array1 in array2.iter() {
                for (i, _) in array1.iter().enumerate() {
                    if array1[i] != 0 && array1.iter().filter(|&n| *n == array1[i]).count() > 1 {
                        return false;
                    }
                }
            }
            return true;
        }
        verify_array(self.horizontal_lines) && verify_array(self.vertical_lines) && verify_array(self.columns)
    }

    fn get_columns_by_horizontal_lines(&self) -> [[u32; 9]; 9] {
        [
            [
                self.horizontal_lines[0][0], self.horizontal_lines[0][1], self.horizontal_lines[0][2],
                self.horizontal_lines[1][0], self.horizontal_lines[1][1], self.horizontal_lines[1][2],
                self.horizontal_lines[2][0], self.horizontal_lines[2][1], self.horizontal_lines[2][2]
            ],
            [
                self.horizontal_lines[0][3], self.horizontal_lines[0][4], self.horizontal_lines[0][5],
                self.horizontal_lines[1][3], self.horizontal_lines[1][4], self.horizontal_lines[1][5],
                self.horizontal_lines[2][3], self.horizontal_lines[2][4], self.horizontal_lines[2][5]
            ],
            [
                self.horizontal_lines[0][6], self.horizontal_lines[0][7], self.horizontal_lines[0][8],
                self.horizontal_lines[1][6], self.horizontal_lines[1][7], self.horizontal_lines[1][8],
                self.horizontal_lines[2][6], self.horizontal_lines[2][7], self.horizontal_lines[2][8]
            ],
            [
                self.horizontal_lines[3][0], self.horizontal_lines[3][1], self.horizontal_lines[3][2],
                self.horizontal_lines[4][0], self.horizontal_lines[4][1], self.horizontal_lines[4][2],
                self.horizontal_lines[5][0], self.horizontal_lines[5][1], self.horizontal_lines[5][2]
            ],
            [
                self.horizontal_lines[3][3], self.horizontal_lines[3][4], self.horizontal_lines[3][5],
                self.horizontal_lines[4][3], self.horizontal_lines[4][4], self.horizontal_lines[4][5],
                self.horizontal_lines[5][3], self.horizontal_lines[5][4], self.horizontal_lines[5][5]
            ],
            [
                self.horizontal_lines[3][6], self.horizontal_lines[3][7], self.horizontal_lines[3][8],
                self.horizontal_lines[4][6], self.horizontal_lines[4][7], self.horizontal_lines[4][8],
                self.horizontal_lines[5][6], self.horizontal_lines[5][7], self.horizontal_lines[5][8]
            ],
            [
                self.horizontal_lines[6][0], self.horizontal_lines[6][1], self.horizontal_lines[6][2],
                self.horizontal_lines[7][0], self.horizontal_lines[7][1], self.horizontal_lines[7][2],
                self.horizontal_lines[8][0], self.horizontal_lines[8][1], self.horizontal_lines[8][2]
            ],
            [
                self.horizontal_lines[6][3], self.horizontal_lines[6][4], self.horizontal_lines[6][5],
                self.horizontal_lines[7][3], self.horizontal_lines[7][4], self.horizontal_lines[7][5],
                self.horizontal_lines[8][3], self.horizontal_lines[8][4], self.horizontal_lines[8][5]
            ],
            [
                self.horizontal_lines[6][6], self.horizontal_lines[6][7], self.horizontal_lines[6][8],
                self.horizontal_lines[7][6], self.horizontal_lines[7][7], self.horizontal_lines[7][8],
                self.horizontal_lines[8][6], self.horizontal_lines[8][7], self.horizontal_lines[8][8]
            ],
        ]
    }

    fn get_vertical_lines_by_horizontal_lines(&self) -> [[u32; 9]; 9] {
        let mut vertical_lines = [[0; 9]; 9];
        for line_index in 0..9 {
            for index in 0..9 {
                vertical_lines[line_index][index] = self.horizontal_lines[index][line_index];
            }
        }
        vertical_lines
    }

    fn update_by_horizontal_lines(&mut self) {
        self.columns = self.get_columns_by_horizontal_lines();
        self.vertical_lines = self.get_vertical_lines_by_horizontal_lines();
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