use std::collections::HashMap;
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

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Hand {
    card_values: Vec<u32>,
    rank: u32,
    bid: u32,
}

fn cmp(hand: &Hand, other: &Hand) -> Ordering {
    match hand.rank.cmp(&other.rank) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => (),
    };

    for i in 0..5 {
        match hand.card_values.get(i).cmp(&other.card_values.get(i)) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        };
    }

    return Ordering::Equal;

}

fn to_digit(c: char) -> u32 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

fn two_helper(value: u32) -> u32 {
    match value {
        4 => 5,
        3 => 4,
        2 => 4,
        1 => 5,
        _ => 100,
    }
}

fn three_helper(first: u32, second: u32) -> u32 {
    match first {
        3 => return 3,
        2 => return 2,
        _ => (),
    };

    match second {
        3 => 3,
        2 => 2,
        1 => 3,
        _ => 100,
    }
}

fn get_rank_from_cards(cards: &Vec<u32>) -> u32 {
    let mut count_map: HashMap<u32, u32> = HashMap::new();
    let mut j = false;
    let (mut max_card, mut max_count) = (0, 0);
    for card in cards {

        let new_value = match count_map.get(card) {
            Some(count) => count + 1,
            None => 1,
        };

        if card == &1 {
            j = true;
        } else if new_value > max_count {
            max_card = *card;
            max_count = new_value;
        }

        count_map.insert(*card, new_value);
    }
    
    if j && count_map.len() > 1 {
        let j_count = count_map.get(&1).unwrap();
        // find most common element
        count_map.insert(max_card, max_count + j_count);
        count_map.remove(&1);
    }

    let count_vec: Vec<(u32, u32)> = count_map.into_iter().collect();

    println!("{:?}", count_vec);

    match count_vec.len() {
        1 => 6,
        2 => two_helper(count_vec.get(0).unwrap().1),
        3 => three_helper(count_vec.get(0).unwrap().1, count_vec.get(1).unwrap().1),
        4 => 1,
        5 => 0,
        _ => 100,
    }

}

fn apply_wildcards(cards: &Vec<u32>) -> Vec<u32> {
    if !cards.contains(&1) {
        return cards.to_vec();
    }

    // if it DOES contain jacks

    Vec::new()
}

fn get_hand_from_string(line: String) -> Hand {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let bid: u32 = parts.get(1).unwrap().parse().unwrap();
    let card_values: Vec<u32> = parts.get(0).unwrap().chars().map(|c| to_digit(c)).collect();
    let rank = get_rank_from_cards(&card_values);

    Hand {
        card_values,
        rank,
        bid,
    }
}

fn main() {
    let lines = lines_from_file("./adventofcode.com_2023_day_7_input.txt");

    let mut hands = Vec::new();

    for line in lines {
        let hand = get_hand_from_string(line);
        hands.push(hand);
    }

    hands.sort_by(|a, b| cmp(a, b));

    // println!("{:?}", hands);

    let mut sum = 0;
    let mut rank = 1;

    for hand in hands {
        sum += hand.bid * rank;
        rank += 1;
    }

    println!("{:?}", sum);

}