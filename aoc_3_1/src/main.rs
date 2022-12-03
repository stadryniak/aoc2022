use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    if let Ok(lines) = read_lines(path) {
        let mut sum = 0;
        for line in lines {
            if let Ok(rucksack) = line {
                let mid = rucksack.len() / 2;

                let left = &rucksack[..mid];
                let mut left_chars: HashSet<char> = HashSet::new();
                for c in left.chars() {
                    left_chars.insert(c);
                }

                let right = &rucksack[mid..];
                let mut right_chars: HashSet<char> = HashSet::new();
                for c in right.chars() {
                    right_chars.insert(c);
                }
                let common = *left_chars
                    .intersection(&right_chars)
                    .collect::<Vec<&char>>()[0];
                let char_as_int = common as i32;
                let a_as_int = 'a' as i32;
                let a_uppercase_as_int = 'A' as i32;
                if common <= 'Z' {
                    sum += char_as_int - a_uppercase_as_int + 27;
                } else {
                    sum += char_as_int - a_as_int + 1;
                }
            }
        }
        println!("{}", sum);
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
