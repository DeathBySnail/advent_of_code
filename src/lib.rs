mod common;
mod day_1;

#[cfg(test)]
mod tests {
    use super::common;

    #[test]
    fn day_1() {
        use super::day_1;
        use std::io::BufRead;
        let file = common::buffer_read("./input/day_1.txt");
        match file {
            Ok(input) => {
                let int_vec: Vec<i32> = input
                    .lines()
                    .map(|x| x.unwrap().parse::<i32>().unwrap())
                    .collect();
                let result = day_1::sonar_sweep(&int_vec, 1);
                assert_eq!(1527, result);

                let result_extra = day_1::sonar_sweep(&int_vec, 3);
                assert_eq!(1575, result_extra);
            }
            Err(e) => panic!("error: {}", e),
        }
    }
}
