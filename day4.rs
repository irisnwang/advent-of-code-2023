use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::convert::TryInto;

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

fn get_game_score(line: String) -> i32 {
    // get rid of card number
    let mut parts = line.split(":");
    parts.next();
    let game = parts.next().unwrap();
    parts = game.split("|");
    let draw = parts.next().unwrap();
    let lucky_string = parts.next().unwrap();

    let mut drawn_numbers: Vec<i32> = draw.split_whitespace().map(|value| value.parse().unwrap()).collect();
    let mut lucky_numbers: Vec<i32> = lucky_string.split_whitespace().map(|value| value.parse().unwrap()).collect();

    drawn_numbers.sort();
    lucky_numbers.sort();

    println!("{:?}, {:?}", drawn_numbers, lucky_numbers);

    let mut matches = 0;
    while !drawn_numbers.is_empty() && !lucky_numbers.is_empty() {
        if drawn_numbers.get(0) == lucky_numbers.get(0) {
            matches += 1;
            drawn_numbers.remove(0);
            lucky_numbers.remove(0);
        } else if drawn_numbers.get(0) < lucky_numbers.get(0) {
            drawn_numbers.remove(0);
        } else {
            lucky_numbers.remove(0);
        }
    }

    let base: i32 = 2;

    match matches {
        0 => 0,
        matches => base.pow(matches - 1),
    }
}

fn add_scratchcards(line: String, game_number: i32, card_counts: &mut HashMap<i32, i32>) {
    let score: i32 = match get_game_score(line).checked_ilog2() {
        Some(x) => (x + 1).try_into().unwrap(),
        None => 0,
    };

    for i in 1..score + 1 {
        let num_current_card = card_counts.get(&game_number).unwrap();
        let old_num_next_card = card_counts.get(&(game_number + i)).unwrap();
        card_counts.insert(game_number + i, num_current_card + old_num_next_card);
    }
 
}

fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    let lines = lines_from_file("/Users/iris/Documents/Personal Projects/Advent of Code/adventofcode.com_2023_day_4_input.txt");

    let mut card_counts = HashMap::new();

    let mut sum = 0;
    let mut game_number = 1;
    for x in 1..lines.len() as i32 + 1 {
        card_counts.insert(x, 1);
        println!("{}", x);
    }

    for line in lines {
        add_scratchcards(line, game_number, &mut card_counts);
        game_number += 1;
    }
    
    sum = card_counts.into_iter().map(|(_game, count)| count).sum();


    println!("{}", sum);

}

