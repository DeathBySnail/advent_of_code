pub enum Command {
    Forward(i128),
    Up(i128),
    Down(i128),
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command: Vec<&str> = s.split_whitespace().collect();
        let distance = command[1].parse::<i128>().unwrap();

        match command[0] {
            "forward" => Ok(Command::Forward(distance)),
            "up" => Ok(Command::Up(distance)),
            "down" => Ok(Command::Down(distance)),
            _ => Err(format!("'{}' is not a valid value for Command", s)),
        }
    }
}

pub fn get_final_position(input: &[Command]) -> i128 {
    let mut horizontal_position: i128 = 0;
    let mut depth: i128 = 0;
    let mut aim: i128 = 0;
    for command in input {
        match command {
            Command::Forward(x) => {
                horizontal_position += x;
                depth += x * aim;
            }
            Command::Up(x) => aim -= x,
            Command::Down(x) => aim += x,
        }
    }

    horizontal_position * depth
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_2() {
        let file = include_str!("../input/day_2.txt");
        let command_vec: Vec<super::Command> = file
            .lines()
            .map(|x| x.parse::<super::Command>().unwrap())
            .collect();

        let position = super::get_final_position(&command_vec);
        assert_eq!(1781819478, position);
    }
}
