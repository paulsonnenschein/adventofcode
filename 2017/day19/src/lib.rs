#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn process_input(input: &str) -> (String, i32) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let starting_position = map.first()
        .unwrap()
        .iter()
        .position(|&ch| ch == '|')
        .unwrap();

    let mut current_position = (starting_position, 0usize);
    let mut current_direction = Direction::Down;

    let mut result = String::new();
    let mut steps = 1;
    loop {
        current_position = match current_direction {
            Direction::Up => (current_position.0, current_position.1 - 1),
            Direction::Down => (current_position.0, current_position.1 + 1),
            Direction::Left => (current_position.0 - 1, current_position.1),
            Direction::Right => (current_position.0 + 1, current_position.1),
        };

        let next_char = map[current_position.1][current_position.0];

        match next_char {
            'A'...'Z' => result.push(next_char),
            '|' | '-' => (),
            '+' => {
                current_direction = match current_direction {
                    Direction::Up | Direction::Down => {
                        if is_empty_tile(
                            map.get(current_position.1)
                                .and_then(|row| row.get(current_position.0 + 1)),
                        ) {
                            Direction::Left
                        } else {
                            Direction::Right
                        }
                    }
                    Direction::Left | Direction::Right => {
                        if is_empty_tile(
                            map.get(current_position.1 + 1)
                                .and_then(|row| row.get(current_position.0)),
                        ) {
                            Direction::Up
                        } else {
                            Direction::Down
                        }
                    }
                };
            }
            _ => break,
        }

        steps += 1;
    }

    (result, steps)
}

fn is_empty_tile(input: Option<&char>) -> bool {
    if let Some(input) = input {
        match *input {
            ' ' | '\n' => true,
            _ => false,
        }
    } else {
        true
    }
}
