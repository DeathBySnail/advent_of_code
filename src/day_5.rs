use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::str::FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            Some((x, y)) => Ok(Point {
                x: x.trim().parse::<i32>().unwrap(),
                y: y.trim().parse::<i32>().unwrap(),
            }),
            None => Err("Failed to parse point".to_string()),
        }
    }
}

struct Line {
    pub start: Point,
    pub end: Point,
}

impl std::str::FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("->") {
            Some((x, y)) => Ok(Line {
                start: x.parse::<Point>().unwrap(),
                end: y.parse::<Point>().unwrap(),
            }),
            None => Err("Failed to parse line".to_string()),
        }
    }
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    pub fn is_diagonal(&self) -> bool {
        let bounds = self.bounds();
        bounds.1.x - bounds.0.x == bounds.1.y - bounds.0.y
    }

    pub fn bounds(&self) -> (Point, Point) {
        (
            Point {
                x: min(self.start.x, self.end.x),
                y: min(self.start.y, self.end.y),
            },
            Point {
                x: max(self.start.x, self.end.x),
                y: max(self.start.y, self.end.y),
            },
        )
    }
}

struct Lines {
    pub lines: Vec<Line>,
}

impl std::str::FromStr for Lines {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Lines {
            lines: s.lines().map(|s| s.parse::<Line>().unwrap()).collect(),
        })
    }
}

struct Grid {
    positions: HashMap<Point, u32>,
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            positions: HashMap::default(),
        }
    }
}

impl Grid {
    pub fn apply_line(&mut self, line: &Line, include_diagonal: bool) {
        if line.is_vertical() {
            for y in min(line.start.y, line.end.y)..=max(line.start.y, line.end.y) {
                self.apply_point(&Point { x: line.start.x, y });
            }
        } else if line.is_horizontal() {
            for x in min(line.start.x, line.end.x)..=max(line.start.x, line.end.x) {
                self.apply_point(&Point { x, y: line.start.y });
            }
        } else if include_diagonal && line.is_diagonal() {
            let bounds = line.bounds();
            let mut x = line.start.x;
            let mut y = line.start.y;
            let x_step = if line.end.x > line.start.x { 1 } else { -1 };
            let y_step = if line.end.y > line.start.y { 1 } else { -1 };

            let point_count = bounds.1.x - bounds.0.x;
            for _ in 0..point_count + 1 {
                self.apply_point(&Point { x, y });

                x += x_step;
                y += y_step;
            }
        }
    }

    pub fn apply_point(&mut self, point: &Point) {
        *self.positions.entry(*point).or_insert(0) += 1;
    }

    pub fn num_overlaps(&self, threshold: u32) -> u32 {
        self.positions
            .iter()
            .fold(0, |acc, e| acc + if e.1 >= &threshold { 1 } else { 0 })
    }
}

fn get_overlaps(lines: &Lines, threshold: u32, include_diagonal: bool) -> u32 {
    let mut grid = Grid::default();

    for l in lines.lines.iter() {
        grid.apply_line(l, include_diagonal);
    }
    grid.num_overlaps(threshold)
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let file = include_str!("../input/day_5_example.txt");

        let lines = file.parse::<super::Lines>().unwrap();

        let overlaps = super::get_overlaps(&lines, 2, false);
        assert_eq!(5, overlaps);

        let overlaps_diagonal = super::get_overlaps(&lines, 2, true);
        assert_eq!(12, overlaps_diagonal);
    }

    #[test]
    fn solution() {
        let file = include_str!("../input/day_5.txt");

        let lines = file.parse::<super::Lines>().unwrap();

        let overlaps = super::get_overlaps(&lines, 2, false);
        assert_eq!(4993, overlaps);

        let overlaps_diagonal = super::get_overlaps(&lines, 2, true);
        assert_eq!(21101, overlaps_diagonal);
    }
}
