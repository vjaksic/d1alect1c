//use std::time::Duration;
//use std::thread::sleep;
use rand::seq::SliceRandom;
use std::io::stdout;
use std::io::Write;
use std::usize;

type Board = [[char; 8]; 8];

fn initialize_board() -> Board {
    let mut board: Board = [['_'; 8]; 8];

    board[0][0] = 'R';
    board[0][1] = 'N';
    board[0][2] = 'B';
    board[0][3] = 'Q';
    board[0][4] = 'K';
    board[0][5] = 'B';
    board[0][6] = 'N';
    board[0][7] = 'R';
    for square in board[1].iter_mut() {
        *square = 'P';
    }

    board[7][0] = 'r';
    board[7][1] = 'n';
    board[7][2] = 'b';
    board[7][3] = 'q';
    board[7][4] = 'k';
    board[7][5] = 'b';
    board[7][6] = 'n';
    board[7][7] = 'r';
    for square in board[6].iter_mut() {
        *square = 'p';
    }

    return board;
}

fn print_board(board: &mut Board) {
    for rank in board.iter().rev() {
        for square in rank {
            print!("{} ", *square);
            stdout().flush().unwrap();
            // sleep(Duration::from_millis(1));
        }
        println!();
    }
}

fn main() {
    let mut board = initialize_board();

    loop {
        print_board(&mut board);

        // input player move
        let mut move_piece = String::new();
        println!("?move? > ");
        std::io::stdin().read_line(&mut move_piece).unwrap();
        if move_piece.chars().nth(0).unwrap() == 'x' {
            break;
        } else if move_piece.trim_end().chars().count() == 4 {
            let mut f_src = move_piece.chars().nth(0).unwrap() as usize;
            f_src -= 'a' as usize;
            let r_src = (move_piece.chars().nth(1).unwrap().to_digit(10).unwrap() - 1) as usize;

            let mut f_dest = move_piece.chars().nth(2).unwrap() as usize;
            f_dest -= 'a' as usize;
            let r_dest = (move_piece.chars().nth(3).unwrap().to_digit(10).unwrap() - 1) as usize;

            move_from_to(&mut board, r_src, f_src, r_dest, f_dest)
        }

        // find out all possible responses
        let mut possible_moves = Vec::new();
        let rotator = vec![
            (1i8, 0i8),
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, 1),
            (-1, 1),
            (-1, -1),
            (1, -1),
        ];
        let rotator_knight = vec![
            (2i8, 1i8),
            (1, 2),
            (-1, 2),
            (-2, 1),
            (-2, -1),
            (-1, -2),
            (1, -2),
            (2, -1),
        ];

        for r in 0..8 {
            for f in 0..8 {
                let piece = board[r][f];
                if piece == 'p' {
                    if (r >= 1) && (board[r - 1][f] == '_') {
                        possible_moves.push((r, f, r - 1, f));
                        if r == 6 && board[r - 2][f] == '_' {
                            possible_moves.push((r, f, r - 2, f));
                        }
                    }
                } else if piece == 'k' {
                    for direction in rotator.iter() {
                        let dest_r = (r as i8 + direction.0) as usize;
                        let dest_f = (f as i8 + direction.1) as usize;
                        let rank_option = board.get_mut(dest_r);
                        match rank_option {
                            Some(rank) => {
                                let dest_option = rank.get_mut(dest_f);
                                match dest_option {
                                    Some(dest) => {
                                        if is_empty(*dest) || is_white(*dest) {
                                            possible_moves.push((r, f, dest_r, dest_f));
                                        }
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }
                    }
                } else if piece == 'n' {
                    for direction in rotator_knight.iter() {
                        let dest_r = (r as i8 + direction.0) as usize;
                        let dest_f = (f as i8 + direction.1) as usize;
                        let rank_option = board.get_mut(dest_r);
                        match rank_option {
                            Some(rank) => {
                                let dest_option = rank.get_mut(dest_f);
                                match dest_option {
                                    Some(dest) => {
                                        if is_empty(*dest) || is_white(*dest) {
                                            possible_moves.push((r, f, dest_r, dest_f));
                                        }
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        if possible_moves.is_empty() {
            println!("cant move shit\n");
            continue;
        }

        let mut rng = rand::thread_rng();
        let possible_move = possible_moves.choose(&mut rng).unwrap();

        move_from_to(
            &mut board,
            possible_move.0,
            possible_move.1,
            possible_move.2,
            possible_move.3,
        );
    }
}

fn move_from_to(board: &mut Board, r_src: usize, f_src: usize, r_dest: usize, f_dest: usize) {
    board[r_dest][f_dest] = board[r_src][f_src];
    board[r_src][f_src] = '_';
}

fn is_empty(square: char) -> bool {
    if square == '_' {
        return true;
    } else {
        return false;
    }
}

fn is_white(square: char) -> bool {
    if is_empty(square) {
        return false;
    }
    return square.is_uppercase();
}
