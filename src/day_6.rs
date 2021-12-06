use std::collections::HashMap;

fn fish_count(state: &[u8], days: u64) -> u128 {
    let mut daymap: HashMap<u8, u128> = HashMap::default();
    for c in state {
        *daymap.entry(*c).or_insert(0) += 1;
    }

    for _ in 0..days {
        let to_spawn = *daymap.entry(0).or_default();

        *daymap.entry(0).or_default() = *daymap.entry(1).or_default();
        *daymap.entry(1).or_default() = *daymap.entry(2).or_default();
        *daymap.entry(2).or_default() = *daymap.entry(3).or_default();
        *daymap.entry(3).or_default() = *daymap.entry(4).or_default();
        *daymap.entry(4).or_default() = *daymap.entry(5).or_default();
        *daymap.entry(5).or_default() = *daymap.entry(6).or_default();
        *daymap.entry(6).or_default() = *daymap.entry(7).or_default() + to_spawn;
        *daymap.entry(7).or_default() = *daymap.entry(8).or_default();
        *daymap.entry(8).or_default() = to_spawn;
    }

    daymap.values().fold(0, |acc, x| acc + *x)
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let file = include_str!("../input/day_6_example.txt");
        let data: Vec<u8> = file.split(',').map(|s| s.parse::<u8>().unwrap()).collect();

        let after_80_days = super::fish_count(&data, 80);
        assert_eq!(5934, after_80_days);

        let after_256_days = super::fish_count(&data, 256);
        assert_eq!(26984457539, after_256_days);
    }

    #[test]
    fn day_6() {
        let file = include_str!("../input/day_6.txt");
        let data: Vec<u8> = file.split(',').map(|s| s.parse::<u8>().unwrap()).collect();

        let after_80_days = super::fish_count(&data, 80);
        assert_eq!(373378, after_80_days);

        let after_256_days = super::fish_count(&data, 256);
        assert_eq!(1682576647495, after_256_days);
    }
}
