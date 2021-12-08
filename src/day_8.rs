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

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let file = include_str!("../input/day_8_example.txt");
        let count = super::part_1(file);
        assert_eq!(26, count);
    }

    #[test]
    fn actual() {
        let file = include_str!("../input/day_8.txt");
        let count = super::part_1(file);
        assert_eq!(284, count);
    }
}
