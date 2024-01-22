use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;

fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}

fn get_names_from_string(line: String) -> (String, String, String) {
    let letters: Vec<&str> = line.split(|c: char| !c.is_alphanumeric()).collect();
    (letters[0].to_string(), letters[4].to_string(), letters[6].to_string())
}

fn main() {
    let mut lines = lines_from_file("./adventofcode.com_2023_day_8_input.txt");

    let mut node_set: HashMap<String, (String, String)> = HashMap::new();

    let instructions: Vec<char> = lines.remove(0).chars().collect();

    lines.remove(0);

    for line in lines {
        let names: (String, String, String) = get_names_from_string(line);

        node_set.insert(names.0, (names.1, names.2));
    }
    
    let mut current: Vec<String> = node_set.keys().filter(|s| s.chars().nth(2).unwrap() == 'A').map(|s| s.to_string()).collect();

    println!("{:?}", current);

    for mut c in current {
        let mut step_count = 0;
        while true {
        
            if c.chars().nth(2).unwrap() == 'Z' {
                println!("{}", step_count);
                break;
            }
    
            let next_instruction = instructions.get(step_count % instructions.len()).unwrap();
    
            c = match next_instruction {
                'L' => node_set.get(&c).unwrap().0.clone(),
                'R' => node_set.get(&c).unwrap().1.clone(),
                _ => c,
            };
    
            step_count += 1;
    
        }
    }
 
}
