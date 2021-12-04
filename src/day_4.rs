use std::slice::Chunks;

const BOARDSIZE: usize = 5;
struct Data {
    drawn_numbers: Vec<i32>,
    boards: Vec<Board>,
}

impl Data {
    pub fn score(&self) -> i32 {
        if let Some((_, winning_draw, unmarked_score)) = self.winning_board() {
            return unmarked_score * winning_draw;
        }
        0
    }

    pub fn losing_score(&self) -> i32 {
        if let Some((_, winning_draw, unmarked_score)) = self.losing_board() {
            return unmarked_score * winning_draw;
        }
        0
    }

    fn winning_board(&self) -> Option<(&Board, i32, i32)> {
        for (i, n) in self.drawn_numbers.iter().enumerate() {
            let drawn = &self.drawn_numbers[0..i + 1];

            for b in &self.boards {
                if b.is_won(drawn) {
                    return Some((b, *n, b.unmarked_score(drawn)));
                }
            }
        }

        None
    }

    fn losing_board(&self) -> Option<(&Board, i32, i32)> {
        let mut last_board: usize = 0;
        for (i, _) in self.drawn_numbers.iter().enumerate() {
            let drawn = &self.drawn_numbers[0..i + 1];

            let mut lost_boards: Vec<usize> = vec![];
            for (i2, b) in self.boards.iter().enumerate() {
                if !lost_boards.contains(&i2) && !b.is_won(drawn) {
                    lost_boards.push(i2);
                }
            }

            if lost_boards.len() == 1 {
                last_board = *lost_boards.last().unwrap();
            } else if lost_boards.is_empty() {
                let last_board = &self.boards[last_board];
                return Some((
                    last_board,
                    self.drawn_numbers[i],
                    last_board.unmarked_score(drawn),
                ));
            }
        }

        None
    }
}

impl std::str::FromStr for Data {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        let mut boards = lines[2..].chunks(BOARDSIZE + 1);

        Ok(Data {
            drawn_numbers: lines[0]
                .split(',')
                .map(|s| i32::from_str(s).unwrap())
                .collect(),
            boards: Board::from_chunks(&mut boards),
        })
    }
}

struct Board {
    numbers: Vec<Vec<i32>>,
}

impl Board {
    pub fn is_won(&self, drawn: &[i32]) -> bool {
        // check rows
        for r in 0..BOARDSIZE {
            if Board::is_winning_line(drawn, &self.numbers[r]) {
                return true;
            }
        }

        // check columns
        for c in 0..BOARDSIZE {
            let mut column: [i32; BOARDSIZE] = [0; BOARDSIZE];
            for r in 0..BOARDSIZE {
                column[r] = self.numbers[r][c];
            }

            if Board::is_winning_line(drawn, &column) {
                return true;
            }
        }
        false
    }

    pub fn unmarked_score(&self, drawn: &[i32]) -> i32 {
        self.numbers.iter().fold(0, |acc, r| {
            acc + r
                .iter()
                .fold(0, |acc, n| acc + if !drawn.contains(n) { *n } else { 0 })
        })
    }

    fn is_winning_line(drawn: &[i32], line: &[i32]) -> bool {
        line.iter().all(|n| drawn.contains(n))
    }
    fn from_lines(lines: &[&str]) -> Board {
        use std::str::FromStr;
        Board {
            numbers: lines[0..BOARDSIZE]
                .iter()
                .map(|r| {
                    r.split_whitespace()
                        .map(|s| i32::from_str(s).unwrap())
                        .collect()
                })
                .collect(),
        }
    }
    fn from_chunks(chunks: &mut Chunks<&str>) -> Vec<Board> {
        chunks.map(|c| Board::from_lines(c)).collect()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn day_4_example() {
        let file = include_str!("../input/day_4_example.txt");

        let data = file.parse::<super::Data>().unwrap();
        let result = data.score();
        assert_eq!(4512, result);

        let result2 = data.losing_score();
        assert_eq!(1924, result2);
    }

    #[test]
    fn day_4() {
        let file = include_str!("../input/day_4.txt");

        let data = file.parse::<super::Data>().unwrap();
        let result = data.score();
        assert_eq!(74320, result);

        let result2 = data.losing_score();
        assert_eq!(17884, result2);
    }
}
