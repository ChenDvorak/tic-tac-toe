/// index of piece
pub struct Index(pub usize, pub usize);

pub enum ChessResult {
    CircleWin,
    ForkWin
}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum Piece {
    Circle(char),
    Fork(char)
}

impl Piece {
    pub fn get_circle() -> Piece {
        Piece::Circle('〇')
    }

    pub fn get_fork() -> Piece {
        Piece::Fork('×')
    }
}

pub struct Board {
    value: [[Option<Piece>; 3]; 3]
}

impl Board {
    pub fn init() -> Board {
        Board {
            value: [
                [None, None, None], 
                [None, None, None], 
                [None, None, None]
            ]
        }
    }

    pub fn set(&mut self, cur_piece: Piece, cur_index: &Index) -> bool {
        return match self.value[cur_index.0][cur_index.1] {
            Some(_) => false,
            None => {
                self.value[cur_index.0][cur_index.1] = Some(cur_piece);
                true
            }
        }
    }

    pub fn print_board(&self) {
        print!("\x1b[2J\x1b[H");
        println!("     0   1   2");
        println!("   _____________");
        println!("0  | {} | {} | {} |", 
            print_piece(&self.value[0][0]),
            print_piece(&self.value[0][1]),
            print_piece(&self.value[0][2]));
        println!("   -------------");
        println!("1  | {} | {} | {} |", 
            print_piece(&self.value[1][0]),
            print_piece(&self.value[1][1]),
            print_piece(&self.value[1][2]));
        println!("   -------------");
        println!("2  | {} | {} | {} |", 
            print_piece(&self.value[2][0]),
            print_piece(&self.value[2][1]),
            print_piece(&self.value[2][2]));
        println!("   -------------");
    }

    pub fn check_win(&self) -> Option<ChessResult> {
        for i in 0..3 {
            //  check row
            if self.value[i][0] != None 
            && self.value[i][0] == self.value[i][1] && self.value[i][1] == self.value[i][2] {
                if let Some(piece) = self.value[i][0] {
                    match piece {
                        Piece::Circle(_) => return Some(ChessResult::CircleWin),
                        Piece::Fork(_) => return Some(ChessResult::ForkWin)
                    }
                }
            }
            //  check column
            if self.value[0][i] != None 
            && self.value[0][i] == self.value[1][i] && self.value[1][i] == self.value[2][i] {
                if let Some(piece) = self.value[0][i] {
                    match piece {
                        Piece::Circle(_) => return Some(ChessResult::CircleWin),
                        Piece::Fork(_) => return Some(ChessResult::ForkWin)
                    }
                }
            }
            // diagonal
            if self.value[0][0] != None
            && self.value[0][0] == self.value[1][1] && self.value[1][1] == self.value[2][2] {
                if let Some(piece) = self.value[0][0] {
                    match piece {
                        Piece::Circle(_) => return Some(ChessResult::CircleWin),
                        Piece::Fork(_) => return Some(ChessResult::ForkWin)
                    }
                }
            }
            if self.value[0][2] != None
            && self.value[0][2] == self.value[1][1] && self.value[1][1] == self.value[2][0] {
                if let Some(piece) = self.value[0][2] {
                    match piece {
                        Piece::Circle(_) => return Some(ChessResult::CircleWin),
                        Piece::Fork(_) => return Some(ChessResult::ForkWin)
                    }
                }
            }
        }
        
        None
    }
}

fn print_piece(piece: &Option<Piece>) -> char {
    if let Some(p) = piece {
        match p {
            Piece::Circle(v) => v.clone(),
            Piece::Fork(v) => v.clone()
        }
    } else {
        ' '
    }
}
