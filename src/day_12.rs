use std::collections::{HashMap, HashSet};

enum CaveType {
    Start,
    End,
    Big,
    Small,
}

struct Cave {
    cave_type: CaveType,
    id: usize,
    neighbors: HashSet<usize>,
}

struct Graph {
    caves: HashMap<usize, Cave>,
}

fn hasher(string: &str) -> usize {
    string
        .chars()
        .enumerate()
        .map(|(i, c)| i * 100 + c as usize)
        .sum()
}

fn cave_type(string: &str) -> CaveType {
    match string {
        "start" => CaveType::Start,
        "end" => CaveType::End,
        x if x == x.to_uppercase() => CaveType::Big,
        _ => CaveType::Small,
    }
}

impl Graph {
    pub fn add_connection(&mut self, a: &str, b: &str) {
        let (idx_a, idx_b) = (hasher(a), hasher(b));
        self.caves
            .entry(idx_a)
            .or_insert(Cave {
                id: idx_a,
                cave_type: cave_type(a),
                neighbors: HashSet::default(),
            })
            .neighbors
            .insert(idx_b);

        self.caves
            .entry(idx_b)
            .or_insert(Cave {
                id: idx_b,
                cave_type: cave_type(b),
                neighbors: HashSet::default(),
            })
            .neighbors
            .insert(idx_a);
    }

    fn paths_to_end(
        &self,
        current_index: usize,
        blocked_nodes: &mut Vec<usize>,
        part2: bool,
    ) -> u32 {
        let cave = self.caves.get(&current_index).unwrap();
        let mut part2 = part2;
        if matches!(
            cave.cave_type,
            CaveType::Start | CaveType::End | CaveType::Small
        ) {
            if part2
                && matches!(cave.cave_type, CaveType::Small)
                && blocked_nodes.contains(&current_index)
            {
                part2 = false;
            }

            blocked_nodes.push(current_index);
        }

        let mut path_count = if matches!(cave.cave_type, CaveType::End) {
            1
        } else {
            0
        };

        if !matches!(cave.cave_type, CaveType::End) {
            for r in cave.neighbors.iter() {
                let neighbor = self.caves.get(r).unwrap();
                if !matches!(neighbor.cave_type, CaveType::Start)
                    && (part2 || !blocked_nodes.contains(r))
                {
                    path_count += self.paths_to_end(*r, &mut blocked_nodes.clone(), part2);
                }
            }
        }

        path_count
    }
    fn paths_through(&self, part_2: bool) -> u32 {
        let (&startidx, _) = self
            .caves
            .iter()
            .find(|c| matches!(c.1.cave_type, CaveType::Start))
            .unwrap();

        self.paths_to_end(startidx, &mut Vec::with_capacity(12), part_2)
    }
}

impl std::str::FromStr for Graph {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Graph {
            caves: HashMap::default(),
        };

        for line in s.lines() {
            let (a, b) = line.split_once('-').unwrap();
            graph.add_connection(a, b);
        }
        Ok(graph)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let graph = include_str!("../input/day_12_example.txt")
            .parse::<super::Graph>()
            .unwrap();

        assert_eq!(10, graph.paths_through(false));
        assert_eq!(36, graph.paths_through(true));
    }

    #[test]
    fn actual() {
        let graph = include_str!("../input/day_12.txt")
            .parse::<super::Graph>()
            .unwrap();

        assert_eq!(4773, graph.paths_through(false));
        assert_eq!(116985, graph.paths_through(true));
    }
}
