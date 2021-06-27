mod everything_checker;
mod main_runner;
mod extra;

fn main() {
    main_runner::start_main_menu();
    everything_checker::eula_checker();
}
