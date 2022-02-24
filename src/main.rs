use std::io::{stdout, stdin, Write};

fn main() {
    println!("Tic tac toe in Rust vs AI");
    let mut line;
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
    minimax();
}


fn minimax(){
    
}
