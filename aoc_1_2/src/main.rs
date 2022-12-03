use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut elves: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        let mut current_elf = 0;
        for line in lines {
            if let Ok(calories) = line {
                match calories.as_str() {
                    "" => {
                        elves.push(current_elf);
                        current_elf = 0;
                    },
                    _ => current_elf += calories.parse::<i32>().unwrap(),
                };
            }
        }
        elves.push(current_elf);
    };
    elves.sort_by(|a, b| b.cmp(a));
    let sum: i32 = elves[..3].iter().sum();
    println!("{}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}