mod common;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

#[cfg(test)]
mod tests {
    #[test]
    fn day_1() {
        use super::day_1;
        let file = include_str!("../input/day_1.txt");

        let int_vec: Vec<u32> = file.lines().map(|x| x.parse::<u32>().unwrap()).collect();
        let result = day_1::sonar_sweep(&int_vec, 1);
        assert_eq!(1527, result);

        let result_extra = day_1::sonar_sweep(&int_vec, 3);
        assert_eq!(1575, result_extra);
    }
}
