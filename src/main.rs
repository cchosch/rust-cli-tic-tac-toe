#![recursion_limit= "2000000"]
use std::cmp::PartialEq;
use std::io::{stdout, stdin, Write};
use std::fmt;


enum BoardState  {
    Empty,
    O,
    X
}

struct TTTBoard {
    board :[[BoardState; 3]; 3],
}

impl Clone for BoardState{
    fn clone(&self) -> Self{
        match self{
            BoardState::X => return BoardState::X,
            BoardState::O => return BoardState::O,
            BoardState::Empty => return BoardState::Empty,
        }
    }
}

impl Clone for TTTBoard{
    fn clone(&self) -> Self{
        let mut newboard = TTTBoard{..Default::default()};
        for row in self.board.iter().enumerate(){
            for state in row.1.iter().enumerate(){
                match state.1{
                    a => newboard.board[row.0][state.0] = a.clone(),
                }
            }
        }
        return newboard;
    }
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

impl BoardState{
    fn opposite(&self) -> BoardState{
        match self{
            BoardState::O => return BoardState::X,
            BoardState::X => return BoardState::O,
            BoardState::Empty => return BoardState::Empty,
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
            seen = row[0].clone();
            for state in row.iter().enumerate(){
                if *state.1 == seen && state.0 == self.board.len()-1{
                    return state.1;
                }if *state.1 != seen{
                    break;
                }
            }
        }
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2]{
            return &self.board[0][0];
        }else if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0]{
            return &self.board[1][1];
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
    let mut win_yet;
    
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
                    println!("YOU WON");
                }else if !(*win_yet == BoardState::O){
                    println!("You Lost");
                }
                break;
            },
            
        }
        if *win_yet != BoardState::Empty {
            if board.full() {
                println!("TIE");
                break;
            }
        }
        minimax(&board);
        println!("{}", board);
        if line == "q"{
            break;
        }
    }

}


fn minimax(board : &TTTBoard){
    let mut boardc = board.clone();
    
    eval(&mut boardc, 0, BoardState::X);
}


fn eval(board : &mut TTTBoard, levels : usize, cstate : BoardState ) -> i8{
    println!("{}", board);
    /*
     * lose = 0
     * tie = 1
     * win = 2
     */
    
    match board.winner(){
        BoardState::Empty => {if board.full(){ return 1; }},
        s => {if *s == cstate {return 2;} return 0;}
    }
    //          (best, (x, y))
    let mut best = (0, (0, 0));
    let mut avail : Vec<(usize, usize)> = Vec::new();
    for row in 0..board.board.len(){
        for state in 0..board.board[0].len(){ //(index, object)
            if board.board[row][state] == BoardState::Empty{
                avail.push((row, state));
            }
        }
    }
    for spot in avail.iter(){
        board.make_move((spot.0,spot.1), cstate.clone());
        let result = eval(board, levels+1, cstate.opposite());
        board.make_move((spot.0,spot.1), BoardState::Empty);
        match cstate.opposite(){
            BoardState::X =>{ // ai
                if result > best.0{
                    best = (result, (spot.0, spot.1));
                }if result == 0{ // if ai lost
                    return 0;
                }
            },
            BoardState::O =>{ // player
                if result == 2{ // if player won
                    return 0;
                }
            },
            _ => {}
        } 
    }
    return 0
}

