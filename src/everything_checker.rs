#[path = "extra.rs"] mod extra;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use serde_json::de::from_reader;
use colored::Colorize;

pub fn eula_checker() {
    // Opens the Eula and gets the eula value
    let file = File::open("eula.json")
        .expect("The file should open in read only");
    let mut json: serde_json::Value = from_reader(file)
        .expect("File should be proper json");
    let eula = json.get("eula")
        .expect("Eula was not found");
    
    
    if eula == false {
        print!("{}", "Hiddeneye can only be used for educational purposes, do you agree to these terms? (yes/no): ".blue());
        let answer = extra::hidden_input();
        if answer == "yes" {
            println!("You accepted the terms and conditions.");
            *json.pointer_mut("/eula").unwrap() = true.into();
            let json_string = json.to_string();
            let mut f = File::create("eula.json").expect("Unable to create file");
            f.write_all(json_string.as_bytes()).expect("Unable to write data");

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
    else if eula == true {
        println!("Eula Accepted")
    }
    else {
        println!("Your json file is corrupted, please replace it.")
    }

}
