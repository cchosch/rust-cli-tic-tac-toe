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
            if row[0] == BoardState::Empty{
                continue
            }
            seen = row[0].clone();
            for state in row.iter().enumerate(){
                if *state.1 != seen{
                    break;
                }if state.0 == self.board.len()-1{
                    return state.1;
                }
            }
        }
        for c in 0..self.board[0].len(){
            if self.board[0][c] == BoardState::Empty{
                continue
            }
            seen = self.board[0][c].clone();
            //println!("SEEN {} at 0, {}", seen,  c);
            for r in 0..self.board.len(){
                if self.board[r][c] != seen{
                    //println!("BREAKING {}", self.board[r][c]);
                    break;
                }if r == self.board.len()-1{
                    //println!("RETURNING {}", self.board[r][c]);
                    return &self.board[r][c];
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
    // initialize vars
    let mut line;
    let mut board = TTTBoard{.. Default::default()};
    let mut x; 
    let mut y;
    let mut win_yet;

    // display x, y format
    println!("| 0, 0 | 0, 1 | 0, 2 |");
    println!("----------------------");
    println!("| 1, 0 | 1, 1 | 1, 2 |");
    println!("----------------------");
    println!("| 2, 0 | 2, 1 | 2, 2 |");
    // gameloop
    loop {
        println!("Make a move x,y");
        
        match stdout().flush() {
            Ok(_) => {},
            Err(e) =>{ println!("ERROR {}", e);}
        }
        line = String::new();
        // get user input
        stdin().read_line(&mut line).unwrap();
        // cut of \n \r or any other stuff like that
        line = String::from(line.trim());
        if line == "q"{ // if user enters q, quit the game
            break;
        }
        // split the input between ,
        let newpos : Vec<&str> = line.split(",").collect();
        // if there are more than 2 sections after we split from ,
        if newpos.len() != 2{ continue; }

        // get integer value from user input
        match newpos[0].trim().parse::<usize>(){
            Ok(o) => x = o,
            Err(_) => continue,
        }
        match newpos[1].trim().parse::<usize>(){
            Ok(o) => y = o,
            Err(_) => continue,
        }

        // make given move as the player, (O)
        board.make_move((x, y), BoardState::O);
        // check if there is a winner
        win_yet = board.winner();
        match win_yet {
            BoardState::Empty => { // if board.winner() return Empty
                if board.full() { // if the board is full
                    println!("TIE");
                    // break the gameloop
                    break;
                }
            },
            _ => { // any other state
                if *win_yet == BoardState::O{
                    println!("{}YOU WON\n", board);
                }else {
                    println!("{}You Lost\n", board);
                }
                // break the gameloop
                break;
            },
        }
        // make computer move
        minimax(&mut board);
        // check winner again
        win_yet = board.winner();
        match win_yet {
            BoardState::Empty => {
                if board.full() {
                    println!("TIE");
                    break;
                }
            },
            _ => {
                if *win_yet == BoardState::O{
                    println!("{}YOU WON\n", board);
                }else {
                    println!("{}You Lost\n", board);
                }
                break;
            },
            
        }
        println!("{}", board);
    }

}


fn minimax(board : &mut TTTBoard){
    let mut res;
    let mut best = (0, (0, 0));
    for r in 0..board.board.len(){
        for s in 0..board.board[r].len(){
            if board.board[r][s] != BoardState::Empty{
                continue
            }
            board.make_move((r, s), BoardState::X);
            res = eval(board, BoardState::O, false);
            if res > best.0{
                best = (res, (r, s));
                if res == 2{
                    return;
                }
            }
            board.make_move((r, s), BoardState::Empty);
        }
    }
    board.make_move((best.1.0, best.1.1), BoardState::X);
    return
}


fn eval(board : &mut TTTBoard, cplayer : BoardState, max :bool) -> i8{
    /*
     * lose = 0
     * tie = 1
     * win = 2
     */
    // check winner
    match board.winner(){
        BoardState::Empty => {if board.full(){ return 1; }},
        s => {
            if max { // if we are evaluating maximizing player
                if *s == cplayer{
                    return 2; // maximizing player won and we return 2
                }
                return 0; // maximizing player lost and we return 0;
            }
            // if we are evaluating minimizing player
            if *s == cplayer{ 
                return 0; // minimizing player won and we return 0;
            }
            return 2; // maximining player lost and we return 2
        },
    }
    //          (best, (x, y))
    let mut optimal: i8 = 0; // initialize optimal variable
    if !max{ // if we are minimizing set the optimal to 2
        optimal = 2;
    }
    let mut result: i8;
    // loop through board
    for row in 0..board.board.len(){
        for column in 0..board.board[0].len(){
            // if we can make a move on (row, column)
            if board.board[row][column] == BoardState::Empty{
                // make the move that we are going to evaluate
                board.make_move((row, column), cplayer.clone());
                // evaluate
                result = eval(board, cplayer.opposite(), !max);
                // reverse move
                board.make_move((row, column), BoardState::Empty);
                if max{ // if we are maximizing player
                    if result > optimal{ // maximize optimal
                        optimal = result;
                    }
                }else{ // if we are minimizing player
                    if result < optimal{ // minimize optimal
                        optimal = result;
                    }
                }
            }
        }
    }
    return optimal // return 
}

