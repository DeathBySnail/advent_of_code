use super::common::HeightMap;

fn parse_map(input: &str) -> HeightMap {
    HeightMap {
        width: input.lines().next().unwrap().len(),
        height: input.lines().count(),
        heightmap: input
            .chars()
            .filter_map(|c| match c {
                '>' => Some(1),
                'v' => Some(2),
                '.' => Some(0),
                _ => None,
            })
            .collect(),
    }
}

fn simulate(map: &mut HeightMap, max_steps: u32) -> u32 {
    let mut step_count = 0;

    let mut to_move: Vec<(usize, usize)> = Vec::with_capacity(map.width * map.height);
    loop {
        to_move.clear();
        let mut moved_elements = 0;
        // right movers first
        for y in 0..map.height as i32 {
            for x in 0..map.width as i32 {
                let cur_index = map.index(&(x, y));
                let value = map.heightmap[cur_index];
                if let Some(goal) = match value {
                    1 => Some(((x + 1) % map.width as i32, y)),
                    //2 => Some((x, (y + 1) % map.height as i32)),
                    _ => None,
                } {
                    let goal_index = map.index(&goal);
                    if map.heightmap[goal_index] == 0 {
                        to_move.push((cur_index, goal_index));
                    }
                }
            }
        }

        for &(cur, goal) in to_move.iter() {
            let val = map.heightmap[cur];
            map.heightmap[cur] = 0;
            map.heightmap[goal] = val;
            moved_elements += 1;
        }
        to_move.clear();

        // down movers next (checking for conflicts)
        for y in 0..map.height as i32 {
            for x in 0..map.width as i32 {
                let cur_index = map.index(&(x, y));
                let value = map.heightmap[cur_index];
                if let Some(goal) = match value {
                    //1 => Some(((x + 1) % map.width as i32, y)),
                    2 => Some((x, (y + 1) % map.height as i32)),
                    _ => None,
                } {
                    let goal_index = map.index(&goal);
                    if map.heightmap[goal_index] == 0 {
                        to_move.push((cur_index, goal_index));
                    }
                }
            }
        }

        for &(cur, goal) in to_move.iter() {
            let val = map.heightmap[cur];
            map.heightmap[cur] = 0;
            map.heightmap[goal] = val;
            moved_elements += 1;
        }
        if moved_elements == 0 || step_count >= max_steps {
            break;
        } else {
            step_count += 1;
        }
    }
    step_count + 1
}

fn draw_map(map: &HeightMap) {
    for y in 0..map.height as i32 {
        for x in 0..map.width as i32 {
            let idx = map.index(&(x, y));
            let token = match map.heightmap[idx] {
                1 => '>',
                2 => 'v',
                _ => '.',
            };
            print!("{}", token);
        }
        print!("\n");
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instructions = include_str!("../input/day_25_example.txt");
        //let expected_1 = include_str!("../input/day_25_example_1_step.txt");
        let mut map = parse_map(instructions);
        // let expected_map = parse_map(expected_1);
        // simulate(&mut map, 0);

        // println!("expected: ");
        // draw_map(&expected_map);
        // println!("actual: ");
        // draw_map(&map);

        // for y in 0..map.height as i32 {
        //     for x in 0..map.width as i32 {
        //         let index = map.index(&(x, y));
        //         assert_eq!(expected_map.heightmap[index], map.heightmap[index]);
        //     }
        // }

        assert_eq!(58, simulate(&mut map, u32::MAX));
    }

    #[test]
    fn actual() {
        let instructions = include_str!("../input/day_25.txt");
        //let expected_1 = include_str!("../input/day_25_example_1_step.txt");
        let mut map = parse_map(instructions);
        // let expected_map = parse_map(expected_1);
        // simulate(&mut map, 0);

        // println!("expected: ");
        // draw_map(&expected_map);
        // println!("actual: ");
        // draw_map(&map);

        // for y in 0..map.height as i32 {
        //     for x in 0..map.width as i32 {
        //         let index = map.index(&(x, y));
        //         assert_eq!(expected_map.heightmap[index], map.heightmap[index]);
        //     }
        // }

        assert_eq!(498, simulate(&mut map, u32::MAX));
    }
}
