#[path = "extra.rs"] mod extra;
use std::fs::File;
use std::process::exit;
use serde_json::de::from_reader;
use colored::Colorize;

pub fn eula_checker() {
    // Opens the Eula and gets the eula value
    let file = File::open("eula.json")
        .expect("The file should open in read only");
    let json: serde_json::Value = from_reader(file)
        .expect("File should be proper json");
    let eula = json.get("eula")
        .expect("Eula was not found");
    
    
    if eula == false {
        print!("{}", "Hiddeneye can only be used for educational purposes, do you agree to these terms? (yes/no): ".blue());
        let answer = extra::hidden_input();
        if answer == "yes" {
            println!("You accepted the terms and conditions.")
        }
        else if answer == "no" {
            println!("{}", "You must accept the terms to use the application.".red());
            exit(1)

        }
        else {
            println!("Incorrect value inputed");
            exit(1);
        }

    }

}
