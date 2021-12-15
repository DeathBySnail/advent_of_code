use super::common::HeightMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse ordering
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn lowest_risk(start: usize, goal: usize, grid: &HeightMap) -> Option<usize> {
    let mut dist: Vec<_> = (0..(grid.width * grid.height))
        .map(|_| usize::MAX)
        .collect();
    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost <= dist[position] {
            for n in grid.neighbors(position, false) {
                let next = State {
                    cost: cost + grid.heightmap[n] as usize,
                    position: n,
                };
                if next.cost < dist[next.position] {
                    heap.push(next);
                    dist[next.position] = next.cost;
                }
            }
        }
    }
    None
}

fn expand_heightmap(input: &HeightMap, expanded_size: usize) -> HeightMap {
    let mut expanded = HeightMap {
        width: input.width * expanded_size,
        height: input.height * expanded_size,
        heightmap: vec![u32::MAX; input.width * input.height * expanded_size * expanded_size],
    };

    for (i, &h) in input.heightmap.iter().enumerate() {
        let (ix, iy) = input.position(i);
        for y_mul in 0..expanded_size {
            let y_offset = y_mul * input.height;
            for x_mul in 0..expanded_size {
                let x_offset = x_mul * input.width;
                let newpos = (ix as usize + x_offset, iy as usize + y_offset);
                let new_index = newpos.1 * input.width * (expanded_size) + newpos.0;
                let new_h = (h + x_mul as u32 + y_mul as u32 - 1) % 9 + 1;
                expanded.heightmap[new_index] = new_h;
            }
        }
    }

    expanded
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let data = include_str!("../input/day_15_example.txt")
            .parse::<super::HeightMap>()
            .unwrap();
        assert_eq!(
            40,
            super::lowest_risk(0, (data.width * data.height) - 1, &data).unwrap()
        );

        let expanded = super::expand_heightmap(&data, 5);

        assert_eq!(
            315,
            super::lowest_risk(0, (expanded.width * expanded.height) - 1, &expanded).unwrap()
        );
    }

    #[test]
    fn actual() {
        let data = include_str!("../input/day_15.txt")
            .parse::<super::HeightMap>()
            .unwrap();
        assert_eq!(
            403,
            super::lowest_risk(0, (data.width * data.height) - 1, &data).unwrap()
        );

        let expanded = super::expand_heightmap(&data, 5);
        assert_eq!(
            2840,
            super::lowest_risk(0, (expanded.width * expanded.height) - 1, &expanded).unwrap()
        );
    }
}
