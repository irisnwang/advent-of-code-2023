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

fn stringToNum(line: String) -> Option<u32> {
    if line.find("one").is_some_and(|x| x == 0) {
        return Some(1);
    }
    if line.find("two").is_some_and(|x| x == 0) {
        return Some(2);
    }
    if line.find("three").is_some_and(|x| x == 0) {
        return Some(3);
    }
    if line.find("four").is_some_and(|x| x == 0) {
        return Some(4);
    }
    if line.find("five").is_some_and(|x| x == 0) {
        return Some(5);
    }
    if line.find("six").is_some_and(|x| x == 0) {
        return Some(6);
    }
    if line.find("seven").is_some_and(|x| x == 0) {
        return Some(7);
    }
    if line.find("eight").is_some_and(|x| x == 0) {
        return Some(8);
    }
    if line.find("nine").is_some_and(|x| x == 0) {
        return Some(9);
    }
    None
}

fn reverseStringToNum(line: String) -> Option<u32> {
    if line.find("eno").is_some_and(|x| x == 0) {
        return Some(1);
    }
    if line.find("owt").is_some_and(|x| x == 0) {
        return Some(2);
    }
    if line.find("eerht").is_some_and(|x| x == 0) {
        return Some(3);
    }
    if line.find("ruof").is_some_and(|x| x == 0) {
        return Some(4);
    }
    if line.find("evif").is_some_and(|x| x == 0) {
        return Some(5);
    }
    if line.find("xis").is_some_and(|x| x == 0) {
        return Some(6);
    }
    if line.find("neves").is_some_and(|x| x == 0) {
        return Some(7);
    }
    if line.find("thgie").is_some_and(|x| x == 0) {
        return Some(8);
    }
    if line.find("enin").is_some_and(|x| x == 0) {
        return Some(9);
    }
    None
}

fn getNums(line: &str) -> (u32, u32) {
    let mut first = 0;
    let mut second = 0;
    let mut i = 0;
    for c in line.chars() {
        if c.to_digit(10).is_some() {
            first = c.to_digit(10).unwrap();
            break;
        }

        let substring: String = line.chars().skip(i).take(5).collect();
        let number = stringToNum(substring);
        if number.is_some() {
            first = number.unwrap();
            break;
        }

        i += 1;
    }

    i = 0;
    for c in line.chars().rev() {
        let digit = c.to_digit(10);
        if digit.is_some() {
            second = digit.unwrap();
            break;
        }

        let substring: String = line.chars().rev().skip(i).take(5).collect();
        let number = reverseStringToNum(substring);
        if number.is_some() {
            second = number.unwrap();
            break;
        }
        i += 1;
    }
    (first, second)
}

fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    let lines = lines_from_file("/Users/iris/Documents/Personal Projects/Advent of Code/adventofcode.com_2023_day_1_input.txt");
    let mut sum = 0;
    for line in lines {
        let (first, second) = getNums(&line);
        let num = first * 10 + second;
        sum = sum + num;
    }

    println!("{}", sum);
}
