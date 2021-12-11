use super::common::HeightMap;

fn step(octopuses: &mut HeightMap) -> u32 {
    let mut flashing_octopuses = vec![];
    for (i, o) in &mut octopuses.heightmap.iter_mut().enumerate() {
        *o += 1;

        if *o > 9 {
            flashing_octopuses.push(i);
        }
    }

    let mut flash_count = 0;
    let mut flashed = vec![];
    while !flashing_octopuses.is_empty() {
        flash_count += 1;
        let octopus = flashing_octopuses.pop().unwrap();
        flashing_octopuses.extend(flash(octopuses, octopus));
        flashed.push(octopus);
    }

    for &i in flashed.iter() {
        octopuses.heightmap[i] = 0;
    }

    flash_count
}

fn flash(octopuses: &mut HeightMap, octopus: usize) -> Vec<usize> {
    let mut flashing_neighbors = vec![];
    for n in octopuses.neighbors(octopus, true).iter() {
        octopuses.heightmap[*n] += 1;
        if octopuses.heightmap[*n] == 10 {
            flashing_neighbors.push(*n);
        }
    }

    flashing_neighbors
}

fn part_1(octopuses: &mut HeightMap, flash_count: u32) -> u32 {
    let mut flashes = 0;
    for _ in 0..flash_count {
        flashes += step(octopuses);
    }

    flashes
}

fn part_2(octopuses: &mut HeightMap) -> u32 {
    let mut stepcount = 0;
    loop {
        let flashed = step(octopuses);
        stepcount += 1;
        if flashed == octopuses.heightmap.len() as u32 {
            return stepcount;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let mut octopuses = include_str!("../input/day_11_example.txt")
            .parse::<super::HeightMap>()
            .unwrap();

        assert_eq!(1656, super::part_1(&mut octopuses, 100));
    }

    #[test]
    fn example_2() {
        let mut octopuses = include_str!("../input/day_11_example.txt")
            .parse::<super::HeightMap>()
            .unwrap();

        assert_eq!(195, super::part_2(&mut octopuses));
    }

    #[test]
    fn actual() {
        let mut octopuses = include_str!("../input/day_11.txt")
            .parse::<super::HeightMap>()
            .unwrap();

        assert_eq!(1741, super::part_1(&mut octopuses, 100));
    }

    #[test]
    fn actual_2() {
        let mut octopuses = include_str!("../input/day_11.txt")
            .parse::<super::HeightMap>()
            .unwrap();

        assert_eq!(440, super::part_2(&mut octopuses));
    }
}
