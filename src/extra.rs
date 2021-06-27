use std::io;
use std::io::Write;

pub fn hidden_input() -> String{
    io::stdout().flush().unwrap();
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .ok()
        .expect("failed to read line");
    let answer = answer.trim_end();
    let answer = answer.to_string();
    answer
}