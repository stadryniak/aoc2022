use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

#[derive(PartialEq)]
enum RSP {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RSP {
    fn get_value(&self) -> i32 {
        match self {
            RSP::Rock => 1,
            RSP::Paper => 2,
            RSP::Scissors => 3,
        }
    }
}

impl From<&str> for RSP {
    fn from(str: &str) -> Self {
        match str {
            _ if str == "A" || str == "X" => RSP::Rock,
            _ if str == "B" || str == "Y" => RSP::Paper,
            _ if str == "C" || str == "Z" => RSP::Scissors,
            _ => panic!("Invalid input: {}", str),
        }
    }
}

impl PartialOrd<RSP> for RSP {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal)
        }
        if matches!(self, RSP::Rock) && matches!(other, RSP::Paper) {
            return Some(std::cmp::Ordering::Less)
        }
        if matches!(self, RSP::Paper) && matches!(other, RSP::Scissors) {
            return Some(std::cmp::Ordering::Less)
        }
        if matches!(self, RSP::Scissors) && matches!(other, RSP::Rock) {
            return Some(std::cmp::Ordering::Less)
        }
        return Some(std::cmp::Ordering::Greater)
    }
}

fn calculate_game_points(str: &String) -> i32 {
    let words: Vec<&str> = str.split(" ").collect();
    let opponent = RSP::from(words[0]);
    let me = RSP::from(words[1]);
    let mut res: i32 = me.get_value();
    if me > opponent {
        res += 6;
    } else if me == opponent {
        res += 3;
    }
    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    if let Ok(lines) = read_lines(path) {
        let mut total_score = 0;
        for line in lines {
            total_score += calculate_game_points(&line.unwrap());
        }
        println!("{}", total_score);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

