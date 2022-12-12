use std::{
    env,
    fs::File,
    io::{self, BufRead},
    num::ParseIntError,
    ops::ControlFlow,
    path::Path,
    str::FromStr,
};

struct Command {
    count: i32,
    from: i32,
    to: i32,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        let count: i32 = parts[1].parse().unwrap();
        let from: i32 = parts[3].parse().unwrap();
        let to: i32 = parts[5].parse().unwrap();
        Ok(Command {
            count,
            from: from - 1,
            to: to - 1,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut stacks: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        let mut mode = 0;
        for str in lines.flatten() {
            let matches: Vec<_> = str.match_indices('[').collect();
            if !matches.is_empty() {
                append_to_stack(matches, &mut stacks, &str);
            } else if mode == 0 {
                reverse_vecs(&mut stacks);
                mode = 1;
            } else if let ControlFlow::Break(_) = process_move_line(str, &mut stacks) {
                continue;
            }
        }
    };
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}

fn reverse_vecs(stacks: &mut [Vec<char>]) {
    for s in stacks.iter_mut() {
        s.reverse();
    }
}

fn process_move_line(str: String, stacks: &mut [Vec<char>]) -> ControlFlow<()> {
    if str.is_empty() {
        return ControlFlow::Break(());
    }
    let cmd = Command::from_str(&str).unwrap();
    for _ in 0..cmd.count {
        let item = stacks.get_mut(cmd.from as usize).unwrap().pop().unwrap();
        stacks.get_mut(cmd.to as usize).unwrap().push(item);
    }
    ControlFlow::Continue(())
}

fn append_to_stack(matches: Vec<(usize, &str)>, stacks: &mut Vec<Vec<char>>, str: &str) {
    for (idx, _) in matches {
        while stacks.len() < idx / 4 + 1 {
            stacks.push(Vec::new());
        }
        stacks
            .get_mut(idx / 4)
            .unwrap()
            .push(str.chars().nth(idx + 1).unwrap());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
