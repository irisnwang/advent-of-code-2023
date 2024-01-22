use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

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

#[derive(Debug)]
struct IrisNumber {
    value: i32,
    x_pos: i32,
    length: i32,
    y_pos: i32,
}

fn count(number: &IrisNumber, special_characters: HashMap<(i32, i32), bool>) -> bool {
    for x in number.x_pos - 1..number.x_pos + number.length + 1 {
        match special_characters.get(&(x, number.y_pos - 1)) {
            Some(true) => return true,
            _ => (),
        };

        match special_characters.get(&(x, number.y_pos + 1)) {
            Some(true) => return true,
            _ => (),
        };
    }

    match special_characters.get(&(number.x_pos - 1, number.y_pos)) {
        Some(true) => return true,
        _ => (),
    };
    
    match special_characters.get(&(number.x_pos + number.length, number.y_pos)) {
        Some(true) => return true,
        _ => (),
    };

    println!("{:?}", number);
    false
}

fn getLineSum(line: &str, y_pos: i32, special_characters: HashMap<(i32, i32), bool>) -> i32 {
    // split into discrete numbers WITH location info
    // effectively get Iris Numbers
    let mut x_pos = 0;
    let mut string_builder: String = "".to_string();
    let mut active = false;
    let mut numbers: Vec<IrisNumber> = Vec::new();

    for character in line.chars() {
        if character.is_numeric() {
            active = true;
            string_builder.push(character);
        } else if active {
            let length = string_builder.len() as i32;
            numbers.push(IrisNumber {
                value: string_builder.parse().unwrap(),
                x_pos: x_pos - length,
                length: length,
                y_pos: y_pos,
            });
            string_builder = "".to_string();
            active = false;
        }
        x_pos += 1;
    }

    if active {
        let length = string_builder.len() as i32;
        numbers.push(IrisNumber {
            value: string_builder.parse().unwrap(),
            x_pos: x_pos - length,
            length: length,
            y_pos: y_pos,
        });
    }

    let mut sum = 0;

    for number in numbers {
        if count(&number, special_characters.clone()) {
            sum += number.value;
        }
    }

    sum
}

fn addToGearMap(number: &IrisNumber, special_characters: &mut HashMap<(i32, i32), (i32, i32)>) {
    for x in number.x_pos - 1..number.x_pos + number.length + 1 {
        match special_characters.get(&(x, number.y_pos - 1)) {
            Some((0, _)) => special_characters.insert((x, number.y_pos - 1), (1, number.value)),
            Some((count, ratio)) => special_characters.insert((x, number.y_pos - 1), (count + 1, ratio * number.value)),
            _ => None,
        };

        match special_characters.get(&(x, number.y_pos + 1)) {
            Some((0, _)) => special_characters.insert((x, number.y_pos + 1), (1, number.value)),
            Some((count, ratio)) => special_characters.insert((x, number.y_pos + 1), (count + 1, ratio * number.value)),
            _ => None,
        };
    }

    match special_characters.get(&(number.x_pos - 1, number.y_pos)) {
        Some((0, _)) => special_characters.insert((number.x_pos - 1, number.y_pos), (1, number.value)),
        Some((count, ratio)) => special_characters.insert((number.x_pos - 1, number.y_pos), (count + 1, ratio * number.value)),
        _ => None,
    };
    
    match special_characters.get(&(number.x_pos + number.length, number.y_pos)) {
        Some((0, _)) => special_characters.insert((number.x_pos + number.length, number.y_pos), (1, number.value)),
        Some((count, ratio)) => special_characters.insert((number.x_pos + number.length, number.y_pos), (count + 1, ratio * number.value)),
        _ => None,
    };
}

fn updateGearRatio(line: &str, y_pos: i32, special_characters: &mut HashMap<(i32, i32), (i32, i32)>) {
    // split into discrete numbers WITH location info
    // effectively get Iris Numbers
    let mut x_pos = 0;
    let mut string_builder: String = "".to_string();
    let mut active = false;
    let mut numbers: Vec<IrisNumber> = Vec::new();

    for character in line.chars() {
        if character.is_numeric() {
            active = true;
            string_builder.push(character);
        } else if active {
            let length = string_builder.len() as i32;
            numbers.push(IrisNumber {
                value: string_builder.parse().unwrap(),
                x_pos: x_pos - length,
                length: length,
                y_pos: y_pos,
            });
            string_builder = "".to_string();
            active = false;
        }
        x_pos += 1;
    }

    if active {
        let length = string_builder.len() as i32;
        numbers.push(IrisNumber {
            value: string_builder.parse().unwrap(),
            x_pos: x_pos - length,
            length: length,
            y_pos: y_pos,
        });
    }

    let mut sum = 0;

    for number in numbers {
        addToGearMap(&number, special_characters);
    }
}

fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    let lines = lines_from_file("/Users/iris/Documents/Personal Projects/Advent of Code/adventofcode.com_2023_day_3_input.txt");

    let mut special_characters = HashMap::new();

    let mut y_index = 0;
    let mut x_index = 0;
    for line in &lines {
        x_index = 0;
        for character in line.chars() {
            if character == '*' {
                special_characters.insert((x_index, y_index), (0, 1));
            }
            x_index += 1;
        } 
        y_index += 1;
    }

    let mut sum = 0;

    y_index = 0;

    // count 
    for line in &lines {
        updateGearRatio(&line, y_index, &mut special_characters);
        y_index += 1;
    }

    let all_scores: Vec<i32> = special_characters.into_iter()
        .map(|(_id, pair)| pair)
        .filter(|(occurrences, ratio)| occurrences > &1)
        .map(|(_occurrences, ratio)| ratio)
        .collect();

    for score in all_scores {
        sum += score;
    }
    
    println!("{}", sum);

}

