use std::collections::HashMap;

enum SyntaxResult {
    Ok,
    Error(char),
    Incomplete(Vec<char>),
}

fn closing_brace(opening_brace: char) -> char {
    match opening_brace {
        '{' => '}',
        '[' => ']',
        '<' => '>',
        '(' => ')',
        _ => ' ',
    }
}

fn parse_line(line: &str) -> SyntaxResult {
    let mut stack: Vec<char> = vec![];

    for c in line.chars() {
        match c {
            '{' | '[' | '<' | '(' => stack.push(c),
            '}' | ']' | '>' | ')' => {
                let top = stack.pop();
                match top {
                    Some(top_char) => {
                        let expected = closing_brace(top_char);
                        if c != expected {
                            return SyntaxResult::Error(c);
                        }
                    }
                    None => {
                        return SyntaxResult::Error(c);
                    }
                }
            }
            _ => {}
        }
    }

    if stack.is_empty() {
        SyntaxResult::Ok
    } else {
        SyntaxResult::Incomplete(stack)
    }
}

fn score_error_results(results: &[SyntaxResult]) -> u32 {
    let mut character_counts: HashMap<char, u32> = HashMap::default();
    for r in results {
        if let SyntaxResult::Error(c) = r {
            *character_counts.entry(*c).or_default() += 1;
        }
    }

    character_counts
        .iter()
        .map(|(character, count)| -> u32 {
            let char_score = match character {
                '}' => 1197,
                ']' => 57,
                '>' => 25137,
                ')' => 3,
                _ => 0,
            };

            char_score * count
        })
        .sum()
}

fn completion_string(stack: &mut Vec<char>) -> String {
    let mut string = String::default();

    while let Some(top) = stack.pop() {
        string.push(closing_brace(top));
    }

    string
}

fn score_completion_string(string: &str) -> u64 {
    let mut score = 0;
    for c in string.chars() {
        score *= 5;

        score += match c {
            '}' => 3,
            ']' => 2,
            '>' => 4,
            ')' => 1,
            _ => 0,
        };
    }

    score
}

fn score_incomplete_results(results: &[SyntaxResult]) -> u64 {
    let mut scores: Vec<u64> = vec![];
    for result in results {
        if let SyntaxResult::Incomplete(stack) = result {
            let completion = completion_string(&mut stack.clone());
            scores.push(score_completion_string(&completion));
        }
    }

    scores.sort_unstable();
    let index = scores.len() / 2;
    *scores.get(index).unwrap()
}

fn part_1(lines: &str) -> u32 {
    let results: Vec<SyntaxResult> = lines.lines().map(|l| parse_line(l)).collect();

    score_error_results(&results)
}

fn part_2(lines: &str) -> u64 {
    let results: Vec<SyntaxResult> = lines
        .lines()
        .map(|l| parse_line(l))
        .filter(|r| matches!(r, SyntaxResult::Incomplete(_)))
        .collect();

    score_incomplete_results(&results)
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let lines = include_str!("../input/day_10_example.txt");
        assert_eq!(26397, super::part_1(lines));
        assert_eq!(288957, super::part_2(lines));
    }

    #[test]
    fn actual() {
        let lines = include_str!("../input/day_10.txt");
        assert_eq!(316851, super::part_1(lines));
        assert_eq!(2182912364, super::part_2(lines));
    }
}
