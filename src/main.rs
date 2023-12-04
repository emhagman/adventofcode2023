mod day1;
mod day2;
mod day3;
mod utils;

use std::time::Instant;

fn main() {
    let before = Instant::now();
    println!(
        "day 1, part 1: {} in {:.2?}",
        day1::part1(&utils::read_entire_file("./data/day1.txt")),
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "day 1, part 2: {} in {:.2?}",
        day1::part2(&utils::read_entire_file("./data/day1.txt")),
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "day 2, part 1: {} in {:.2?}",
        day2::part1(&utils::read_entire_file("./data/day2.txt")),
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "day 2, part 2: {} in {:.2?}",
        day2::part2(&utils::read_entire_file("./data/day2.txt")),
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "day 3, part 1: {} in {:.2?}",
        day3::part1(&utils::read_entire_file("./data/day3.txt")),
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "day 3, part 2: {} in {:.2?}",
        day3::part2(&utils::read_entire_file("./data/day3.txt")),
        before.elapsed()
    );
}
