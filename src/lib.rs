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
        sudoku_instance
    }

    pub fn solve(&mut self) {
        fn finalize(array2: [[u32; 9]; 9]) -> [[u32; 9]; 9] {
            let mut array_return = array2;
            for (index, array) in array2.iter().enumerate() {
                if array.iter().filter(|&n| *n == 0).count() == 1 {
                    let position = array.iter().position(|&v| v == 0).unwrap();
                    let value = get_missing_values(array.clone())[0];
                    array_return[index][position] = value;
                }
            }
            array_return
        }

        while self.is_correct() && !self.is_solved() {
            self.horizontal_lines = finalize(self.horizontal_lines);
            self.update_by_horizontal_lines();
            self.vertical_lines = finalize(self.vertical_lines);
            self.update_by_vertical_lines();
            self.columns = finalize(self.columns);
            self.solve_by_column(0, [0, 1, 2], [0, 1, 2]);
            self.solve_by_column(1, [0, 1, 2], [3, 4, 5]);
            self.solve_by_column(2, [0, 1, 2], [6, 7, 8]);
            self.solve_by_column(3, [3, 4, 5], [0, 1, 2]);
            self.solve_by_column(4, [3, 4, 5], [3, 4, 5]);
            self.solve_by_column(5, [3, 4, 5], [6, 7, 8]);
            self.solve_by_column(6, [6, 7, 8], [0, 1, 2]);
            self.solve_by_column(7, [6, 7, 8], [3, 4, 5]);
            self.solve_by_column(8, [6, 7, 8], [6, 7, 8]);
            self.update_by_columns();
            self.solve_by_horizontal_line(0, [0, 1, 2]);
            self.solve_by_horizontal_line(1, [0, 1, 2]);
            self.solve_by_horizontal_line(2, [0, 1, 2]);
            self.solve_by_horizontal_line(3, [3, 4, 5]);
            self.solve_by_horizontal_line(4, [3, 4, 5]);
            self.solve_by_horizontal_line(5, [3, 4, 5]);
            self.solve_by_horizontal_line(6, [6, 7, 8]);
            self.solve_by_horizontal_line(7, [6, 7, 8]);
            self.solve_by_horizontal_line(8, [6, 7, 8]);
            self.update_by_horizontal_lines();
            self.solve_by_vertical_line(0, [0, 3, 6]);
            self.solve_by_vertical_line(1, [0, 3, 6]);
            self.solve_by_vertical_line(2, [0, 3, 6]);
            self.solve_by_vertical_line(3, [1, 4, 7]);
            self.solve_by_vertical_line(4, [1, 4, 7]);
            self.solve_by_vertical_line(5, [1, 4, 7]);
            self.solve_by_vertical_line(6, [2, 5, 8]);
            self.solve_by_vertical_line(7, [2, 5, 8]);
            self.solve_by_vertical_line(8, [2, 5, 8]);
            self.update_by_vertical_lines();
            print_sudoku(self);
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn solve_by_column(&mut self, column_index: usize, horizontal_lines_indexes: [usize; 3], vertical_lines_indexes: [usize; 3]) {
        let missing_values = get_missing_values(self.columns[column_index]);
        let mut column = self.columns[column_index];
        for value in missing_values {
            if self.horizontal_lines[horizontal_lines_indexes[0]].contains(&value) {
                column[0] = 10;
                column[1] = 10;
                column[2] = 10;
            }
            if self.horizontal_lines[horizontal_lines_indexes[1]].contains(&value) {
                column[3] = 10;
                column[4] = 10;
                column[5] = 10;
            }
            if self.horizontal_lines[horizontal_lines_indexes[2]].contains(&value) {
                column[6] = 10;
                column[7] = 10;
                column[8] = 10;
            }
            if self.vertical_lines[vertical_lines_indexes[0]].contains(&value) {
                column[0] = 10;
                column[3] = 10;
                column[6] = 10;
            }
            if self.vertical_lines[vertical_lines_indexes[1]].contains(&value) {
                column[1] = 10;
                column[4] = 10;
                column[7] = 10;
            }
            if self.vertical_lines[vertical_lines_indexes[2]].contains(&value) {
                column[2] = 10;
                column[5] = 10;
                column[8] = 10;
            }
            if column.iter().filter(|&n| *n == 0).count() == 1 {
                let mut position = 0;
                for (i, v) in column.iter().enumerate() {
                    if v.clone() == 0 {
                        position = i;
                    }
                };
                self.columns[column_index][position] = value;
            }
            column = self.columns[column_index];
        }
        self.update_by_columns()
    }

    fn solve_by_horizontal_line(&mut self, line_index: usize, columns_indexes: [usize; 3]) {
        let missing_values = get_missing_values(self.horizontal_lines[line_index]);
        let mut line = self.horizontal_lines[line_index];
        for value in missing_values {
            if self.columns[columns_indexes[0]].contains(&value) {
                line[0] = 10;
                line[1] = 10;
                line[2] = 10;
            }
            if self.columns[columns_indexes[1]].contains(&value) {
                line[3] = 10;
                line[4] = 10;
                line[5] = 10;
            }
            if self.columns[columns_indexes[2]].contains(&value) {
                line[6] = 10;
                line[7] = 10;
                line[8] = 10;
            }
            for i in 0..9 {
                if self.vertical_lines[i].contains(&value) {
                    line[i] = 10;
                }
            }
            if line.iter().filter(|&n| *n == 0).count() == 1 {
                let mut position = 0;
                for (i, v) in line.iter().enumerate() {
                    if v.clone() == 0 {
                        position = i;
                    }
                };
                self.horizontal_lines[line_index][position] = value;
            }
            line = self.horizontal_lines[line_index];
        }
        self.update_by_horizontal_lines()
    }

    fn solve_by_vertical_line(&mut self, line_index: usize, columns_indexes: [usize; 3]) {
        let missing_values = get_missing_values(self.vertical_lines[line_index]);
        let mut line = self.vertical_lines[line_index];
        for value in missing_values {
            if self.columns[columns_indexes[0]].contains(&value) {
                line[0] = 10;
                line[1] = 10;
                line[2] = 10;
            }
            if self.columns[columns_indexes[1]].contains(&value) {
                line[3] = 10;
                line[4] = 10;
                line[5] = 10;
            }
            if self.columns[columns_indexes[2]].contains(&value) {
                line[6] = 10;
                line[7] = 10;
                line[8] = 10;
            }
            for i in 0..9 {
                if self.horizontal_lines[i].contains(&value) {
                    line[i] = 10;
                }
            }
            if line.iter().filter(|&n| *n == 0).count() == 1 {
                let mut position = 0;
                for (i, v) in line.iter().enumerate() {
                    if v.clone() == 0 {
                        position = i;
                    }
                };
                self.vertical_lines[line_index][position] = value;
            }
            line = self.vertical_lines[line_index];
        }
        self.update_by_vertical_lines();
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

    fn is_solved(&self) -> bool {
        for line in self.horizontal_lines.iter() {
            if line.contains(&0) {
                return false;
            }
        }
        true
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

    fn get_horizontal_lines_by_columns(&self) -> [[u32; 9]; 9] {
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

    fn get_horizontal_lines_by_vertical_lines(&self) -> [[u32; 9]; 9] {
        let mut horizontal_lines = [[0; 9]; 9];
        for line_index in 0..9 {
            for index in 0..9 {
                horizontal_lines[line_index][index] = self.vertical_lines[index][line_index];
            }
        }
        horizontal_lines
    }

    fn update_by_horizontal_lines(&mut self) {
        self.columns = self.get_columns_by_horizontal_lines();
        self.vertical_lines = self.get_vertical_lines_by_horizontal_lines();
    }

    fn update_by_vertical_lines(&mut self) {
        self.horizontal_lines = self.get_horizontal_lines_by_vertical_lines();
        self.columns = self.get_columns_by_horizontal_lines();
    }

    fn update_by_columns(&mut self) {
        self.horizontal_lines = self.get_horizontal_lines_by_columns();
        self.vertical_lines = self.get_vertical_lines_by_horizontal_lines();
    }
}

fn get_missing_values(array: [u32; 9]) -> Vec<u32> {
    let mut missing_values = Vec::new();
    missing_values.extend([1, 2, 3, 4, 5, 6, 7, 8, 9].iter());
    for value in array.iter() {
        if value.clone() != 0 {
            let index = missing_values.iter().position(|&v| v == value.clone()).unwrap();
            missing_values.remove(index);
        }
    }
    missing_values
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