use regex::Regex;
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct State {
    pos: (i32, i32),
    vel: (i32, i32),
}

struct Target {
    min: (i32, i32),
    max: (i32, i32),
}

#[derive(Clone, Copy)]
struct EndState {
    state: State,
    start_vel: (i32, i32),
    highest_y: i32,
}

fn step_state(state: &State) -> State {
    State {
        pos: (state.pos.0 + state.vel.0, state.pos.1 + state.vel.1),
        vel: (
            if state.vel.0 > 0 { state.vel.0 - 1 } else { 0 },
            state.vel.1 - 1,
        ),
    }
}

fn on_target(pos: (i32, i32), target: &Target) -> bool {
    pos.0 >= target.min.0 && pos.0 <= target.max.0 && pos.1 >= target.min.1 && pos.1 <= target.max.1
}

fn past_target(state: &State, target: &Target) -> bool {
    (state.vel.1 < 0 && state.pos.1 < target.min.1)
        || (state.vel.0 > 0 && state.pos.0 > target.max.0)
        || (state.vel.0 <= 0 && state.pos.0 < target.min.0)
}

fn fire(state: &State, target: &Target) -> Option<EndState> {
    let mut current_state: State = *state;
    let mut highest_y = state.pos.1;
    let start_vel = state.vel;
    while !past_target(&current_state, target) {
        current_state = step_state(&current_state);
        highest_y = highest_y.max(current_state.pos.1);
        if on_target(current_state.pos, target) {
            return Some(EndState {
                state: current_state,
                start_vel,
                highest_y,
            });
        }
    }

    None
}

fn brute_force_trick_shot(target: &Target) -> Option<EndState> {
    let start_pos = (0, 0);
    let mut best_end_state: Option<EndState> = None;
    for try_dx in 0..500 {
        for try_dy in -500..500 {
            if let Some(end_state) = fire(
                &State {
                    pos: start_pos,
                    vel: (try_dx, try_dy),
                },
                target,
            ) {
                if best_end_state.is_none()
                    || best_end_state.unwrap().highest_y < end_state.highest_y
                {
                    best_end_state = Some(end_state);
                }
            }
        }
    }

    best_end_state
}

fn num_possible_start_velocities(target: &Target) -> usize {
    let start_pos = (0, 0);
    let mut end_states: HashSet<(i32, i32)> = HashSet::default();
    for try_dx in 0..500 {
        for try_dy in -500..500 {
            if let Some(end_state) = fire(
                &State {
                    pos: start_pos,
                    vel: (try_dx, try_dy),
                },
                target,
            ) {
                end_states.insert((try_dx, try_dy));
            }
        }
    }

    end_states.len()
}

fn parse_input(string: &str) -> Target {
    lazy_static! {
        static ref RE: Regex = Regex::new("x=(.+)\\.\\.(.+), y=(.+)\\.\\.(.+)").unwrap();
    }

    let caps = RE.captures(string).unwrap();

    let parser = |m: regex::Match| {
        println!("{:?}", m.as_str());
        m.as_str().parse::<i32>().unwrap()
    };
    let x_range = (caps.get(1).map_or(0, parser), caps.get(2).map_or(0, parser));
    let y_range = (caps.get(3).map_or(0, parser), caps.get(4).map_or(0, parser));

    Target {
        min: (x_range.0.min(x_range.1), y_range.0.min(y_range.1)),
        max: (x_range.0.max(x_range.1), y_range.0.max(y_range.1)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let start = State {
            pos: (0, 10),
            vel: (1, 0),
        };
        let next = step_state(&start);

        assert_eq!(1, next.pos.0);
        assert_eq!(10, next.pos.1);
        assert_eq!(0, next.vel.0);
        assert_eq!(-1, next.vel.1);

        let next = step_state(&next);
        assert_eq!(1, next.pos.0);
        assert_eq!(9, next.pos.1);
        assert_eq!(0, next.vel.0);
        assert_eq!(-2, next.vel.1);
    }

    #[test]
    fn test_trick_shot() {
        let target = parse_input("target area: x=20..30, y=-10..-5");
        let result = brute_force_trick_shot(&target);

        assert!(result.is_some());
        assert_eq!(45, result.unwrap().highest_y);

        assert_eq!(112, num_possible_start_velocities(&target));
    }

    #[test]
    fn actual_trick_shot() {
        let target = parse_input("target area: x=169..206, y=-108..-68");
        let result = brute_force_trick_shot(&target);

        assert!(result.is_some());
        assert_eq!(5778, result.unwrap().highest_y);

        assert_eq!(2576, num_possible_start_velocities(&target));
    }
}
