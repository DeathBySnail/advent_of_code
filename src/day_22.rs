use regex::Regex;
use std::collections::HashMap;
type Point = (i64, i64, i64);
type Rect = (Point, Point);

#[derive(Hash)]
struct Step {
    min: Point,
    max: Point,
    on: bool,
}

fn parse_step(step: &str) -> Step {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(\w+) x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)").unwrap();
    }
    let caps = RE.captures(step).unwrap();

    let parser = |m: &str| m.parse::<i64>().unwrap();

    Step {
        min: (parser(&caps[2]), parser(&caps[4]), parser(&caps[6])),
        max: (parser(&caps[3]), parser(&caps[5]), parser(&caps[7])),
        on: &caps[1] == "on",
    }
}

fn parse_steps(instructions: &str, max_steps: usize) -> Vec<Step> {
    instructions
        .lines()
        .take(max_steps)
        .map(parse_step)
        .collect()
}

fn run_step(step: &Step, map: &mut HashMap<Point, bool>) {
    for x in step.min.0..=step.max.0 {
        for y in step.min.1..=step.max.1 {
            for z in step.min.2..=step.max.2 {
                *map.entry((x, y, z)).or_default() = step.on;
            }
        }
    }
}

fn run_steps(steps: &[Step]) -> usize {
    let mut map = HashMap::default();

    for step in steps.iter() {
        run_step(step, &mut map);
    }

    map.iter().filter(|(k, &v)| v).count()
}

fn part_2(steps: &[Step]) -> i64 {
    let mut cubes: HashMap<Rect, i64> = HashMap::default();
    for step in steps.iter() {
        let newsign = if step.on { 1 } else { -1 };
        let new_cuboid = (step.min, step.max);
        let mut new_cuboids: HashMap<Rect, i64> = HashMap::default();

        for (k, v) in &cubes {
            let cur_sign = v;

            let min_point = (
                new_cuboid.0 .0.max(k.0 .0),
                new_cuboid.0 .1.max(k.0 .1),
                new_cuboid.0 .2.max(k.0 .2),
            );
            let max_point = (
                new_cuboid.1 .0.min(k.1 .0),
                new_cuboid.1 .1.min(k.1 .1),
                new_cuboid.1 .2.min(k.1 .2),
            );
            let tmp_cuboid = (min_point, max_point);

            // remove intersections
            if min_point.0 <= max_point.0
                && min_point.1 <= max_point.1
                && min_point.2 <= max_point.2
            {
                *new_cuboids.entry(tmp_cuboid).or_default() -= cur_sign;
            }
        }

        if newsign == 1 {
            *new_cuboids.entry(new_cuboid).or_default() += newsign;
        }

        for (k, v) in new_cuboids {
            *cubes.entry(k).or_default() += v;
        }
    }
    cubes
        .iter()
        .map(|(k, &v)| {
            ((k.1 .0 - k.0 .0 + 1) * (k.1 .1 - k.0 .1 + 1) * (k.1 .2 - k.0 .2 + 1) * v) as i64
        })
        //.inspect(|v| print!("{:?} + ", v))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instructions = parse_steps(include_str!("../input/day_22_example.txt"), 20);
        assert_eq!(590784, run_steps(&instructions));
    }

    #[test]
    fn example_2() {
        let instructions = parse_steps(include_str!("../input/day_22_example_2.txt"), 2000);
        assert_eq!(2758514936282235, part_2(&instructions));
    }

    #[test]
    fn actual() {
        let instructions = parse_steps(include_str!("../input/day_22.txt"), 20);
        assert_eq!(524792, run_steps(&instructions));
    }

    #[test]
    fn actual_2() {
        let instructions = parse_steps(include_str!("../input/day_22.txt"), 2000);
        assert_eq!(1213461324555691, part_2(&instructions));
    }
}
