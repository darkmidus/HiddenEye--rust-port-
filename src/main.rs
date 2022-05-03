mod everything_checker;
mod main_runner;
mod extra;

fn main() {
    everything_checker::eula_checker();
    everything_checker::connection_checker();
    main_runner::start_main_menu();
}
