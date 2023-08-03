use crate::input::read_lines;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, Copy)]
enum GameResult {
    Lose = 0,
    Tie = 1,
    Win = 2,
}

fn map_plays() -> HashMap<char, Play> {
    let mut choices: HashMap<char, Play> = HashMap::new();
    choices.insert('A', Play::Rock);
    choices.insert('B', Play::Paper);
    choices.insert('C', Play::Scissors);
    choices.insert('X', Play::Rock);
    choices.insert('Y', Play::Paper);
    choices.insert('Z', Play::Scissors);
    choices
}

fn get_result(opp_play: &Play, my_play: &Play) -> GameResult {
    match opp_play {
        Play::Rock => match my_play {
            Play::Rock => GameResult::Tie,
            Play::Paper => GameResult::Win,
            Play::Scissors => GameResult::Lose,
        },
        Play::Paper => match my_play {
            Play::Rock => GameResult::Lose,
            Play::Paper => GameResult::Tie,
            Play::Scissors => GameResult::Win,
        },
        Play::Scissors => match my_play {
            Play::Rock => GameResult::Win,
            Play::Paper => GameResult::Lose,
            Play::Scissors => GameResult::Tie,
        },
    }
}

pub fn run(input_file_path: &str) {
    println!("Day 2. Input file: {}", input_file_path);
    if let Ok(lines) = read_lines(input_file_path) {
        let mut total_score = 0;
        for line in lines {
            if let Ok(read_line) = line {
                let choices = map_plays();
                let plays: Vec<char> = read_line.chars().filter(|x| *x != ' ').collect();
                let opp_play = choices.get(&plays[0]).unwrap();
                let my_play = choices.get(&plays[1]).unwrap();
                let result = get_result(opp_play, my_play);
                total_score += result as i32 * 3 + *my_play as i32;
            }
        }
        println!("Total score: {}", total_score);
    }
}
