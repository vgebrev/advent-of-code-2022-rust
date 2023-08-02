use std::collections::HashMap;
use std::env;

mod input;
mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].as_str();
    let input_file_path = args[2].as_str();

    let mut day_fn_map: HashMap<&str, fn(&str)> = HashMap::new();
    day_fn_map.insert("1", day1::run);
    day_fn_map.insert("2", day2::run);

    if let Some(day_fn) = day_fn_map.get(day) {
        day_fn(input_file_path);
    }
}
