use std::time::Duration;
use std::thread::sleep;
use std::io::stdout;
use std::io::Write;
use array2d::Array2D;

fn main() {
    let board = Array2D::filled_with('_', 8, 8);   

    board[(0,0)] = 'R';
    board[(0,0)]= 'N';
    board[(0,0)] = 'B';
    board[(0,0)]= 'Q';
    board[(0,0)]= 'K';
    board[5][0] = 'B';
    board[6][0] = 'N';
    board[7][0] = 'R';
    for square in board[1].iter_mut()
    {
        *square = 'P';
    }

    board[0][7] = 'r';
    board[1][7] = 'n';
    board[2][7] = 'b';
    board[3][7] = 'q';
    board[4][7] = 'k';
    board[5][7] = 'b';
    board[6][7] = 'n';
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
                    sleep(Duration::from_millis(2));
                }
            println!();
        }

        // input player move (white)
        let mut move_piece = String::new();
        println!("?move? > ");
        std::io::stdin().read_line(&mut move_piece).unwrap();
        if move_piece.chars().nth(0).unwrap() == 'x' {
            break;
        }

        let mut file_from = move_piece.chars().nth(0).unwrap() as usize;
        file_from -= 'a' as usize;
        let rank_from = (move_piece.chars().nth(1).unwrap().to_digit(10).unwrap() - 1) as usize;

        let mut file_to = move_piece.chars().nth(2).unwrap() as usize;
        file_to -= 'a' as usize;
        let rank_to = (move_piece.chars().nth(3).unwrap().to_digit(10).unwrap() - 1) as usize;
        
        board[rank_to][file_to] = board[rank_from][file_from];
        board[rank_from][file_from] = '_';

        // respond (black)

        // find all legal moves

    }
}