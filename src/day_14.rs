use std::collections::HashMap;

struct Data {
    template: String,
    pair_insertions: HashMap<(char, char), char>,
}

impl std::str::FromStr for Data {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let template = s.lines().next().ok_or("")?;
        let mut pair_insertions = HashMap::default();

        for pair_insertion in s.lines().skip(2) {
            let a = pair_insertion.chars().next().ok_or("")?;
            let b = pair_insertion.chars().nth(1).ok_or("")?;
            let c = pair_insertion.chars().nth(6).ok_or("")?;
            pair_insertions.insert((a, b), c);
        }

        Ok(Data {
            template: template.to_string(),
            pair_insertions,
        })
    }
}

fn to_pair_set(
    template: &str,
    char_frequencies: &mut HashMap<char, i64>,
) -> HashMap<(char, char), i64> {
    let vec = template.chars().collect::<Vec<char>>();
    let windows = vec.windows(2);

    let mut pair_set = HashMap::default();
    for window in windows {
        *pair_set.entry((window[0], window[1])).or_insert(0) += 1;
    }

    for c in template.chars() {
        *char_frequencies.entry(c).or_insert(0) += 1;
    }

    pair_set
}

fn step(
    data: &Data,
    pair_set: &mut HashMap<(char, char), i64>,
    char_frequencies: &mut HashMap<char, i64>,
) {
    let mut operations: Vec<(char, char, i64)> = vec![];
    for ((a, b), c) in &data.pair_insertions {
        let entry = pair_set.entry((*a, *b)).or_insert(0);
        if *entry > 0 {
            operations.push((*a, *b, -*entry));
            operations.push((*a, *c, *entry));
            operations.push((*c, *b, *entry));

            *char_frequencies.entry(*c).or_insert(0) += *entry;
        }
    }

    for (a, b, count) in operations {
        *pair_set.entry((a, b)).or_insert(0) += count;
    }
}

fn steps(
    data: &Data,
    pair_set: &mut HashMap<(char, char), i64>,
    char_frequencies: &mut HashMap<char, i64>,
    step_count: u32,
) {
    for _ in 0..step_count {
        step(data, pair_set, char_frequencies);
    }
}

fn solution(data: &Data, step_count: u32) -> i64 {
    let mut char_frequencies = HashMap::default();
    let mut pair_set = to_pair_set(&data.template, &mut char_frequencies);
    steps(data, &mut pair_set, &mut char_frequencies, step_count);

    let max = char_frequencies.values().max().unwrap();
    let min = char_frequencies.values().min().unwrap();
    max - min
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let data = include_str!("../input/day_14_example.txt")
            .parse::<super::Data>()
            .unwrap();
        assert_eq!(1588, super::solution(&data, 10));
    }

    #[test]
    fn actual() {
        let data = include_str!("../input/day_14.txt")
            .parse::<super::Data>()
            .unwrap();
        assert_eq!(3058, super::solution(&data, 10));
        assert_eq!(3447389044530, super::solution(&data, 40));
    }
}
