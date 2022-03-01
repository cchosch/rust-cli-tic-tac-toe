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
}

impl fmt::Display for TTTBoard{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        writeln!(f, "-------------");
        for row in self.board.iter(){
            write!(f, "|");
            for state in row.iter(){
                write!(f, " {} |", state);
            }
            writeln!(f, "\n-------------");
        }
        return Ok(())
    }
}

fn main() {
    println!("Tic tac toe in Rust vs UNBEATABLE AI");
    let mut line;
    let mut board = TTTBoard{.. Default::default()};
    
    loop {
        println!("Make a move x,y");
        match stdout().flush() {
            Ok(_) => {},
            Err(e) =>{ println!("ERROR {}", e);}
        }
        line = String::new();
        stdin().read_line(&mut line).unwrap();
        line = String::from(line.trim());
        let mut newpos : Vec<&str> = line.split(",").collect();
        if newpos.len() != 2{ continue; }
        newpos[0] = newpos[0].trim();
        newpos[1] = newpos[1].trim();
        let x; 
        let y;
        match newpos[0].parse::<i32>(){
            Ok(o) => x = o,
            Err(e) => continue,
        }
        match newpos[1].parse::<i32>(){
            Ok(o) => y = o,
            Err(e) => continue,
        }
        println!("({}, {})", x, y);
        println!("{}", board);
        if line == "q"{
            break;
        }
    }

    minimax(); // 
}


fn minimax(){
    
}
