pub fn sonar_sweep(input: &[i32], sweep_range: usize) -> i32 {
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
