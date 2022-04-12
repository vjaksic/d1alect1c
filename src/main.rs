use std::time::Duration;
use std::thread::sleep;
use std::io::stdout;
use std::io::Write;

fn main() {

    let mut board: [[char; 8]; 8] = [['_'; 8]; 8];

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