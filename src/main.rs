use std::fmt;
use std::fmt::Formatter;

fn main() {
    let mut sudoku: SudokuStruct = SudokuStruct { array:[
        0, 0, 1,  0, 0, 5,  0, 0, 0,
        8, 0, 0,  4, 0, 0,  0, 0, 0,
        0, 0, 0,  2, 0, 0,  1, 0, 9,

        0, 5, 0,  0, 0, 0,  0, 0, 3,
        0, 4, 0,  0, 0, 0,  0, 2, 5,
        7, 6, 8,  0, 0, 0,  0, 0, 0,

        0, 0, 4,  0, 8, 0,  0, 6, 0,
        0, 0, 0,  0, 0, 0,  0, 7, 0,
        0, 0, 0,  0, 6, 7,  5, 0, 0,
    ]};
    
    let has_solution = solve(&mut sudoku);
    if has_solution {
        println!("{}", sudoku);
    } else {
        println!("No solution found :(")
    }
}

struct SudokuStruct {
    array: [i32; 81]
}

impl fmt::Display for SudokuStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for row in 0..9 {
            for col in 0..9 {
                result.push_str(self.array[row * 9 + col].to_string().as_str());
                result.push(' ');
                if col == 2 || col == 5 {
                    result.push(' ');
                    result.push(' ');
                }
            }
            result.push('\n');
            if row == 2 || row == 5 {
                result.push('\n');
            }
        }
        return f.write_str(result.as_str());
    }
}


fn solve(sudoku: &mut SudokuStruct) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if sudoku.array[row * 9  + col] == 0 {
                for value_to_try in 1..10 {
                    sudoku.array[row * 9 + col] = value_to_try;
                    if is_legal(&sudoku) {
                        if solve(sudoku) {
                            return true;
                        }
                    }
                }
                sudoku.array[row * 9 + col] = 0;
                return false;
            };
        }
    }
    return true;
}

fn is_legal(sudoku: &SudokuStruct) -> bool {
    return no_duplicates_in_rows(&sudoku) && no_duplicates_in_columns(&sudoku) && no_duplicates_in_blocks(&sudoku);
}

fn no_duplicates_in_rows(sudoku: &SudokuStruct) -> bool {
    for row in 0..9 {
        let mut has_digit_in_row = [false, false, false, false, false, false, false, false, false,];
        for col in 0..9 {
            let value = sudoku.array[row * 9 + col];
            if value == 0 {
                continue;
            }
            if has_digit_in_row[(value - 1) as usize] {
                return false;
            } else {
                has_digit_in_row[(value - 1) as usize] = true;
            }
        }
    };
    return true;
}

fn no_duplicates_in_columns(sudoku: &SudokuStruct) -> bool {
    for col in 0..9 {
        let mut has_digit_in_row = [false, false, false, false, false, false, false, false, false,];
        for row in 0..9 {
            let value = sudoku.array[row * 9 + col];
            if value == 0 {
                continue;
            }
            if has_digit_in_row[(value - 1) as usize] {
                return false;
            } else {
                has_digit_in_row[(value - 1) as usize] = true;
            }
        }
    };
    return true;
}

fn no_duplicates_in_blocks(sudoku: &SudokuStruct) -> bool {
    for block_row in 0..3 {
        for block_col in 0..3 {
            let mut has_digit = [false, false, false, false, false, false, false, false, false,];
            for row_in_block in 0..3 {
                for col_in_block in 0..3 {
                    let row = block_row * 3 + row_in_block;
                    let col = block_col * 3 + col_in_block;
                    let value = sudoku.array[row * 9 + col];
                    if value == 0 {
                        continue;
                    }
                    if has_digit[(value - 1) as usize] {
                        return false;
                    } else {
                        has_digit[(value - 1) as usize] = true;
                    }
                }
            }
        }
    }
    return true;
}
