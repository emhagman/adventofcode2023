mod day1;
mod day2;
mod day3;
mod day4;
mod utils;

use std::time::Instant;

fn main() {
    let tree = include_bytes!("../data/tree.txt");
    println!("\x1b[31m{}\x1b[0m", "AoC 2023");
    println!("\x1b[32m{}\x1b[0m", String::from_utf8_lossy(tree));
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
    let before = Instant::now();
    println!(
        "day 4, part 1: {} in {:.2?}",
        day4::part1(&utils::read_entire_file("./data/day4.txt")),
        before.elapsed()
    );
    let before = Instant::now();
    println!(
        "day 4, part 2: {} in {:.2?}",
        day4::part2(&utils::read_entire_file("./data/day4.txt")),
        before.elapsed()
    );
}
