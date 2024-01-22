mod day7;

use std::time::Instant;

fn get_wins(time: usize, record: usize) -> usize {
    let mut wins = 0;
    for charge in 0..time {
        let distance = charge * (time - charge);

        if distance > record {
            return charge;
        }
    }

    wins
}

fn main() {

    let start = Instant::now();
    // Statements here are executed when the compiled binary is called.
    let wins = get_wins(58819676, 434104122191218);
    // it's symmetrical -- get what the first winning game is

    println!("{}", 58819676 - wins * 2 + 1);
    println!("{:?}", start.elapsed());
}
