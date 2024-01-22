use std::fs::File;
use std::io::Read;
use std::time::Instant;

// DURATION: 711.142687875s

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

fn vec_from_file(filename: &str) -> Vec<(usize, usize, usize)> {
    let mut ret: Vec<(usize, usize, usize)> = Vec::new();
    // read shit into maps
    for line in lines_from_file(filename) {
        let vec: Vec<&str> = line.split_whitespace().collect();
        ret.push((vec.get(0).unwrap().parse().unwrap(), vec.get(1).unwrap().parse().unwrap(), vec.get(2).unwrap().parse().unwrap()));
    }
    
    ret
}

fn apply(input: usize, map: &Vec<(usize, usize, usize)>) -> usize {
    for triple in map {
        // let's say we want 1 is in between 0, length 2 [0, 1]
        // 1 >= 0 AND 1 < 2
        if input >= triple.1 && input < triple.1 + triple.2 {
            // great! it's in this range!
            // find the DIFF and add it to the OFFSET
            return input - triple.1 + triple.0;
        }
    }

    input
}

fn main() {
    // Statements here are executed when the compiled binary is called.

    let timer = Instant::now();

    let lines: Vec<usize> = lines_from_file("./day_5_seeds.txt")
        .get(0)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut seeds: Vec<usize> = Vec::new();

    let mut start = None;

    for seed in lines {
        if start == None {
            start = Some(seed);
        } else {
            for num in start.unwrap()..start.unwrap() + seed {
                seeds.push(num);
            }
            start = None;
        }
    }
    
    let seed_to_soil: Vec<(usize, usize, usize)> = vec_from_file("./day_5_seed_to_soil.txt");

    let soil_to_fertilizer: Vec<(usize, usize, usize)> = vec_from_file("./day_5_soil_to_fertilizer.txt");

    let fertilizer_to_water: Vec<(usize, usize, usize)> = vec_from_file("./day_5_fertilizer_to_water.txt");

    let water_to_light: Vec<(usize, usize, usize)> = vec_from_file("./day_5_water_to_light.txt");

    let light_to_temperature: Vec<(usize, usize, usize)> = vec_from_file("./day_5_light_to_temperature.txt");

    let temperature_to_humidity: Vec<(usize, usize, usize)> = vec_from_file("./day_5_temperature_to_humidity.txt");

    let humidity_to_location: Vec<(usize, usize, usize)> = vec_from_file("./day_5_humidity_to_location.txt");

    let sum = seeds.iter()
        .map(|seed| apply(*seed, &seed_to_soil))
        .map(|soil| apply(soil, &soil_to_fertilizer))
        .map(|fertilizer| apply(fertilizer, &fertilizer_to_water))
        .map(|water| apply(water, &water_to_light))
        .map(|light| apply(light, &light_to_temperature))
        .map(|temperature| apply(temperature, &temperature_to_humidity))
        .map(|humidity| apply(humidity, &humidity_to_location))
        .min();

    println!("{:?}", sum);

    let duration = timer.elapsed();

    println!("DURATION: {:?}", duration);

}

