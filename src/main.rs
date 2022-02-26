use std::io::{stdout, stdin, Write};
use std::fmt;
fn type_of<T>(_: &T) -> &str{
    return std::any::type_name::<T>();
}

enum BoardState  {
    Empty,
    O,
    X
}

struct TTTBoard {
    size : u32,
    board :[[BoardState; 3]; 3],
}

impl Default for TTTBoard{
    fn default() -> TTTBoard {
        TTTBoard{
            size: 9,
            board: [[BoardState::Empty,BoardState::Empty,BoardState::Empty],[BoardState::Empty,BoardState::Empty,BoardState::Empty],[BoardState::Empty,BoardState::Empty,BoardState::Empty]] 
        }
    }
}

impl fmt::Display for BoardState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            BoardState::Empty => write!(f, " "),
            BoardState::X => write!(f, "X"),
            BoardState::O => write!(f, "O"),
            _ => return Err(fmt::Error),
        }
        
    }
}

impl TTTBoard{
    fn make_move(&mut self, pos: (i8, i8), newstate: BoardState) {
        
    }
    fn print_board(&mut self){
        for row in self.board.iter().enumerate(){
            //for state in self.board[row].iter().enumerate(){
            //    println!("{}, {}", row, state);
            //}
        }
    }
}


fn main() {
    println!("Tic tac toe in Rust vs AI");
    let mut line;
    let mut board = TTTBoard{.. Default::default()};
    let bs = BoardState::O;
    println!("{}", bs);
    board.print_board();
    loop {
        match stdout().flush() {
            Ok(_) => {},
            Err(e) =>{ println!("ERROR {}", e);}
        }
        line = String::new();
        stdin().read_line(&mut line).unwrap();
        line = String::from(line.trim());
        println!("{}", line);
        if line == "q"{
            break;
        }
    }

    minimax(); // 
}


fn minimax(){
    
}
