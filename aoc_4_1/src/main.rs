use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    if let Ok(lines) = read_lines(path) {
        let mut contained = 0;
        for line_res in lines {
            if let Ok(line) = line_res {
                let pair: Vec<(i32, i32)> = line
                    .split(",")
                    .map(|pair_str| {
                        pair_str
                            .split("-")
                            .map(|el| el.parse::<i32>().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect();

                let range1 = pair.get(0).unwrap();
                let range2 = pair.get(1).unwrap();

                if range1.0 <= range2.0 && range1.1 >= range2.1 {
                    contained += 1;
                } else if range1.0 >= range2.0 && range1.1 <= range2.1 {
                    contained += 1;
                }
            }
        }
        println!("{}", contained);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
