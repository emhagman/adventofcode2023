mod day1;
mod utils;

fn main() {
    println!(
        "day 1, part 1: {}",
        day1::part1(&utils::read_entire_file("./data/day1.txt"))
    );
    println!(
        "day 1, part 2: {}",
        day1::part2(&utils::read_entire_file("./data/day1.txt"))
    );
}
