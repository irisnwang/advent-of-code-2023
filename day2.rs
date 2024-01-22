use std::fs::File;
use std::io::Read;
use std::cmp;

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

fn isValidColorAmt(draw: &str) -> bool {

    let parts: Vec<&str> = draw.split_whitespace().collect();
    let num: i32 = parts[0].parse().unwrap();
    let color = parts[1];

    match color {
        "green" => num < 14,
        "blue" => num < 15,
        "red" => num < 13,
        _ => false,
    }
}

fn isValidDraw(draw: &str) -> bool {
    let colors: Vec<&str> = draw.split(", ").collect();
    for color in colors {
        if !isValidColorAmt(color) {
            return false;
        }
    }
    return true;
}

fn isValidGame(draws: String) -> bool {
    let drawList: Vec<&str> = draws.split(";").collect();
    for draw in drawList {
        if !isValidDraw(draw) {
            return false;
        }
    }
    
    true
}

fn getValidGameId(line: String) -> Option<u32> {
    // Get game ID
    let parts: Vec<&str> = line.split(":").collect();
    let mut gameId = parts[0];
    let mut idNumber = gameId.split(" ");
    idNumber.next();
    let gameId = idNumber.next().unwrap();
    let gameData = parts[1];

    if isValidGame(gameData.to_string()) {
        return gameId.parse::<u32>().ok();
    }
    None
}

fn getDrawValues(draw: &str) -> (Option<u32>, Option<u32>, Option<u32>) {
    let colors: Vec<&str> = draw.split(", ").collect();
    let mut green = None;
    let mut blue = None;
    let mut red = None;
    for color in colors {
        let parts: Vec<&str> = color.split_whitespace().collect();
        let num: u32 = parts[0].parse().unwrap();
        let color = parts[1];
    
        match color {
            "green" => green = Some(num),
            "blue" => blue = Some(num),
            "red" => red = Some(num),
            _ => (),
        }
    }
    return (green, blue, red);
}

fn getGameMinimums(draws: &str) -> (u32, u32, u32) {
    let drawList: Vec<&str> = draws.split(";").collect();
    let mut greenMin = None;
    let mut blueMin = None;
    let mut redMin = None;
    for draw in drawList {
        let (green, blue, red) = getDrawValues(draw);

        match (green, greenMin) {
            (Some(x), None) => greenMin = Some(x),
            (None, None) => greenMin = None,
            (Some(x), Some(y)) => greenMin = Some(cmp::max(x, y)),
            (None, Some(x)) => greenMin = Some(x),
        }

        match (blue, blueMin) {
            (Some(x), None) => blueMin = Some(x),
            (None, None) => blueMin = None,
            (Some(x), Some(y)) => blueMin = Some(cmp::max(x, y)),
            (None, Some(x)) => blueMin = Some(x),
        }

        match (red, redMin) {
            (Some(x), None) => redMin = Some(x),
            (None, None) => redMin = None,
            (Some(x), Some(y)) => redMin = Some(cmp::max(x, y)),
            (None, Some(x)) => redMin = Some(x),
        }
    }
    
    (greenMin.unwrap_or(0), redMin.unwrap_or(0), blueMin.unwrap_or(0))
}

fn getGamePower(line: String) -> u32 {
    // Get game ID
    let parts: Vec<&str> = line.split(":").collect();
    let gameData = parts[1];

    let (green, red, blue) = getGameMinimums(gameData);
    return green * red * blue;
}

fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    let mut lines = lines_from_file("/Users/iris/Documents/Personal Projects/Advent of Code/adventofcode.com_2023_day_2_input.txt");
    lines.truncate(100);
    let mut sum = 0;
    for line in lines {
        let gamePower = getGamePower(line.to_string());
        sum += gamePower;

    }

    println!("{}", sum);
}
