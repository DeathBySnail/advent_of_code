pub struct HeightMap {
    pub width: usize,
    pub height: usize,

    pub heightmap: Vec<u32>,
}

impl HeightMap {
    pub fn position(&self, index: usize) -> (i32, i32) {
        ((index % self.width) as i32, (index / self.width) as i32)
    }

    pub fn index(&self, pos: &(i32, i32)) -> usize {
        pos.1 as usize * self.width + pos.0 as usize
    }

    pub fn neighbors(&self, index: usize, with_diagonals: bool) -> Vec<usize> {
        let pos = self.position(index);

        let neighbor_offsets: Vec<(i32, i32)> = vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, 1),
            (1, -1),
        ];
        neighbor_offsets
            .iter()
            .take(if with_diagonals { 8 } else { 4 })
            .map(|n| (n.0 + pos.0, n.1 + pos.1))
            .filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < self.width as i32 && p.1 < self.height as i32)
            .map(|p| self.index(&p))
            .collect()
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
