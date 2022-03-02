use std::io::{stdout, stdin, Write};
use std::fmt;
use std::cmp::PartialEq;

enum BoardState  {
    Empty,
    O,
    X
}


struct TTTBoard {
    board :[[BoardState; 3]; 3],
}


impl PartialEq for BoardState{
    fn eq(&self, other : &Self) -> bool{

    }
}


impl Default for TTTBoard{
    fn default() -> TTTBoard {
        TTTBoard{
            board: [[BoardState::Empty,BoardState::Empty,BoardState::Empty],[BoardState::Empty,BoardState::Empty,BoardState::Empty],[BoardState::Empty,BoardState::Empty,BoardState::Empty]] 
        }
    }
}


impl fmt::Display for TTTBoard{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        writeln!(f, "-------------");
        for row in self.board.iter(){
            write!(f, "|");
            for state in row.iter(){
                match write!(f, " {} |", state){
                    Ok(o) => {},
                    Err(e) => return Err(e),
                }
            }
            writeln!(f, "\n-------------");
        }
        return Ok(())
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
    fn make_move(&mut self, pos: (usize, usize), newstate: BoardState) {
        self.board[pos.0][pos.1] = newstate;
    }
    fn winner(&self, winstate : BoardState) -> &BoardState{
        let mut seen;
        for row in self.board.iter(){
            seen = &row[0];
            println!("SEEN {}", seen);
            for state in row.iter().enumerate(){
                if *state.1 == *seen && state.0 == self.board.len()-1{
                    return state.1;
                }
            }
        }
        
        return &BoardState::Empty
    }
}


fn main() {
    //println!("{}", BoardState::O == BoardState::O);
    return;
    println!("Tic tac toe in Rust vs UNBEATABLE AI");
    let mut line;
    let mut board = TTTBoard{.. Default::default()};
    let mut x; 
    let mut y;
    
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
        
        match newpos[0].trim().parse::<usize>(){
            Ok(o) => x = o,
            Err(e) => continue,
        }
        match newpos[1].trim().parse::<usize>(){
            Ok(o) => y = o,
            Err(e) => continue,
        }
        board.make_move((x, y), BoardState::O);
        println!("{}", board);
        if line == "q"{
            break;
        }
    }

    minimax(); // 
}


fn minimax(){
    
}
