use std::io::{stdout, stdin, Write};


struct TTTBoard {
    size : i64
}

impl Default for TTTBoard{
    fn default() -> TTTBoard {
        TTTBoard{
            size:9
        }
    }
}

impl TTTBoard{
}


fn main() {
    println!("Tic tac toe in Rust vs AI");
    let mut line;
    let board = TTTBoard{.. Default::default()};
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
