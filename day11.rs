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

fn main() {
    let lines = lines_from_file("adventofcode.com_2023_day_11_input.txt");

    let mut x_index = 0;
    let mut y_index = 0;

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    let mut lines_without_galaxies = 0;

    for line in &lines {
        let mut galaxy_seen = false;
        x_index = 0;

        for character in line.chars() {
            if character == '#' {
                if galaxy_seen {
                } else {
                    galaxy_seen = true;
                }
                galaxies.push((x_index, y_index + (lines_without_galaxies * 999999)));

            }
            x_index += 1;
        } 

        if !galaxy_seen {
            lines_without_galaxies += 1;
        }

        y_index += 1;
    }

    let mut cols_without_galaxies_so_far: Vec<usize> = Vec::new();
    let mut col_counter = 0;
    for i in 0..lines[0].len() {
        let mut galaxy_seen = false;

        // check if theres any galaxies
        for line in &lines {
            if line.chars().nth(i).unwrap() == '#' {
                galaxy_seen = true;
            }
        }
        
        // if there is NOT increment counter
        if !galaxy_seen {
            col_counter += 1;
        }

        // push the number
        cols_without_galaxies_so_far.push(col_counter);

    }

    galaxies = galaxies.into_iter().map(|(x, y)| (x + (cols_without_galaxies_so_far[x] * 999999), y)).collect();

    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let (first_a, first_b) = galaxies[i];
            let (second_a, second_b) = galaxies[j];
            let distance = second_a.abs_diff(first_a) + second_b.abs_diff(first_b);

            sum = sum + distance;
        }
    }

    println!("{}", sum);

}