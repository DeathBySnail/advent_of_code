fn fuel_cost(target: i32, pos: i32, exponential: bool) -> i32 {
    let dist = i32::abs(pos - target);
    if exponential {
        dist * (dist + 1) / 2
    } else {
        dist
    }
}
fn cheapest_position(positions: &[i32], exponential: bool, return_fuel: bool) -> i32 {
    let (smallest, largest) = (
        *positions.iter().min().unwrap(),
        *positions.iter().max().unwrap(),
    );

    let costs: Vec<i32> = (smallest..=largest)
        .into_iter()
        .map(|target| {
            positions
                .iter()
                .map(|p| fuel_cost(target, *p, exponential))
                .sum()
        })
        .collect();

    let cheapest_value = costs.iter().min().unwrap();

    if return_fuel {
        *cheapest_value
    } else {
        costs.iter().position(|e| e == cheapest_value).unwrap() as i32
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let file = include_str!("../input/day_7_example.txt");
        let data: Vec<i32> = file.split(',').map(|s| s.parse::<i32>().unwrap()).collect();

        let cheapest = super::cheapest_position(&data, false, false);
        assert_eq!(2, cheapest);

        let cheapest_exp = super::cheapest_position(&data, true, true);
        assert_eq!(168, cheapest_exp);
    }

    #[test]
    fn actual() {
        let file = include_str!("../input/day_7.txt");
        let data: Vec<i32> = file.split(',').map(|s| s.parse::<i32>().unwrap()).collect();

        let cheapest = super::cheapest_position(&data, false, false);
        assert_eq!(347, cheapest);

        let cheapest_exp = super::cheapest_position(&data, true, true);
        assert_eq!(486, cheapest_exp);
    }
}
