use std::time::Duration;
use std::thread::sleep;
use std::io::stdout;
use std::io::Write;

fn main() {

    let mut board: [[u8; 8]; 8] = [[95; 8]; 8];

    board[0][0] = 'R' as u8;
    board[0][1] = 'N' as u8;
    board[0][2] = 'B' as u8;
    board[0][3] = 'Q' as u8;
    board[0][4] = 'K' as u8;
    board[0][5] = 'B' as u8;
    board[0][6] = 'N' as u8;
    board[0][7] = 'R' as u8;
    for square in board[1].iter_mut()
    {
        *square = 'P' as u8;
    }

    board[7][0] = 'r' as u8;
    board[7][1] = 'n' as u8;
    board[7][2] = 'b' as u8;
    board[7][3] = 'q' as u8;
    board[7][4] = 'k' as u8;
    board[7][5] = 'b' as u8;
    board[7][6] = 'n' as u8;
    board[7][7] = 'r' as u8;
    for square in board[6].iter_mut()
    {
        *square = 'p' as u8;
    }
    
    loop {
        for rank in board.iter().rev() {
            for square in rank {
                    print!("{} ", *square as char);
                    stdout().flush().unwrap();
                    sleep(Duration::from_millis(5));
                }
            println!();
        }

        let mut move_piece = String::new();
        println!("?move? > ");
        std::io::stdin().read_line(&mut move_piece).unwrap();
        let mut file_from = move_piece.chars().nth(0).unwrap() as usize;
        file_from -= 'a' as usize;
        let rank_from = (move_piece.chars().nth(1).unwrap().to_digit(10).unwrap() - 1) as usize;

        let mut file_to = move_piece.chars().nth(2).unwrap() as usize;
        file_to -= 'a' as usize;
        let rank_to = (move_piece.chars().nth(3).unwrap().to_digit(10).unwrap() - 1) as usize;
        
        board[rank_to][file_to] = board[rank_from][file_from];
        board[rank_from][file_from] = 95;
    }
}