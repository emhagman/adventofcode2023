struct RawSchematic(String, usize, usize);

impl RawSchematic {
    fn new(input: impl Into<String>) -> Self {
        RawSchematic(input.into(), 0, 0)
    }
}

#[derive(Debug)]
struct NumberOrSymbol {
    value: String,
    row_index: usize,
    col_index: usize,
}

impl NumberOrSymbol {
    pub fn parsed_value(&self) -> i32 {
        self.value.parse().unwrap()
    }
    pub fn is_symbol(&self) -> bool {
        if self.value.len() > 1 {
            return false;
        }
        let c = self.value.chars().next().unwrap();
        !c.is_ascii_digit() && c != '.'
    }
    pub fn is_number(&self) -> bool {
        self.value.chars().next().unwrap().is_ascii_digit()
    }
    pub fn is_gear(&self) -> bool {
        self.value.chars().next().unwrap() == '*'
    }
    pub fn is_adjacent(&self, symbol: &NumberOrSymbol) -> bool {
        for i in 0..self.value.len() {
            let similar_row = (self.row_index + i).abs_diff(symbol.row_index) <= 1;
            let similar_col = self.col_index.abs_diff(symbol.col_index) <= 1;
            if similar_row && similar_col {
                return true;
            }
        }
        false
    }
}

impl Iterator for RawSchematic {
    type Item = NumberOrSymbol;
    fn next(&mut self) -> Option<NumberOrSymbol> {
        self.0.chars().next()?;
        let mut c = self.0.remove(0);
        if c == '\n' {
            self.1 = 0;
            self.2 += 1;
            self.0.chars().next()?;
            c = self.0.remove(0);
        }
        if c == '.' || !c.is_ascii_digit() {
            let s = Some(NumberOrSymbol {
                value: c.to_string(),
                row_index: self.1,
                col_index: self.2,
            });
            self.1 += 1;
            return s;
        } else if c.is_ascii_digit() {
            let mut number = String::from("");
            loop {
                number.push(c);
                if self.0.chars().next()?.is_ascii_digit() {
                    c = self.0.remove(0);
                } else {
                    break;
                }
            }
            let count = number.len();
            let n = Some(NumberOrSymbol {
                value: number,
                row_index: self.1,
                col_index: self.2,
            });
            self.1 += count;
            return n;
        } else {
            None
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let schematic = RawSchematic::new(input);
    let nos: Vec<_> = schematic.collect();
    let numbers: Vec<_> = nos.iter().filter(|n| n.is_number()).collect();
    let symbols: Vec<_> = nos.iter().filter(|n| n.is_symbol()).collect();
    let mut sum = 0;
    for a in numbers.iter() {
        for b in symbols.iter() {
            if a.is_adjacent(b) {
                sum += a.parsed_value();
            }
        }
    }
    sum
}

pub fn part2(input: &str) -> i32 {
    let schematic = RawSchematic::new(input);
    let nos: Vec<_> = schematic.collect();
    let gears: Vec<_> = nos.iter().filter(|n| n.is_gear()).collect();
    let numbers: Vec<_> = nos.iter().filter(|n| n.is_number()).collect();
    let mut sum = 0;
    for a in gears.iter() {
        let mut gear_adjency = Vec::new();
        for b in numbers.iter() {
            if b.is_adjacent(a) {
                gear_adjency.push(b);
            }
        }
        if gear_adjency.len() == 2 {
            sum += gear_adjency.get(0).unwrap().parsed_value() * gear_adjency.get(1).unwrap().parsed_value();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::{
        day3::{part1, part2},
        utils,
    };

    #[test]
    fn test_day3_part1_example() {
        let data = utils::read_entire_file("./data/day3.example.txt");
        assert_eq!(part1(&data), 4361);
    }

    #[test]
    fn test_day3_part2_example() {
        let data = utils::read_entire_file("./data/day3.example.txt");
        assert_eq!(part2(&data), 467835);
    }
}
