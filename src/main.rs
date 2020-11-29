mod chess_board;
use chess_board::board::*;
use std::io;

fn main() {
    let mut chess_board = Board::init();
    chess_board.print_board();
    let mut current_piece = Piece::get_circle();
    let mut current_index: Index;
    let mut counter = 0;
    loop {
        println!("Your turn {}: ", match current_piece {
            Piece::Circle(v) => v,
            Piece::Fork(v) => v
        });
        current_index = match read_next_index() {
            Some(index) => index,
            None => {
                println!("Please type correct index like: 0,0");
                continue;
            }
        };

        if !chess_board.set(current_piece, &current_index) {
            continue;
        }
        chess_board.print_board();

        //  unit win or draw
        if counter >= 5 {
            
        }
        
        match current_piece {
            Piece::Circle(_) => {
                //  used
                current_piece = Piece::get_fork()
            },
            Piece::Fork(_) => {
                //  used
                current_piece = Piece::get_circle();
            }
        }
        //  used
        counter +=1;
        break;
    }
    
    
}

fn read_next_index() -> Option<Index> {
    let mut index = String::new();
    match io::stdin().read_line(&mut index) {
        Ok(_) => {
            let v: Vec<&str> = index.split(',').collect();
            if v.len() != 2 {
                return None;
            }
            let first: usize;
            match v[0].trim().parse() {
                Ok(v) => first = v,
                Err(_) => return None
            }
            let second: usize;
            match v[1].trim().parse() {
                Ok(v) => second = v,
                Err(_) => return None
            }
            Some(Index{ 0: first, 1: second })
        },
        Err(_) => {
            None
        }
    }
}