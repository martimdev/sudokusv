mod sudoku;

use sudoku::Sudoku;

fn main() {

    let mut column1: [u32; 9] = [0; 9];
    column1[0] = 0;
    column1[1] = 0;
    column1[2] = 0;
    column1[3] = 2;
    column1[4] = 0;
    column1[5] = 0;
    column1[6] = 0;
    column1[7] = 4;
    column1[8] = 0;

    let mut column2: [u32; 9] = [0; 9];
    column2[0] = 0;
    column2[1] = 1;
    column2[2] = 2;
    column2[3] = 0;
    column2[4] = 5;
    column2[5] = 4;
    column2[6] = 0;
    column2[7] = 0;
    column2[8] = 0;

    let mut column3: [u32; 9] = [0; 9];
    column3[0] = 8;
    column3[1] = 0;
    column3[2] = 7;
    column3[3] = 0;
    column3[4] = 6;
    column3[5] = 9;
    column3[6] = 0;
    column3[7] = 0;
    column3[8] = 0;

    let mut column4: [u32; 9] = [0; 9];
    column4[0] = 3;
    column4[1] = 0;
    column4[2] = 4;
    column4[3] = 0;
    column4[4] = 0;
    column4[5] = 2;
    column4[6] = 0;
    column4[7] = 9;
    column4[8] = 0;

    let mut column5: [u32; 9] = [0; 9];
    column5[0] = 2;
    column5[1] = 9;
    column5[2] = 5;
    column5[3] = 0;
    column5[4] = 0;
    column5[5] = 0;
    column5[6] = 4;
    column5[7] = 6;
    column5[8] = 3;

    let mut column6: [u32; 9] = [0; 9];
    column6[0] = 0;
    column6[1] = 8;
    column6[2] = 0;
    column6[3] = 4;
    column6[4] = 0;
    column6[5] = 0;
    column6[6] = 2;
    column6[7] = 0;
    column6[8] = 5;

    let mut column7: [u32; 9] = [0; 9];
    column7[0] = 0;
    column7[1] = 0;
    column7[2] = 0;
    column7[3] = 1;
    column7[4] = 6;
    column7[5] = 0;
    column7[6] = 5;
    column7[7] = 0;
    column7[8] = 9;

    let mut column8: [u32; 9] = [0; 9];
    column8[0] = 0;
    column8[1] = 0;
    column8[2] = 0;
    column8[3] = 5;
    column8[4] = 3;
    column8[5] = 0;
    column8[6] = 1;
    column8[7] = 4;
    column8[8] = 0;

    let mut column9: [u32; 9] = [0; 9];
    column9[0] = 0;
    column9[1] = 7;
    column9[2] = 0;
    column9[3] = 0;
    column9[4] = 0;
    column9[5] = 4;
    column9[6] = 0;
    column9[7] = 0;
    column9[8] = 0;

    let mut columns: [[u32; 9]; 9] = [column1; 9];
    columns[0] = column1;
    columns[1] = column2;
    columns[2] = column3;
    columns[3] = column4;
    columns[4] = column5;
    columns[5] = column6;
    columns[6] = column7;
    columns[7] = column8;
    columns[8] = column9;

    let sudoku = Sudoku::new(columns);
    sudoku.solve();

}
