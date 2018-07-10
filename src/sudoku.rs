extern crate term_painter;

use self::term_painter::Color::Custom;
use self::term_painter::ToStyle;
use std::thread;
use std::time::Duration;

pub struct Sudoku {
    pub columns: [[u32; 9]; 9],
    pub horizontal_lines: [[u32; 9]; 9],
    pub vertical_lines: [[u32; 9]; 9],
}

impl Sudoku {
    pub fn new(columns: [[u32; 9]; 9]) -> Sudoku {
        let mut sudoku_instance = Sudoku {
            columns,
            horizontal_lines: [[0; 9]; 9],
            vertical_lines: [[0; 9]; 9],
        };
        sudoku_instance.horizontal_lines = sudoku_instance.get_horizontal_lines_by_columns();
        return sudoku_instance;
    }

    pub fn solve(&self) {
        loop {
            print_sudoku(self);
            thread::sleep(Duration::from_millis(500));
        }
    }

    pub fn get_horizontal_lines_by_columns(&self) -> [[u32; 9]; 9] {
        let mut lines_by_columns: [[u32; 9]; 9] = [[0; 9]; 9];

        let mut line1: [u32; 9] = [0; 9];
        line1[0] = self.columns[0][0];
        line1[1] = self.columns[0][1];
        line1[2] = self.columns[0][2];
        line1[3] = self.columns[1][0];
        line1[4] = self.columns[1][1];
        line1[5] = self.columns[1][2];
        line1[6] = self.columns[2][0];
        line1[7] = self.columns[2][1];
        line1[8] = self.columns[2][2];
        lines_by_columns[0] = line1;

        let mut line2: [u32; 9] = [0; 9];
        line2[0] = self.columns[0][3];
        line2[1] = self.columns[0][4];
        line2[2] = self.columns[0][5];
        line2[3] = self.columns[1][3];
        line2[4] = self.columns[1][4];
        line2[5] = self.columns[1][5];
        line2[6] = self.columns[2][3];
        line2[7] = self.columns[2][4];
        line2[8] = self.columns[2][5];
        lines_by_columns[1] = line2;

        let mut line3: [u32; 9] = [0; 9];
        line3[0] = self.columns[0][6];
        line3[1] = self.columns[0][7];
        line3[2] = self.columns[0][8];
        line3[3] = self.columns[1][6];
        line3[4] = self.columns[1][7];
        line3[5] = self.columns[1][8];
        line3[6] = self.columns[2][6];
        line3[7] = self.columns[2][7];
        line3[8] = self.columns[2][8];
        lines_by_columns[2] = line3;

        let mut line4: [u32; 9] = [0; 9];
        line4[0] = self.columns[3][0];
        line4[1] = self.columns[3][1];
        line4[2] = self.columns[3][2];
        line4[3] = self.columns[4][0];
        line4[4] = self.columns[4][1];
        line4[5] = self.columns[4][2];
        line4[6] = self.columns[5][0];
        line4[7] = self.columns[5][1];
        line4[8] = self.columns[5][2];
        lines_by_columns[3] = line4;

        let mut line5: [u32; 9] = [0; 9];
        line5[0] = self.columns[3][3];
        line5[1] = self.columns[3][4];
        line5[2] = self.columns[3][5];
        line5[3] = self.columns[4][3];
        line5[4] = self.columns[4][4];
        line5[5] = self.columns[4][5];
        line5[6] = self.columns[5][3];
        line5[7] = self.columns[5][4];
        line5[8] = self.columns[5][5];
        lines_by_columns[4] = line5;

        let mut line6: [u32; 9] = [0; 9];
        line6[0] = self.columns[3][6];
        line6[1] = self.columns[3][7];
        line6[2] = self.columns[3][8];
        line6[3] = self.columns[4][6];
        line6[4] = self.columns[4][7];
        line6[5] = self.columns[4][8];
        line6[6] = self.columns[5][6];
        line6[7] = self.columns[5][7];
        line6[8] = self.columns[5][8];
        lines_by_columns[5] = line6;

        let mut line7: [u32; 9] = [0; 9];
        line7[0] = self.columns[6][0];
        line7[1] = self.columns[6][1];
        line7[2] = self.columns[6][2];
        line7[3] = self.columns[7][0];
        line7[4] = self.columns[7][1];
        line7[5] = self.columns[7][2];
        line7[6] = self.columns[8][0];
        line7[7] = self.columns[8][1];
        line7[8] = self.columns[8][2];
        lines_by_columns[6] = line7;

        let mut line8: [u32; 9] = [0; 9];
        line8[0] = self.columns[6][3];
        line8[1] = self.columns[6][4];
        line8[2] = self.columns[6][5];
        line8[3] = self.columns[7][3];
        line8[4] = self.columns[7][4];
        line8[5] = self.columns[7][5];
        line8[6] = self.columns[8][3];
        line8[7] = self.columns[8][4];
        line8[8] = self.columns[8][5];
        lines_by_columns[7] = line8;

        let mut line9: [u32; 9] = [0; 9];
        line9[0] = self.columns[6][6];
        line9[1] = self.columns[6][7];
        line9[2] = self.columns[6][8];
        line9[3] = self.columns[7][6];
        line9[4] = self.columns[7][7];
        line9[5] = self.columns[7][8];
        line9[6] = self.columns[8][6];
        line9[7] = self.columns[8][7];
        line9[8] = self.columns[8][8];
        lines_by_columns[8] = line9;

        return lines_by_columns;
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
