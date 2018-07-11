extern crate sudokusv;

use sudokusv::Sudoku;

fn main() {
    let horizontal_lines = [
        [5, 0, 0, 0, 4, 0, 0, 7, 0],
        [0, 0, 0, 0, 6, 0, 2, 4, 0],
        [0, 4, 1, 0, 0, 8, 0, 3, 5],
        [0, 5, 0, 0, 8, 9, 0, 0, 4],
        [3, 0, 0, 2, 0, 5, 0, 0, 6],
        [9, 0, 0, 4, 1, 0, 0, 8, 0],
        [6, 7, 0, 9, 0, 0, 8, 1, 0],
        [0, 1, 2, 0, 5, 0, 0, 0, 0],
        [0, 3, 0, 0, 2, 0, 0, 0, 7]
    ];

    let sudokusv = Sudoku::new(horizontal_lines);
    sudokusv.solve();
}
