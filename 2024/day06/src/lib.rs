use pathfinding::matrix::directions::DIRECTIONS_4;
use pathfinding::matrix::Matrix;
use std::collections::HashSet;

// refactored based on this: https://todd.ginsberg.com/post/advent-of-code/2024/day6/

pub fn parse(input: &str) -> (Matrix<char>, (usize, usize)) {
    let mat = Matrix::from_rows(input.trim().lines().map(|line| line.chars())).unwrap();
    let initial_pos = mat
        .items()
        .filter_map(|(pos, c)| if *c == '^' { Some(pos) } else { None })
        .next()
        .unwrap();

    (mat, initial_pos)
}

const DIRECTIONS: &[(isize, isize); 4] = &DIRECTIONS_4;

fn traverse(input: &Matrix<char>, initial_pos: &(usize, usize)) -> (HashSet<(usize, usize)>, bool) {
    let mut current_direction = 3usize;

    let mut seen_positions = HashSet::new();
    seen_positions.insert((*initial_pos, current_direction));

    let mut loops = false;

    let mut steps = input
        .in_direction(*initial_pos, DIRECTIONS[current_direction])
        .peekable();

    'outer: while let Some(step) = steps.next() {
        if !seen_positions.insert((step, current_direction)) {
            loops = true;
            break;
        }

        while steps.peek().map(|pos| input[*pos]) == Some('#') {
            current_direction = (current_direction + 1) % DIRECTIONS.len();
            if !seen_positions.insert((step, current_direction)) {
                loops = true;
                break 'outer;
            }
            steps = input
                .in_direction(step, DIRECTIONS[current_direction])
                .peekable();
        }
    }

    (
        seen_positions.into_iter().map(|(pos, _dir)| pos).collect(),
        loops,
    )
}

pub fn part1((input, initial_pos): &(Matrix<char>, (usize, usize))) -> usize {
    traverse(input, initial_pos).0.len()
}

pub fn part2((input, initial_pos): &(Matrix<char>, (usize, usize))) -> usize {
    traverse(input, initial_pos)
        .0
        .into_iter()
        .filter(|pos| pos != initial_pos)
        .filter(|pos| {
            let mut input_clone = input.clone();
            input_clone[*pos] = '#';
            traverse(&input_clone, initial_pos).1
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run06() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("part1: {:?}", part1(&parsed));
        println!("part2: {:?}", part2(&parsed));
    }
}
