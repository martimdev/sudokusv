extern crate sudokusv;

use sudokusv::Sudoku;

fn main() {
    let horizontal_lines = [
        //[5, 0, 0, 0, 4, 0, 0, 7, 0],
        //[0, 0, 0, 0, 6, 0, 2, 4, 0],
        //[0, 4, 1, 0, 0, 8, 0, 3, 5],
        //[0, 5, 0, 0, 8, 9, 0, 0, 4],
        //[3, 0, 0, 2, 0, 5, 0, 0, 6],
        //[9, 0, 0, 4, 1, 0, 0, 8, 0],
        //[6, 7, 0, 9, 0, 0, 8, 1, 0],
        //[0, 1, 2, 0, 5, 0, 0, 0, 0],
        //[0, 3, 0, 0, 2, 0, 0, 0, 7]

        [7, 3, 0, 0, 1, 6, 0, 0, 2],
        [0, 6, 0, 3, 0, 0, 7, 1, 5],
        [4, 0, 1, 5, 7, 0, 0, 6, 0],
        [0, 0, 7, 2, 9, 5, 6, 0, 0],
        [5, 8, 2, 0, 0, 1, 0, 4, 0],
        [0, 1, 0, 4, 0, 0, 5, 2, 7],
        [8, 0, 0, 0, 0, 4, 1, 7, 6],
        [0, 0, 9, 1, 5, 7, 0, 0, 4],
        [1, 7, 4, 0, 8, 0, 2, 0, 0]
    ];

    let mut sudokusv = Sudoku::new(horizontal_lines);
    sudokusv.solve();
}
