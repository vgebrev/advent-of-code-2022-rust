use crate::input::read_lines;

pub fn run(input_file_path: &str)
{
    println!("Day 2. Input file: {}", input_file_path);
    if let Ok(lines) = read_lines(input_file_path) {
        for line in lines {
            if let Ok(read_line) = line {
                //TODO: The puzzle
            }
        }
    }
}