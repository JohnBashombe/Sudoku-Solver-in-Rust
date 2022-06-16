pub const GRID_SIZE: u8 = 9;

fn main() {
    let mut board: [[u8; 9]; 9] = [
        [8u8, 0u8, 1u8, 3u8, 5u8, 6u8, 0u8, 0u8, 0u8],
        [5u8, 0u8, 6u8, 0u8, 0u8, 7u8, 8u8, 0u8, 3u8],
        [3u8, 0u8, 9u8, 8u8, 0u8, 0u8, 0u8, 0u8, 7u8],
        [9u8, 1u8, 4u8, 6u8, 8u8, 3u8, 0u8, 0u8, 0u8],
        [0u8, 8u8, 0u8, 5u8, 1u8, 0u8, 0u8, 0u8, 4u8],
        [0u8, 0u8, 3u8, 2u8, 0u8, 0u8, 1u8, 9u8, 8u8],
        [0u8, 9u8, 0u8, 1u8, 4u8, 2u8, 5u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 6u8, 8u8, 4u8, 7u8, 2u8],
        [0u8, 6u8, 0u8, 7u8, 3u8, 0u8, 0u8, 0u8, 1u8],
    ];

    println!("Sudoku To Solved...");
    print_board(&board);

    println!();
    println!("Solving...");

    if solve_board(&mut board) {
        println!("Solved...");
        println!();
    } else {
        println!("Unsolvable");
        println!();
    }

    print_board(&board);
}

fn print_board(board: &[[u8; 9]; 9]) {
    for row in 0..GRID_SIZE {
        if row % 3 == 0 && row != 0 {
            println!("-----------");
        }

        for column in 0..GRID_SIZE {
            if column % 3 == 0 && column != 0 {
                print!("|");
            }

            print!("{:?}", &board[row as usize][column as usize]);
        }
        println!();
    }
}

fn is_number_in_row(board: &[[u8; 9]; 9], number: u8, row: u8) -> bool {
    for i in 0..GRID_SIZE {
        if &board[row as usize][i as usize] == &number {
            return true;
        }
    }
    return false;
}

fn is_number_in_column(board: &[[u8; 9]; 9], number: u8, column: u8) -> bool {
    for i in 0..GRID_SIZE {
        if &board[i as usize][column as usize] == &number {
            return true;
        }
    }
    return false;
}

fn is_number_in_box(board: &[[u8; 9]; 9], number: u8, row: u8, column: u8) -> bool {
    let local_box_row: u8 = &row - &row % 3;
    let local_box_column: u8 = &column - &column % 3;

    for i in local_box_row..local_box_row + 3 {
        for j in local_box_column..local_box_column + 3 {
            if &board[i as usize][j as usize] == &number {
                return true;
            }
        }
    }

    return false;
}

fn is_valid_placement(board: &mut [[u8; 9]; 9], number: u8, row: u8, column: u8) -> bool {
    return !is_number_in_row(board, number, row)
        && !is_number_in_column(board, number, column)
        && !is_number_in_box(board, number, row, column);
}

pub fn solve_board(board: &mut [[u8; 9]; 9]) -> bool {
    for row in 0..GRID_SIZE {
        for column in 0..GRID_SIZE {
            let x: usize = row as usize;
            let y: usize = column as usize;

            if &board[x][y] == &0u8 {
                for number_to_try in 1..=GRID_SIZE {
                    if is_valid_placement(board, number_to_try, row, column) {
                        board[x][y] = number_to_try;

                        if solve_board(board) {
                            return true;
                        } else {
                            board[x][y] = 0u8;
                        }
                    }
                }
                return false;
            }
        }
    }
    return true;
}
