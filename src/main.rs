//use std::time::Duration;
//use std::thread::sleep;
use std::io::stdout;
use std::io::Write;
use std::usize;
use rand::seq::SliceRandom;

type Board = [[char; 8]; 8];

fn main() {

    let mut board: Board = [['_'; 8]; 8];

    board[0][0] = 'R';
    board[0][1] = 'N';
    board[0][2] = 'B';
    board[0][3] = 'Q';
    board[0][4] = 'K';
    board[0][5] = 'B';
    board[0][6] = 'N';
    board[0][7] = 'R';
    for square in board[1].iter_mut()
    {
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
    for square in board[6].iter_mut()
    {
        *square = 'p';
    }
    
    loop {
        // draw board
        for rank in board.iter().rev() {
            for square in rank {
                    print!("{} ", *square);
                    stdout().flush().unwrap();
                    // sleep(Duration::from_millis(1));
                }
            println!();
        }

        // input player move
        let mut move_piece = String::new();
        println!("?move? > ");
        std::io::stdin().read_line(&mut move_piece).unwrap();
        if move_piece.chars().nth(0).unwrap() == 'x' {
            break;
        } else if move_piece.chars().count() >=4 {          
            let mut f_src = move_piece.chars().nth(0).unwrap() as usize;
            f_src -= 'a' as usize;
            let r_src = (move_piece.chars().nth(1).unwrap().to_digit(10).unwrap() - 1) as usize;

            let mut f_dest = move_piece.chars().nth(2).unwrap() as usize;
            f_dest -= 'a' as usize;
            let r_dest = (move_piece.chars().nth(3).unwrap().to_digit(10).unwrap() - 1) as usize;

            move_from_to(&mut board, r_src, f_src, r_dest, f_dest)
        }

        // find out all possible responses
        let mut possile_moves = Vec::new();

        for r in 0..8 {
            for f in 0..8 {
                let piece = board[r][f];
                if piece == 'p'
                {
                    if r >= 1 && board[r - 1][f] == '_'
                    {
                        possile_moves.push((r,f,r -1,f));
                    }
                }
            }
        }        

        if possile_moves.is_empty(){
            println!("cant move shit");
            continue;
        }

        let mut rng = rand::thread_rng();
        let possible_move = possile_moves.choose(&mut rng).unwrap();

        move_from_to(&mut board, possible_move.0, possible_move.1, possible_move.2, possible_move.3);
    }

    fn move_from_to(board: &mut Board, r_src: usize, f_src: usize, r_dest: usize, f_dest: usize)
    {
        board[r_dest][f_dest] = board[r_src][f_src];
        board[r_src][f_src] = '_';
    }
}