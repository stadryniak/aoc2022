use std::{
    collections::HashSet,
    env,
    fs::File,
    hash::Hash,
    io::{self, BufRead},
    path::Path,
    vec,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut rows: Vec<String> = vec![];
    let mut cols: Vec<String> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            rows.push(String::from(&line));
            for (idx, c) in line.chars().enumerate() {
                if let Some(col) = cols.get_mut(idx) {
                    col.push(c);
                } else {
                    cols.push(String::from(c));
                }
            }
        }
    }
    let res = calculate_solution(&rows, &cols);
    println!("{}", res);
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn swap_x_y(&self) -> Self {
        Location {
            x: self.y,
            y: self.x,
        }
    }
}

fn calculate_solution(rows: &[String], cols: &[String]) -> usize {
    let mut visible_trees: HashSet<Location> = HashSet::new();
    for (idx, row) in rows.iter().enumerate() {
        let visible = visible_in_line(row, idx);
        visible_trees = visible_trees.union(&visible).copied().collect();
    };

    for (idx, col) in cols.iter().enumerate() {
        let visible = visible_in_line(col, idx)
            .iter()
            .map(|l| l.swap_x_y())
            .collect();
        visible_trees = visible_trees.union(&visible).copied().collect();
    };

    visible_trees.len()
}

fn visible_in_line(line: &str, y_val: usize) -> HashSet<Location> {
    let mut visible_in_line: HashSet<Location> = HashSet::new();
    let mut heights: Vec<usize> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    visible_in_str(&mut visible_in_line,  0, y_val, &heights);
    heights.reverse();
    visible_in_str(&mut visible_in_line, line.len() - 1, y_val, &heights);
    visible_in_line
}

fn visible_in_str(visible_in_line: &mut HashSet<Location>, x_start: usize, y_val: usize, heights: &Vec<usize>) {
    let mut max = heights.first().unwrap();
    visible_in_line.insert(Location { x: x_start, y: y_val });
    for (idx, el) in heights.iter().enumerate() {
        if el > max {
            let xd =  x_start.abs_diff(idx);
            visible_in_line.insert(Location { x: xd, y: y_val });
            max = el;
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
