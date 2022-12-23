use std::{
    env,
    fs::File,
    io::{self, BufRead},
    mem,
    ops::ControlFlow,
    path::Path,
    str::FromStr,
    string::ParseError,
};

type NodeIdx = usize;
type ChildDirs = Vec<NodeIdx>;

#[derive(Clone, PartialEq)]
struct DirFile {
    size: usize,
}

impl FromStr for DirFile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        let size = parts.first().unwrap().parse().unwrap();
        Ok(Self { size })
    }
}

#[derive(Clone, PartialEq)]
struct Dir {
    name: String,
    files: Vec<DirFile>,
    children: ChildDirs,
    parent: Option<NodeIdx>,
}

impl FromStr for Dir {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        let name = parts.get(1).unwrap().parse().unwrap();
        Ok(Self {
            name,
            files: vec![],
            children: vec![],
            parent: None,
        })
    }
}

struct Area {
    tree: Vec<Dir>,
}

impl Area {
    fn new() -> Area {
        let root = Dir {
            name: String::from("/"),
            files: vec![],
            children: vec![],
            parent: None,
        };
        Area { tree: vec![root] }
    }

    fn get_node(&self, idx: usize) -> &Dir {
        self.tree.get(idx).unwrap()
    }

    fn get_child_by_name<'a>(&'a self, dir: &'a Dir, name: &str) -> Option<&'a Dir> {
        if name == ".." {
            if dir.name == "/" {
                return Some(dir);
            }
            return Some(self.get_node(dir.parent.unwrap()));
        }
        for i in &dir.children {
            let current_node = self.get_node(*i);
            if current_node.name == name {
                return Some(current_node);
            }
        }
        None
    }

    fn add_dir(&mut self, dir: Dir) -> usize {
        self.tree.push(dir);
        self.tree.len() - 1
    }

    fn get_dir_idx(&self, dir: &Dir) -> Option<usize> {
        self.tree
            .iter()
            .position(|d| d.name == dir.name && d.parent == dir.parent)
    }

    fn update_dir(&mut self, dir: &Dir) {
        if let Some(idx) = self.get_dir_idx(dir) {
            let _ = mem::replace(&mut self.tree[idx], dir.clone());
        } else {
            panic!("dir not found")
        }
    }

    fn dir_size(&self, dir: &Dir) -> usize {
        let mut size: usize = 0;
        for f in &dir.files {
            size += f.size;
        }
        for child_idx in &dir.children {
            let child = self.get_node(*child_idx);
            size += self.dir_size(child);
        }
        size
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut filesystem = Area::new();
    if let Ok(lines) = read_lines(path) {
        let mut current_node = filesystem.get_node(0).clone();
        for line in lines.flatten() {
            if line.starts_with('$') {
                if let ControlFlow::Continue(_) =
                    handle_command(&line, &filesystem, &mut current_node)
                {
                    continue;
                }
            } else if line.starts_with("dir") {
                add_dir(&line, &mut filesystem, &mut current_node);
            } else {
                add_files(line, &mut current_node, &mut filesystem);
            }
        }
    }

    let mut sum = 0;
    for i in &filesystem.tree {
        let size = filesystem.dir_size(i);
        if size <= 100000 {
            sum += size;
        }
    }
    println!("{}", sum);
}

fn add_files(line: String, current_node: &mut Dir, filesystem: &mut Area) {
    let file_data: Vec<&str> = line.split(' ').collect();
    current_node.files.push(DirFile {
        size: file_data[0].parse().unwrap(),
    });
    filesystem.update_dir(&*current_node);
}

fn add_dir(line: &str, filesystem: &mut Area, current_node: &mut Dir) {
    let dir_data: Vec<&str> = line.split(' ').collect();
    let parent_idx = filesystem.get_dir_idx(&*current_node);
    let child = Dir {
        name: String::from(dir_data[1]),
        files: vec![],
        children: vec![],
        parent: parent_idx,
    };
    let idx = filesystem.add_dir(child);
    current_node.children.push(idx);
    filesystem.update_dir(&*current_node);
}

fn handle_command(line: &str, filesystem: &Area, current_node: &mut Dir) -> ControlFlow<()> {
    let cmd: Vec<&str> = line.split(' ').collect();
    match cmd[1] {
        "cd" => {
            if cmd[2] == "/" {
                return ControlFlow::Continue(());
            }
            if let Some(dir) = filesystem.get_child_by_name(&*current_node, cmd[2]) {
                *current_node = dir.clone();
            }
        }
        "ls" => return ControlFlow::Continue(()),
        _ => panic!("Unknown command"),
    }
    ControlFlow::Break(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
