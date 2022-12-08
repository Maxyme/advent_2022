use std::collections::HashMap;
use std::fs;
use std::str::Lines;

#[derive(Debug, Clone)]
struct Folder {
    name: String,
    parent: Option<String>,
    children: Vec<String>,
    files: Vec<usize>,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    // Skip first line
    lines.next();

    let folders_map = parse_commands(lines);

    // Part 1: find all of the directories with a total size of at most 100000, and calculate the sum
    let mut dirs = Vec::<usize>::new();
    let _ = walk_folder("/", &folders_map, &mut dirs);
    let sum = dirs.iter().filter(|&x| x <= &100000).sum::<usize>();
    println!("Part 1: {sum}");

    // Part 2: Choose the smallest directory under 30000000
    dirs.sort();
    let smallest_size_needed = 70000000 - dirs[dirs.len() - 1];
    let size = dirs
        .into_iter()
        .find(|x| x >= &(30000000 - smallest_size_needed))
        .unwrap();
    println!("Part 2: {size:?}");
}

// Return folder size and update tree structure
fn walk_folder(root_dir: &str, fm: &HashMap<String, Folder>, dirs: &mut Vec<usize>) -> usize {
    let folder = fm.get(root_dir).unwrap();
    let sum_children = folder
        .children
        .iter()
        .fold(0, |sum, child| sum + walk_folder(child, fm, dirs));

    // Add local files
    let sum_local_files = folder.files.iter().sum::<usize>();
    let folder_size = sum_children + sum_local_files;

    dirs.push(folder_size);

    folder_size
}

/// Parse commands into a directory structure
fn parse_commands(lines: Lines) -> HashMap<String, Folder> {
    let mut curr_dir = Folder {
        name: "/".to_string(),
        parent: None,
        children: Vec::<String>::new(),
        files: Vec::<usize>::new(),
    };

    let mut all_folders: HashMap<String, Folder> = HashMap::new();

    for command in lines {
        // todo use match
        if command.starts_with("$ ls") {
            // skip ls commands
            continue;
        } else if command == "$ cd .." {
            // Save previous dir in hashmap
            all_folders.insert(curr_dir.name.clone(), curr_dir.clone());

            // Set the curr dir as the parent dir
            let parent_dir = all_folders.get(&curr_dir.parent.unwrap()).unwrap();
            curr_dir = parent_dir.clone();
        } else if command.starts_with("$ cd") {
            // Save previous dir in hashmap
            all_folders.insert(curr_dir.name.clone(), curr_dir.clone());

            // Set new current dir with full path as name
            // Todo: we can use RefCell for this and point to another folder up or down
            let dir_name = command.split(' ').last().unwrap().to_string();
            curr_dir = Folder {
                name: format!("{}/{}", curr_dir.name, dir_name),
                parent: Some(curr_dir.name.clone()),
                children: Vec::<String>::new(),
                files: Vec::<usize>::new(),
            };
            continue;
        }
        if !command.starts_with('$') {
            if command.starts_with("dir") {
                let dir = command.split(' ').last().unwrap();
                let full_path = format!("{}/{}", curr_dir.name, dir);
                curr_dir.children.push(full_path.to_string());
            } else {
                let file_size = command.split(' ').next().unwrap();
                curr_dir.files.push(file_size.parse::<usize>().unwrap());
            }
        }
    }

    // Update curr_dir
    all_folders.insert(curr_dir.name.clone(), curr_dir.clone());
    all_folders
}
