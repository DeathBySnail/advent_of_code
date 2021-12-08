use std::collections::HashSet;
use std::iter::FromIterator;

fn is_digit_unique(digit: &str) -> bool {
    matches!(digit.len(), 2 | 3 | 4 | 7)
}
fn part_1_count(line: &str) -> usize {
    line.split_once('|')
        .unwrap()
        .1
        .split_whitespace()
        .fold(0, |acc, x| acc + if is_digit_unique(x) { 1 } else { 0 })
}

fn part_1(file: &str) -> usize {
    file.lines().map(|l| part_1_count(l)).sum()
}

fn decode_entry(line: &str) -> u64 {
    let (signal, output) = line.split_once(" | ").unwrap();

    let splits: Vec<HashSet<u8>> = signal
        .split_whitespace()
        .map(|code| code.bytes().collect())
        .collect();

    let key_1 = splits.iter().find(|&x| x.len() == 2).unwrap();
    let key_4 = splits.iter().find(|&x| x.len() == 4).unwrap();

    let result = output
        .split_whitespace()
        .map(|code| code.bytes().collect())
        .fold(
            Vec::with_capacity(4),
            |mut result: Vec<u64>, digit: HashSet<u8>| {
                match (
                    digit.len(),
                    key_1.intersection(&digit).count(),
                    key_4.intersection(&digit).count(),
                ) {
                    (2, _, _) => result.push(1),
                    (3, _, _) => result.push(7),
                    (4, _, _) => result.push(4),
                    (5, 2, _) => result.push(3),
                    (5, _, 2) => result.push(2),
                    (5, _, _) => result.push(5),
                    (6, 1, _) => result.push(6),
                    (6, _, 4) => result.push(9),
                    (6, _, _) => result.push(0),
                    (7, _, _) => result.push(8),
                    _ => {}
                }
                result
            },
        );

    result[0] * 1000 + result[1] * 100 + result[2] * 10 + result[3]
}
fn part_2(file: &str) -> u64 {
    file.lines().map(|l| decode_entry(l)).sum()
}
#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let file = include_str!("../input/day_8_example.txt");
        let count = super::part_1(file);
        assert_eq!(26, count);
    }

    #[test]
    fn example_2() {
        let file = include_str!("../input/day_8_example_2.txt");
        let count = super::part_2(file);
        assert_eq!(5353, count);
    }

    #[test]
    fn actual() {
        let file = include_str!("../input/day_8.txt");
        let count = super::part_1(file);
        assert_eq!(284, count);
        let decoded = super::part_2(file);
        assert_eq!(284, decoded);
    }
}
