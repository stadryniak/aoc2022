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
        let mut wrapper = 0;
        let mut elves: Vec<String> = vec![String::new(); 3];

        for line in lines {
            if let Ok(rucksack) = line {
                if wrapper > 2 {
                    sum += handle_triplet(&elves);
                    wrapper = 0;
                }
                elves[wrapper] = String::from(rucksack);
                wrapper += 1;
            }
        }
        sum += handle_triplet(&elves);

        println!("{}", sum);
    };
}

fn handle_triplet(elves: &Vec<String>) -> i32 {
    let mut elves_sets: Vec<HashSet<char>> = Vec::new();
    for elf in elves {
        let mut elf_chars: HashSet<char> = HashSet::new();
        for c in elf.chars() {
            elf_chars.insert(c);
        }
        elves_sets.push(elf_chars);
    }
    let common = common_value_in_sets(elves_sets);
    let char_as_int = common as i32;
    let a_as_int = 'a' as i32;
    let a_uppercase_as_int = 'A' as i32;
    let mut res = 0;
    if common <= 'Z' {
        res += char_as_int - a_uppercase_as_int + 27;
    } else {
        res += char_as_int - a_as_int + 1;
    }
    res
}

fn common_value_in_sets(elves_sets: Vec<HashSet<char>>) -> char {
    let common = **elves_sets
        .iter()
        .fold(None, |acc: Option<HashSet<&char>>, hs| {
            let hs = hs.iter().map(|s| s).collect();
            acc.map(|a| a.intersection(&hs).map(|s| *s).collect())
                .or(Some(hs))
        })
        .unwrap()
        .iter()
        .collect::<Vec<&&char>>()[0];
    common
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
