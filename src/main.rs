use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = &args[1];
    day1::run(&input_file_path);
}
