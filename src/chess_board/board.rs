/// index of piece
pub struct Index(pub usize, pub usize);

pub enum ChessResult {
    CircleWin,
    ForkWin,
    Draw
}

#[derive(Copy, Clone)]
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
        let mut is_draw = false;
        'outer: for i in 0..3 {
            for j in 0..3 {
                if let None = self.value[i][j] {
                    is_draw = true;
                    break 'outer;
                }
            }
        }
        if is_draw {
            return Some(ChessResult::Draw);
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