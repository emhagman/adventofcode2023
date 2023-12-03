#[derive(Debug)]
struct CubeCount {
    red: i32,
    green: i32,
    blue: i32,
}

impl CubeCount {
    fn new() -> Self {
        CubeCount {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    pub fn assign(&mut self, c: &str, v: i32) {
        match c {
            "red" => self.red = v,
            "green" => self.green = v,
            "blue" => self.blue = v,
            _ => panic!("not a real color"),
        }
    }
}

#[derive(Debug)]
struct GameSession {
    pub id: u32,
    pub rounds: Vec<CubeCount>,
}

impl GameSession {
    fn new() -> Self {
        GameSession {
            id: 0,
            rounds: Vec::new(),
        }
    }

    pub fn parse(input: impl Into<String>) -> Self {
        let mut gs = GameSession::new();
        let text = input.into();
        for l in text.lines() {
            gs.parse_line(l);
        }
        gs
    }

    fn parse_line(&mut self, line: &str) {
        let parts: Vec<&str> = line.split(":").collect();
        self.id = parts
            .get(0)
            .expect("failed to split on :")
            .split_whitespace()
            .skip(1)
            .next()
            .expect("not two parts within :")
            .parse()
            .expect("id is not a number");
        let rounds: Vec<&str> = parts[1].split(";").collect();
        for r in rounds {
            let mut cc = CubeCount::new();
            let counts: Vec<&str> = r.split(",").map(|f| f.trim()).collect();
            for c in counts {
                let mut count_iter = c.split_whitespace();
                let num: i32 = count_iter.next().unwrap().parse().unwrap();
                let color = count_iter.next().unwrap();
                cc.assign(color, num);
            }
            self.rounds.push(cc);
        }
    }

    pub fn is_possible(&self, bag: &CubeCount) -> bool {
        for r in &self.rounds {
            if r.red > bag.red {
                return false;
            }
            if r.green > bag.green {
                return false;
            }
            if r.blue > bag.blue {
                return false;
            }
        }
        return true;
    }
}

fn parse_sessions(input: &str) -> Vec<GameSession> {
    let mut sessions = Vec::new();
    for l in input.lines() {
        let gs = GameSession::parse(l);
        sessions.push(gs);
    }
    sessions
}

pub fn part1(input: &str) -> i32 {
    let bag = CubeCount {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sessions = parse_sessions(input);
    sessions
        .iter()
        .filter(|s| s.is_possible(&bag))
        .map(|s| s.id as i32)
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let sessions = parse_sessions(input);
    sessions
        .iter()
        .map(|s| {
            let max_red = s.rounds.iter().map(|r| r.red).max().unwrap();
            let max_green = s.rounds.iter().map(|r| r.green).max().unwrap();
            let max_blue = s.rounds.iter().map(|r| r.blue).max().unwrap();
            max_red * max_green * max_blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{
        day2::{part1, part2},
        utils,
    };

    #[test]
    fn test_day2_part1_example() {
        let data = utils::read_entire_file("./data/day2.example.txt");
        assert_eq!(part1(&data), 8);
    }

    #[test]
    fn test_day2_part2_example() {
        let data = utils::read_entire_file("./data/day2.example.txt");
        assert_eq!(part2(&data), 2286);
    }
}
