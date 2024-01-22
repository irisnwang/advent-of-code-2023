extern crate itertools;

use std::fs::File;
use std::io::Read;
use itertools::Itertools;

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

fn check_combination(springs: String, groups: &Vec<usize>) -> bool {
    let springs_grouped = springs.split('.').filter(|s| s.len() > 0);
    for (spring_grouped, group) in springs_grouped.zip(groups.into_iter()) {
        if spring_grouped.len() != *group {
            return  false;
        }
    }

    true
}

fn num_possible_combinations(line: &str) -> i32 {
    // get all combinations
    let parts: Vec<&str> = line.split(" ").collect();
    let spring_list = vec![parts[0]; 5].join("?");
    let mut groups: Vec<usize> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();
    groups = vec![groups; 5].concat();

    let mut unknown_spring_indices: Vec<usize> = Vec::new();
    let mut number_springs_missing: usize = groups.iter().sum();
    for (idx, char) in spring_list.char_indices() {
        match char {
            '?' => unknown_spring_indices.push(idx),
            '#' => number_springs_missing -= 1,
            _ => (),
        };
    }
    let combinations: Vec<Vec<&usize>> = unknown_spring_indices.iter().combinations(number_springs_missing).collect();

    let mut sum = 0;

    for combo in combinations {
        let mut sb = String::new();

        for (idx, char) in spring_list.char_indices() {
            match char {
                '?' => if combo.contains(&&idx) { sb.push('#') } else { sb.push('.') },
                x => sb.push(x),
            };
        }

        if check_combination(sb, &groups) { sum += 1 };
    }

    // println!("{}, {:?}", spring_list, groups);

    sum
}

fn main() {
    let lines = lines_from_file("adventofcode.com_2023_day_12_input.txt");

    let mut sum = 0;
    for line in lines {
        sum += num_possible_combinations(&line);
    }


    println!("{}", sum);

    // let a = ".#".to_string();
    // let a_unfold = vec![a; 5].join("?");

    // let b = vec![1, 1, 3];
    // let b_unfold = vec![b; 5].concat();

    // println!("{} {:?}", a_unfold, b_unfold);

}