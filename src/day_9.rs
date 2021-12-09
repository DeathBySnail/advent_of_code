struct HeightMap {
    pub width: usize,
    pub height: usize,

    pub heightmap: Vec<u32>,
}

struct Basin {
    low_point: usize,
    size: u32,
}

impl HeightMap {
    pub fn low_point_risk_level(&self) -> u32 {
        let low_points = self.low_points();

        return low_points.iter().map(|i| self.heightmap[*i] + 1).sum();
    }

    pub fn basin_sizes(&self) -> u32 {
        let basins = self.basins();
        let mut basin_sizes: Vec<u32> = basins.iter().map(|b| b.size).collect();
        basin_sizes.sort_unstable();

        basin_sizes.iter().rev().take(3).product()
    }

    fn low_points(&self) -> Vec<usize> {
        let low_points = self
            .heightmap
            .iter()
            .enumerate()
            .filter(|(i, _)| self.is_low_point(*i))
            .map(|(i, _)| i)
            .collect();

        low_points
    }

    fn position(&self, index: usize) -> (i32, i32) {
        ((index % self.width) as i32, (index / self.width) as i32)
    }

    fn index(&self, pos: &(i32, i32)) -> usize {
        pos.1 as usize * self.width + pos.0 as usize
    }

    fn is_low_point(&self, index: usize) -> bool {
        let height = self.heightmap[index];
        for n in self.neighbors(index) {
            let n_height = self.heightmap[n];
            if n_height <= height {
                return false;
            }
        }
        true
    }

    fn neighbors(&self, index: usize) -> Vec<usize> {
        let pos = self.position(index);

        let neighbor_offsets: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        neighbor_offsets
            .iter()
            .map(|n| (n.0 + pos.0, n.1 + pos.1))
            .filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < self.width as i32 && p.1 < self.height as i32)
            .map(|p| self.index(&p))
            .collect()
    }

    fn basins(&self) -> Vec<Basin> {
        self.low_points()
            .iter()
            .map(|i| Basin {
                low_point: *i,
                size: self.basin_size(*i),
            })
            .collect()
    }

    fn basin_size(&self, low_point: usize) -> u32 {
        let mut open_list: Vec<usize> = vec![low_point];
        let mut closed_list: Vec<usize> = vec![];
        let mut size = 0;
        while !open_list.is_empty() {
            let point = open_list.pop().unwrap();
            closed_list.push(point);
            size += 1;

            for n in self.neighbors(point).iter() {
                if !open_list.contains(n) && !closed_list.contains(n) && self.heightmap[*n] != 9 {
                    open_list.push(*n);
                }
            }
        }
        size
    }
}

impl std::str::FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HeightMap {
            width: s.lines().next().unwrap().len(),
            height: s.lines().count(),
            heightmap: s.chars().filter_map(|c| c.to_digit(10)).collect(),
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let heightmap = include_str!("../input/day_9_example.txt")
            .parse::<super::HeightMap>()
            .unwrap();
        assert_eq!(15, heightmap.low_point_risk_level());
        assert_eq!(1134, heightmap.basin_sizes())
    }

    #[test]
    fn actual() {
        let heightmap = include_str!("../input/day_9.txt")
            .parse::<super::HeightMap>()
            .unwrap();
        assert_eq!(480, heightmap.low_point_risk_level());
        assert_eq!(1045660, heightmap.basin_sizes())
    }
}
