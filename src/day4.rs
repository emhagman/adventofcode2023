#[derive(Debug)]
struct Card {
    id: i32,
    numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
}

impl Card {
    pub fn parse(line: &str) -> Self {
        let id: i32 = line
            .split(":")
            .next()
            .expect("missing colon for game")
            .split_ascii_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .replace(":", "")
            .parse()
            .expect("id is not a number");
        let number_parts: Vec<_> = line.split(":").skip(1).next().unwrap().split(" | ").collect();
        let winning_numbers: Vec<i32> = number_parts[0]
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let numbers: Vec<i32> = number_parts[1]
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Card {
            id,
            numbers,
            winning_numbers,
        }
    }
    pub fn winners(&self) -> Vec<i32> {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .map(|n| *n)
            .collect()
    }
}

pub fn part1(input: &str) -> i32 {
    let mut sum = 0;
    for l in input.lines() {
        let c = Card::parse(l);
        let l = c.winners().len();
        sum += if l == 0 { 0 } else { (2 as i32).pow(l as u32 - 1) };
    }
    sum
}

pub fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::{
        day4::{part1, part2},
        utils,
    };

    #[test]
    fn test_day4_part1_example() {
        let data = utils::read_entire_file("./data/day4.example.txt");
        assert_eq!(part1(&data), 13);
    }

    #[test]
    fn test_day4_part2_example() {
        let data = utils::read_entire_file("./data/day4.example.txt");
        assert_eq!(part2(&data), 0);
    }
}
