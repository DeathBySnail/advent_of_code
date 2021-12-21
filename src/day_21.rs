use std::collections::HashMap;

fn part_1(starting_pos: [u8; 2]) -> u32 {
    let mut points: [u32; 2] = [0; 2];
    let mut positions: [u8; 2] = [starting_pos[0] - 1, starting_pos[1] - 1];
    let mut last_die_value = 1;

    let mut player_index = 0;
    let mut die_rolls = 0;
    while points[0] < 1000 && points[1] < 1000 {
        for _ in 0..3 {
            positions[player_index] = (positions[player_index] + last_die_value) % 10;
            last_die_value = (last_die_value + 1) % 100;
            die_rolls += 1;
        }

        points[player_index] += positions[player_index] as u32 + 1;
        player_index = (player_index + 1) % 2;
    }

    die_rolls
        * if points[0] > points[1] {
            points[1]
        } else {
            points[0]
        }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct State {
    pos: [u8; 2],
    score: [u8; 2],
    die_sum: u8,
    roll_count: u8,
    is_p0: bool,
}

// number of wins from a given state
type WinCache = HashMap<State, u64>;

fn roll(state: &mut State, win_cache: &mut WinCache) -> u64 {
    if win_cache.contains_key(state) {
        return *win_cache.entry(*state).or_default();
    }

    if state.roll_count > 0 && state.roll_count % 3 == 0 {
        // update score
        if state.roll_count % 2 != 0 {
            state.pos[0] = (state.pos[0] + state.die_sum) % 10;
            state.score[0] = state.score[0] + state.pos[0] + 1;
        } else {
            state.pos[1] = (state.pos[1] + state.die_sum) % 10;
            state.score[1] = state.score[1] + state.pos[1] + 1;
        }
        state.die_sum = 0;
        if state.score[0] >= 21 {
            return if state.is_p0 { 1 } else { 0 };
        }
        if state.score[1] >= 21 {
            return if state.is_p0 { 0 } else { 1 };
        }
    }

    let mut wins = 0;
    for die in 1..=3 {
        let mut new_state = State {
            pos: state.pos,
            score: state.score,
            die_sum: state.die_sum + die,
            roll_count: state.roll_count + 1,
            is_p0: state.is_p0,
        };

        wins += roll(&mut new_state, win_cache);
    }

    win_cache.entry(*state).or_insert(wins);
    wins
}

fn roll_with_start_pos(starting_pos: [u8; 2], player_0: bool) -> u64 {
    let mut starting_state = State {
        pos: [starting_pos[0] - 1, starting_pos[1] - 1],
        score: [0, 0],
        die_sum: 0,
        roll_count: 0,
        is_p0: player_0,
    };

    let mut cache = HashMap::default();
    roll(&mut starting_state, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let test1 = part_1([4, 8]);
        assert_eq!(739785, test1);

        let test2 = roll_with_start_pos([4, 8], true).max(roll_with_start_pos([4, 8], false));
        assert_eq!(444356092776315, test2);
    }

    #[test]
    fn actual() {
        let test1 = part_1([8, 1]);
        assert_eq!(518418, test1);

        let test2 = roll_with_start_pos([8, 1], true).max(roll_with_start_pos([8, 1], false));
        assert_eq!(444356092776315, test2);
    }
}
