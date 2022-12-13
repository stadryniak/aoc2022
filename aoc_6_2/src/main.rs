use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead},
    ops::ControlFlow,
    path::Path,
};

const WINDOW_SIZE: usize = 14;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    if let Ok(lines) = read_lines(path) {
        let mut window: HashMap<char, i32> = HashMap::new();
        let chars: Vec<_> = lines
            .collect::<Result<Vec<String>, _>>()
            .unwrap()
            .get(0)
            .unwrap()
            .chars()
            .collect();
        for i in 0..WINDOW_SIZE {
            *window.entry(*chars.get(i as usize).unwrap()).or_insert(0) += 1;
        }
        if let ControlFlow::Break(_) = success_condition(&window, 4) {
            return;
        }
        for i in WINDOW_SIZE..chars.len() {
            let prev = chars.get(i - WINDOW_SIZE).copied().unwrap();
            let entry = window.entry(prev).and_modify(|f| *f -= 1).or_default();
            if *entry == 0 {
                window.remove(&prev);
            }
            let current = chars.get(i).copied().unwrap();
            *window.entry(current).or_insert(0) += 1;
            if let ControlFlow::Break(_) = success_condition(&window, i + 1) {
                return;
            }
        }
    }
}

fn success_condition(window: &HashMap<char, i32>, idx: usize) -> ControlFlow<()> {
    if window.len() == WINDOW_SIZE {
        println!("{}", idx);
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
