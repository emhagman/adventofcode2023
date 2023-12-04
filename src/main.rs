mod day1;
mod day2;
mod day3;
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
    println!(
        "day 2, part 1: {}",
        day2::part1(&utils::read_entire_file("./data/day2.txt"))
    );
    println!(
        "day 2, part 2: {}",
        day2::part2(&utils::read_entire_file("./data/day2.txt"))
    );
    println!(
        "day 3, part 1: {}",
        day3::part1(&utils::read_entire_file("./data/day3.txt"))
    );
    println!(
        "day 3, part 2: {}",
        day3::part2(&utils::read_entire_file("./data/day3.txt"))
    );
}
