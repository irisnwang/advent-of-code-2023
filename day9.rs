//10  13  16  21  30  45
use std::fs::File;
use std::io::Read;

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

fn next_element(numbers: Vec<i32>) -> i32 {
    if numbers.clone().into_iter().all(|x| x == 0) {
        return 0;
    }

    let last = numbers.last().unwrap();
    let mut prev = numbers.first().unwrap();

    let mut diff: Vec<i32> = Vec::new();

    for i in 1..numbers.len() {
        diff.push(numbers[i] - prev);

        prev = &numbers[i];
    }

    return next_element(diff) + last;
}

fn main() {
    let lines = lines_from_file("adventofcode.com_2023_day_9_input.txt");

    let mut sum = 0;
    for line in lines {
        let numbers: Vec<i32> = line.split_whitespace().into_iter().rev().map(|x| x.parse().unwrap()).collect();
        sum += next_element(numbers);
    }
    println!("{}", sum);

}