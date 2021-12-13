use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Clone)]
struct Grid {
    points: HashMap<(i32, i32), i32>,
    width: usize,
    height: usize,
}
struct Data {
    grid: Grid,
    folds: Vec<Fold>,
}

impl std::str::FromStr for Data {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut x_max, mut y_max) = (0, 0);
        let mut points: Vec<(i32, i32)> = Vec::with_capacity(1000);
        let mut folds: Vec<Fold> = Vec::with_capacity(12);
        for l in s.lines() {
            if l.contains(',') {
                let (x, y) = l.split_once(',').ok_or("")?;
                let point = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
                x_max = x_max.max(point.0 + 1);
                y_max = y_max.max(point.1 + 1);
                points.push(point);
            } else if l.contains("fold") {
                let fold = l.trim_start_matches("fold along ");
                let (axis_str, coord_str) = fold.split_once('=').ok_or("")?;
                let coord = coord_str.parse::<usize>().unwrap();
                let fold = if axis_str.starts_with('x') {
                    Fold::X(coord)
                } else {
                    Fold::Y(coord)
                };

                folds.push(fold);
            }
        }
        let mut data = Data {
            grid: Grid {
                width: x_max as usize,
                height: y_max as usize,
                points: HashMap::default(),
            },
            folds,
        };

        for p in points {
            data.grid.points.entry(p).or_insert(1);
        }

        Ok(data)
    }
}

fn fold(heightmap: &Grid, fold: Fold) -> Grid {
    let mut out_width = heightmap.width;
    let mut out_height = heightmap.height;
    let mut points: HashMap<(i32, i32), i32> = HashMap::default();
    match fold {
        Fold::Y(coord) => {
            out_height = coord;
        }
        Fold::X(coord) => {
            out_width = coord;
        }
    }

    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            if y < out_height && x < out_width {
                if heightmap.points.contains_key(&(x as i32, y as i32)) {
                    *points.entry((x as i32, y as i32)).or_default() += 1;
                }
            } else if y > out_height {
                let dist = y - out_height;
                let folded_pos = y - dist * 2;

                if heightmap.points.contains_key(&(x as i32, y as i32)) {
                    *points.entry((x as i32, folded_pos as i32)).or_default() += 1;
                }
            } else if x > out_width {
                let dist = x - out_width;
                let folded_pos = x - dist * 2;
                if heightmap.points.contains_key(&(x as i32, y as i32)) {
                    *points.entry((folded_pos as i32, y as i32)).or_default() += 1;
                }
            }
        }
    }

    Grid {
        width: out_width,
        height: out_height,
        points,
    }
}

fn part_2(data: &Data) {
    let mut grid = data.grid.clone();
    for &f in data.folds.iter() {
        let folded = fold(&grid, f);
        grid = folded;
    }

    for y in 0..grid.height {
        for x in 0..grid.width {
            let dot = grid.points.contains_key(&(x as i32, y as i32));
            print!("{}", if dot { '.' } else { ' ' });
        }
        println!();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let data = include_str!("../input/day_13_example.txt")
            .parse::<super::Data>()
            .unwrap();

        let folded = super::fold(&data.grid, data.folds[0]);
        let dot_count = folded
            .points
            .iter()
            .fold(0, |acc, (&_, &count)| acc + if count > 0 { 1 } else { 0 });
        assert_eq!(17, dot_count);

        let folded = super::fold(&folded, data.folds[1]);
        let dot_count = folded
            .points
            .iter()
            .fold(0, |acc, (&_, &count)| acc + if count > 0 { 1 } else { 0 });
        assert_eq!(16, dot_count);
    }

    #[test]
    fn actual() {
        let data = include_str!("../input/day_13.txt")
            .parse::<super::Data>()
            .unwrap();

        let folded = super::fold(&data.grid, data.folds[0]);
        let dot_count = folded
            .points
            .iter()
            .fold(0, |acc, (&_, &count)| acc + if count > 0 { 1 } else { 0 });
        assert_eq!(837, dot_count);

        super::part_2(&data);
    }
}
