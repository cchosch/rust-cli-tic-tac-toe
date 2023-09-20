use std::cmp::PartialEq;
use std::io::{stdout, stdin, Write};
use std::fmt::{self, Display};

#[derive(PartialEq, Eq, Clone, Debug)]
enum BoardState  {
    Empty,
    O,
    X
}

#[derive(Clone)]
struct TTTBoard {
    board :[[BoardState; 3]; 3],
}

impl Default for TTTBoard{
    fn default() -> TTTBoard {
        TTTBoard{
            board: [
                [BoardState::Empty,BoardState::Empty,BoardState::Empty],
                [BoardState::Empty,BoardState::Empty,BoardState::Empty],
                [BoardState::Empty,BoardState::Empty,BoardState::Empty]
            ]
        }
    }
}


impl Display for TTTBoard{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        writeln!(f, "-------------")?;
        for (i, row) in self.board.iter().enumerate(){
            write!(f, "|")?;
            for state in row.iter(){
                write!(f, " {} |", state)?;
            }
            write!(f, "\n-------------{}", if i == self.board.len()-1 {""} else { "\n" })?;
        }
        return Ok(())
    }
}


impl Display for BoardState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            BoardState::Empty => write!(f, " "),
            BoardState::X => write!(f, "X"),
            BoardState::O => write!(f, "O"),
        }
    }
}

impl BoardState {
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
                if s == &BoardState::Empty{
                    return false
                }
            }
        }
        return true
    }

    fn winner(&self) -> Option<BoardState>{
        let mut seen;
        // left to right
        for row in self.board.iter() {
            if row[0] == BoardState::Empty{
                continue
            }
            seen = row[0].clone();
            for state in row.iter().enumerate(){
                if *state.1 != seen{
                    break;
                }if state.0 == self.board.len()-1{
                    return Some(state.1.clone());
                }
            }
        }
        // top to bottom
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
                    return Some(self.board[r][c].clone());
                }
            }
        }
        if self.board[1][1] != BoardState::Empty {
            // diagonal
            if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2]{
                return Some(self.board[0][0].clone());
            }else if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0]{
                return Some(self.board[1][1].clone());
            }
        }
            
        return if self.full() { Some(BoardState::Empty) } else {None};
    }
}


fn main() {
    println!("Tic tac toe in Rust vs UNBEATABLE AI");
    // initialize vars
    let mut line: String;
    let mut board = TTTBoard::default();
    let mut x: usize; 
    let mut y: usize;
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

        if board.board[x][y] != BoardState::Empty {
            println!("Invalid move, try again");
            continue;
        }
        // make given move as the player, (O)
        board.make_move((x, y), BoardState::O);

        // check if there is a winner
        win_yet = board.winner();
        match win_yet {
            Some(val) => {
                println!("{}", match val {
                    BoardState::Empty => "TIE".to_string(),
                    BoardState::O => format!("{}\nYOU WON", board),
                    BoardState::X => format!("{}\nYOU Lost :(", board),
                });
                break
            }
            _ => {}
        }
        // make computer move
        minimax(&mut board);
        // check winner again
        win_yet = board.winner();
        match win_yet {
            Some(val) => {
                println!("{}", match val {
                    BoardState::Empty => "TIE".to_string(),
                    BoardState::O => format!("{}\nYOU WON", board),
                    BoardState::X => format!("{}\nYOU Lost :(", board),
                });
                break
            }
            _ => {}
        }
        println!("{}", board);
    }

}


fn minimax(board : &mut TTTBoard){
    if board.full() {
        return;
    }
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
        Some(s) => {
            if s == BoardState::Empty {
                return 1;
            }

            if max { // if we are evaluating maximizing player
                if s == cplayer{
                    return 2; // maximizing player won and we return 2
                }
                return 0; // maximizing player lost and we return 0;
            }
            // if we are evaluating minimizing player
            if s == cplayer{ 
                return 0; // minimizing player won and we return 0;
            }
            return 2; // maximining player lost and we return 2
        }
        _=>{}
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
                if max { // if we are maximizing player
                    optimal = optimal.max(result);
                } else { // if we are minimizing player
                    optimal = optimal.min(result);
                }
            }
        }
    }
    return optimal // return 
}
