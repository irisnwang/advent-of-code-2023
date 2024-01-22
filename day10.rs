use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
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

fn go_next<'a>(x: i32, y: i32, prev_direction: &'a str, grid: &'a HashMap<(i32, i32), char>) -> (i32, i32, &'a str) {
    // println!("{}", grid.get(&(x, y)).unwrap());
    match grid.get(&(x, y)).unwrap() {
        'L' => {
            match prev_direction {
                "down" => (x + 1, y, "right"),
                "left" => (x, y - 1, "up"),
                _ => panic!("Hit an L from {}", prev_direction),
            }
        },
        'J' => {
            match prev_direction {
                "down" => (x - 1, y, "left"),
                "right" => (x, y - 1, "up"),
                _ => panic!("Hit a J from {}", prev_direction),
            }
        },
        '7' => {
            match prev_direction {
                "up" => (x - 1, y, "left"),
                "right" => (x, y + 1, "down"),
                _ => panic!("Hit a 7 from {}", prev_direction),
            }
        },
        'F' => {
            match prev_direction {
                "up" => (x + 1, y, "right"),
                "left" => (x, y + 1, "down"),
                _ => panic!("Hit a F from {}", prev_direction),
            }
        },
        '-' => {
            match prev_direction {
                "right" => (x + 1, y, "right"),
                "left" => (x - 1, y, "left"),
                _ => panic!("Hit a -from {}", prev_direction),
            }
        },
        '|' => {
            match prev_direction {
                "down" => (x, y + 1, "down"),
                "up" => (x, y - 1, "up"),
                _ => panic!("Hit a -from {}", prev_direction),
            }
        },
        'S' => (-1, -1, "whatever"),
        other => panic!("Found value {}", other),
    }
}

fn main() {
    let lines = lines_from_file("adventofcode.com_2023_day_10_input.txt");

    let height: i32 = lines.len().try_into().unwrap();
    let width: i32 = lines[0].len().try_into().unwrap();

    let mut x_index = 0;
    let mut y_index = 0;

    let mut grid: HashMap<(i32, i32), char> = HashMap::new();

    let (mut x, mut y) = (-1, -1);

    // write into grid table idk this is all sorts of wackadoodle
    for line in lines {
        x_index = 0;
        for character in line.chars() {
            if character == 'S' {
                x = x_index;
                y = y_index;
            }
            grid.insert((x_index, y_index), character);
            x_index += 1;
        } 
        y_index += 1;
    }

    let mut dist = 1;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert((x, y));
    let (mut x_a, mut y_a) = (x + 1, y);
    let (mut x_b, mut y_b) = (x, y + 1);

    let mut next_direction_a = "right";
    let mut next_direction_b = "down";

    for _i in 0..width * height {
        visited.insert((x_a, y_a));
        visited.insert((x_b, y_b));
        if (x_a, y_a) == (x_b, y_b) {
            println!("{}, {}, {} steps", x_a, y_a, dist);
            break;
        }
        (x_a, y_a, next_direction_a) = go_next(x_a, y_a, next_direction_a, &grid);
        (x_b, y_b, next_direction_b) = go_next(x_b, y_b, next_direction_b, &grid);
        dist += 1;
    }

    let mut inside_sum = 0;
    // for each line
    for y_idx in 0..height {
        let mut lines_crossed = 0;
        let mut last_curve = '.';
        for x_idx in 0..width {
            let entry =  grid.get(&(x_idx, y_idx)).unwrap();
            let is_pipe = visited.contains(&(x_idx, y_idx));

            if is_pipe {
                match entry {
                    '|' => lines_crossed += 1,
                    'L' => last_curve = 'L',
                    'F' => last_curve = 'F',
                    'J' => if last_curve == 'F' { lines_crossed += 1 },
                    '7' => if last_curve == 'L' {lines_crossed += 1 },
                    'S' => last_curve = 'F',
                    _ => (),
                }
            } else {
                inside_sum += lines_crossed % 2 
            }
          
        }
        println!("");
    }

    println!("{}", inside_sum);

}
