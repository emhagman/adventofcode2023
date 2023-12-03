const WORD_MAP: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

struct NumberFromWord {
    index: usize,
    value: String,
}

fn first_last(line: &str) -> (char, char) {
    let mut nums: Vec<char> = Vec::new();
    for c in line.chars() {
        if c.is_ascii_digit() {
            nums.push(c);
        }
    }
    (*nums.first().unwrap(), *nums.last().unwrap())
}

pub fn part1(input: &str) -> u32 {
    let mut numbers: Vec<((char, char), &str)> = Vec::new();
    for l in input.lines() {
        numbers.push((first_last(l), l));
    }
    let combined: Vec<u32> = numbers
        .iter()
        .map(|((a, b), _)| u32::from_str_radix(&format!("{}{}", a, b), 10).unwrap())
        .collect();
    combined.iter().sum()
}

fn replace_words(input: String) -> String {
    let mut words_found = Vec::new();
    for (w, i) in WORD_MAP.iter().enumerate() {
        let mut cloned_inputed = input.to_string();
        let mut current_idx = 0;
        while let Some(x) = cloned_inputed.find(i) {
            words_found.push(NumberFromWord {
                index: current_idx + x,
                value: if i.len() == 1 {
                    i.to_string()
                } else {
                    format!("{}", w as u32 + 1)
                },
            });
            let next_idx = x + i.len();
            current_idx += next_idx;
            cloned_inputed = cloned_inputed[next_idx..].to_string();
        }
    }
    words_found.sort_by(|a, b| a.index.partial_cmp(&b.index).unwrap());
    let mut line = "".to_string();
    for w in words_found.iter() {
        line += &w.value;
    }
    line
}

fn transform(input: &str) -> String {
    let mut new_lines = Vec::new();
    for l in input.lines() {
        let line = replace_words(l.to_string());
        new_lines.push(line);
    }
    new_lines.join("\n")
}

pub fn part2(input: &str) -> u32 {
    let lines = transform(input);
    part1(&lines)
}

#[cfg(test)]
mod tests {
    use crate::{
        day1::part1,
        day1::part2,
        day1::{first_last, transform},
        utils,
    };

    #[test]
    fn test_day1_part1_example() {
        let data = utils::read_entire_file("./data/day1.part1.example.txt");
        assert_eq!(part1(&data), 142);
    }

    #[test]
    fn test_first_last() {
        assert_eq!(first_last("618rksvzhltnn5fqvnxcsjzz31"), ('6', '1'));
        assert_eq!(first_last("2r"), ('2', '2'));
    }

    #[test]
    fn test_transform() {
        assert_eq!(transform("two1nine\ntwo1nine"), "219\n219");
        assert_eq!(transform("eightwothree"), "823");
        assert_eq!(transform("abcone2threexyz"), "123");
        assert_eq!(transform("xtwone3four"), "2134");
        assert_eq!(transform("4nineeightseven2"), "49872");
        assert_eq!(transform("zoneight234"), "18234");
        assert_eq!(transform("7pqrstsixteen"), "76");
        assert_eq!(transform("ninenineeightfivedhthreelfour5"), "9985345");
        assert_eq!(transform("nineightwoneighthree321"), "982183321");
        assert_eq!(transform("fivefour3one37sevenbhd"), "5431377");
        assert_eq!(transform("six1eightrksvzhltnnfivefqvnxcsjzzthree1"), "618531");
        assert_eq!(transform("ffnrprtnine1tjznmckv5sixczv"), "9156");
        assert_eq!(transform("one"), "1");
        assert_eq!(transform("onetwoone"), "121");
        assert_eq!(transform("moneeight2jjrfvfxztcseven"), "1827");
        assert_eq!(transform("twoz"), "2");
        assert_eq!(transform("tonewo"), "1");
        assert_eq!(transform("one"), "1");
        assert_eq!(transform("oneight"), "18");
    }

    #[test]
    fn test_day1_part2_example() {
        let data = utils::read_entire_file("./data/day1.part2.example.txt");
        assert_eq!(part2(&data), 281);
    }
}
