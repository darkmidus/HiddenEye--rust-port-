use colored::Colorize;
#[path = "extra.rs"] mod extra;


pub fn start_main_menu() {
  println!("{}", "HiddenEye Reborn".red());
  println!("{}", "-----------------------".blue());
  println!("{}{}{}{}", "{".red(), "1".blue(), "}".red(), " Twitter".blue());
  println!();
  println!("{}", "Which site? ".red());
  let mut answer = extra::hidden_input();
  println!("{}", answer)
  
}
