pub fn sonar_sweep(input: &[u32], sweep_range: usize) -> i32 {
    let mut depth_increases = 0;

    let mut prev_depth = input[0];

    for i in 1..input.len() {
        let mut accum_depth = input[i];
        for j in 1..sweep_range {
            if i + j < input.len() {
                accum_depth += input[i + j];
            }
        }

        if accum_depth > prev_depth {
            depth_increases += 1;
        }
        prev_depth = accum_depth;
    }

    depth_increases
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_1() {
        use super::sonar_sweep;
        let file = include_str!("../input/day_1.txt");

        let int_vec: Vec<u32> = file.lines().map(|x| x.parse::<u32>().unwrap()).collect();
        let result = sonar_sweep(&int_vec, 1);
        assert_eq!(1527, result);

        let result_extra = sonar_sweep(&int_vec, 3);
        assert_eq!(1575, result_extra);
    }
}
