struct Data {
    readings: Vec<i64>,
    reading_count: i64,
}

impl Data {
    pub fn power_consumption(&self) -> i64 {
        let half_count = self.reading_count / 2;

        let mut bit_position: i64 = self.readings.len() as i64 - 1;
        let mut gamma = 0;
        let mut epsilon = 0;
        for r in &self.readings {
            if r > &half_count {
                gamma |= 1 << bit_position;
            } else {
                epsilon |= 1 << bit_position;
            }
            bit_position -= 1;
        }

        gamma * epsilon
    }

    pub fn life_support(&self, input: &str) -> i64 {
        let input_vec: Vec<String> = input
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let oxygen = self.filter_values(&input_vec, true);
        let scrubber = self.filter_values(&input_vec, false);

        i64::from_str_radix(&oxygen[0], 2).unwrap() * i64::from_str_radix(&scrubber[0], 2).unwrap()
    }

    fn filter_values(&self, input_data: &[String], most_common: bool) -> Vec<String> {
        let mut possible_values: Vec<String> = input_data.to_vec();
        for i in 0..self.readings.len() {
            if possible_values.len() == 1 {
                break;
            }

            let new_readings = get_data(&possible_values.join("\n"));
            let half_count = (new_readings.reading_count) / 2;

            let mut row_value: char = if (new_readings.readings[i] > half_count) == most_common {
                '1'
            } else {
                '0'
            };

            if new_readings.reading_count % 2 == 0 && new_readings.readings[i] == half_count {
                row_value = if most_common { '1' } else { '0' };
            }

            let mut valid_lines: Vec<String> = Vec::new();
            for line in possible_values {
                if line.chars().nth(i).unwrap() == row_value {
                    valid_lines.push(line);
                }
            }

            possible_values = valid_lines;
        }

        possible_values
    }
}

fn get_data(input: &str) -> Data {
    let char_count = input.lines().next().unwrap().chars().count();
    let mut data = Data {
        readings: vec![0; char_count],
        reading_count: input.lines().count() as i64,
    };

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                data.readings[i] += 1;
            }
        }
    }

    data
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_3() {
        let file = include_str!("../input/day_3.txt");

        let data = super::get_data(file);

        assert_eq!(2261546, data.power_consumption());
        assert_eq!(6775520, data.life_support(file));
    }

    #[test]
    fn day_3_example() {
        let file = include_str!("../input/day_3_example.txt");

        let data = super::get_data(file);

        let result = data.life_support(file);
        assert_eq!(230, result);
    }
}
