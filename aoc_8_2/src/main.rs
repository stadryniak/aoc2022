use std::{
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

fn calculate_solution(rows: &[String], cols: &[String]) -> i32 {
    let mut max = 0;
    for y in 0..cols.len() {
        let row = rows.get(y).unwrap();
        for x in 0..rows.len() {
            let col = cols.get(x).unwrap();
            let mut score = scenic_score(row, x);
            score *= scenic_score(col, y);
            if score > max {
                max = score;
            }
        }
    }
    max
}

fn scenic_score(line: &str, node_idx: usize) -> i32 {
    let heights: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let node_height = heights.get(node_idx).unwrap();
    let mut left_part = heights[0..node_idx].to_vec();
    left_part.reverse();
    let right_part = heights[node_idx + 1..heights.len()].to_vec();

    let mut visible = num_visible(left_part, node_height);
    visible *= num_visible(right_part, node_height);

    visible
}

fn num_visible(look_trees: Vec<u32>, node_height: &u32) -> i32 {
    let mut visible = 0;
    for i in look_trees {
        visible += 1;
        if &i >= node_height {
            break;
        }
    }
    visible
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
