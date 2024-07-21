use std::time::Instant;

type Board = Vec<Vec<u8>>;
type Position = (usize, usize);

fn build_board_from_str(board_str: &str, width: usize, length: usize) -> Board {
    let mut board: Board = vec![vec![0; width]; length];
    let chars: Vec<char> = board_str.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        let row = i / width;
        let col = i % width;
        board[row][col] = if c == 'x' { 9 } else { c.to_digit(10).unwrap() as u8 };
    }
    board
}

fn display_moves_on_board(board: &Board, sol: &Vec<Position>) -> Board {
    let mut display_board = board.clone();
    for &(x, y) in sol {
        display_board[x][y] = if display_board[x][y] == 9 { 9 } else { 8 };
    }
    display_board
}

fn action(board: &Board, pos: Position) -> Board {
    let (x, y) = pos;
    if board[x][y] == 9 {
        panic!("Cannot action on a hole.");
    }
    let mut new_board = board.clone();
    // flip at pos
    new_board[x][y] = 1 - new_board[x][y];
    for ind in (0..x).rev() {
        // going up
        if new_board[ind][y] == 9 {
            break;
        }
        new_board[ind][y] = 1 - new_board[ind][y];
    }
    for ind in (x + 1)..new_board.len() {
        // going down
        if new_board[ind][y] == 9 {
            break;
        }
        new_board[ind][y] = 1 - new_board[ind][y];
    }
    for ind in (0..y).rev() {
        // going left
        if new_board[x][ind] == 9 {
            break;
        }
        new_board[x][ind] = 1 - new_board[x][ind];
    }
    for ind in (y + 1)..new_board[0].len() {
        // going right
        if new_board[x][ind] == 9 {
            break;
        }
        new_board[x][ind] = 1 - new_board[x][ind];
    }
    new_board
}

fn is_complete(board: &[Vec<u8>]) -> bool {
    !board.iter().flatten().any(|&b| b == 0)
}

fn is_complete_minor(board: &[Vec<u8>], minor: usize) -> bool {
    for x in 0..minor {
        for y in 0..minor {
            if board[x][y] == 0 {
                return false
            }
        }
    }
    true
}

fn solve(board: &mut Board) -> Option<Vec<Position>> {
    let mut history: Vec<usize> = Vec::new();
    let mut possible_moves: Vec<Position> = Vec::new();
    for (x, row) in board.iter().enumerate() {
        for (y, &tile) in row.iter().enumerate() {
            if tile != 9 {
                possible_moves.push((x, y));
            }
        }
    }
    possible_moves.sort_by_key(|&tup| (tup.0.min(tup.1), tup.0, tup.1));
    let mut break_points: Vec<usize> = Vec::new();
    for (ind, move_) in possible_moves.iter().enumerate() {
        if move_.0.min(move_.1) > break_points.len() {
            break_points.push(ind);
        }
    }
    let result = solve_subroutine(board, &possible_moves, 0, &mut history, &break_points);
    if let Some(history) = result {
        Some(history.iter().map(|&h| possible_moves[h]).collect())
    } else {
        None
    }
}

fn solve_subroutine(
    board: &mut Board,
    possible_moves: &Vec<Position>,
    step: usize,
    history: &mut Vec<usize>,
    break_points: &Vec<usize>,
) -> Option<Vec<usize>> {
    if is_complete(board) {
        Some(history.clone())
    } else if step >= possible_moves.len() {
        None
    } else {
        let bp = possible_moves[step].0.min(possible_moves[step].1);
        if !is_complete_minor(&board, bp) && break_points.contains(&step) {
            None
        } else {
            let mut new_board = action(board, possible_moves[step]);
            let mut new_history = history.clone();
            new_history.push(step);
            let result = solve_subroutine(&mut new_board, possible_moves, step + 1, &mut new_history, break_points);
            if result.is_some() {
                result
            } else {
                solve_subroutine(board, possible_moves, step + 1, history, break_points)
            }
        }
    }
}

fn main() {
    let length = 7;
    let width = 7;

    // board_str is left-to-right, up-to-down. 1=black, 0=white, x=hole.
    let board_str = "010011x0x0100001x011110101x0000x1011001x01x011010";

    let mut init_board = build_board_from_str(board_str, width, length);
    let start_time = Instant::now();
    let has_sol = solve(&mut init_board);
    println!("{:?}", has_sol.is_some());
    println!("{:?}", start_time.elapsed());

    if let Some(sol) = has_sol {
        let display_board = display_moves_on_board(&init_board, &sol);
        for row in display_board {
            for tile in row {
                print!("{}", if tile == 9 { "x" } else if tile == 8 { "O" } else { "." });
            }
            println!();
        }
    }
}
