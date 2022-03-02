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
        let f: i8;
        match self{
            BoardState::O => f = 1,
            BoardState::X => f = 2,
            BoardState::Empty => f = 0,
        }match other{
            BoardState::O => if f == 1 {return true},
            BoardState::X => if f == 2 {return true},
            BoardState::Empty => if f == 0 {return true},
        }
        return false
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
        writeln!(f, "-------------").ok();
        for row in self.board.iter(){
            write!(f, "|").ok();
            for state in row.iter(){
                write!(f, " {} |", state).ok();
            }
            writeln!(f, "\n-------------").ok();
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
        }
        
    }
}


impl TTTBoard{
    fn make_move(&mut self, pos: (usize, usize), newstate: BoardState) {
        self.board[pos.0][pos.1] = newstate;
    }
    fn full(&self) -> bool{
        for row in self.board.iter(){
            for s in row.iter(){
                if *s == BoardState::Empty{
                    return false
                }
            }
        }
        return true
    }
    fn winner(&self) -> &BoardState{
        let mut seen;
        for row in self.board.iter(){
            seen = &row[0];
            for state in row.iter().enumerate(){
                if *state.1 == *seen && state.0 == self.board.len()-1{
                    return state.1;
                }
            }
        }
        
        return &BoardState::Empty;
    }
}


fn main() {
    println!("Tic tac toe in Rust vs UNBEATABLE AI");
    let mut line;
    let mut board = TTTBoard{.. Default::default()};
    let mut x; 
    let mut y;
    let mut win_yet = &BoardState::Empty;
    
    loop {
        println!("Make a move x,y");
        match stdout().flush() {
            Ok(_) => {},
            Err(e) =>{ println!("ERROR {}", e);}
        }
        line = String::new();
        stdin().read_line(&mut line).unwrap();
        line = String::from(line.trim());
        let newpos : Vec<&str> = line.split(",").collect();
        if newpos.len() != 2{ continue; }
        
        match newpos[0].trim().parse::<usize>(){
            Ok(o) => x = o,
            Err(_) => continue,
        }
        match newpos[1].trim().parse::<usize>(){
            Ok(o) => y = o,
            Err(_) => continue,
        }
        
        board.make_move((x, y), BoardState::O);
        win_yet = board.winner();

        match *win_yet {
            BoardState::Empty => {},
            BoardState::O | BoardState::X => {
                if *win_yet == BoardState::O{
                    println!("YOU WON")
                }
                else if !(*win_yet == BoardState::O){
                    println!("You Lost")
                }
            },
            
        }
        if *win_yet != BoardState::Empty {
            if board.full() {
                println!("TIE");
                break;
            }
        }
        println!("{}", board);
        if line == "q"{
            break;
        }
    }

    minimax(); // 
}


fn minimax(&mut board:TTTBoard){
    
}
