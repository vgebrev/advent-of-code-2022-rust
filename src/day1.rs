use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(input_file_path: &String) {
    println!("Day 1. Input file: {}", input_file_path);
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    if let Ok(lines) = read_lines(input_file_path) {
        let mut calories = 0;
        for line in lines {
            if let Ok(read_line) = line {
                if read_line == "" {
                    if heap.len() < 3 {
                        heap.push(Reverse(calories));
                    } else if calories > heap.peek().unwrap().0 {
                        heap.pop();
                        heap.push(Reverse(calories));
                    }
                    calories = 0;
                }

                calories = calories
                    + match read_line.parse::<i32>() {
                        Ok(n) => n,
                        Err(_) => 0,
                    };
            }
        }
        let max: Vec<i32> = heap
            .into_sorted_vec()
            .into_iter()
            .map(|x| x.0)
            .collect();
        println!(
            "Max calories: {}. Top 3 max calories: {}",
            max[0],
            max[0] + max[1] + max[2]
        );
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
