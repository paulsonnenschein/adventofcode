use grid::Grid;
use std::fmt::{Debug, Formatter, Write};
use std::ops::{Index, IndexMut};

pub fn parse(input: &str) -> Grid<char> {
    let content = input.trim().lines().flat_map(|line| line.chars()).collect();
    Grid::from_vec(content, input.lines().next().unwrap().len())
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn apply(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (pos.0.overflowing_sub(1).0, pos.1),
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1.overflowing_sub(1).0),
        }
    }
}

impl Debug for BeamSpot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let num_beams = self.0.iter().filter(|s| **s).count();
        let c = match num_beams {
            0 => ' ',
            1 => {
                if self.0[0] {
                    '^'
                } else if self.0[1] {
                    '>'
                } else if self.0[2] {
                    'V'
                } else {
                    '<'
                }
            }
            _ => (b'0' + num_beams as u8) as char,
        };
        f.write_char(c)
    }
}

#[derive(Default)]
struct BeamSpot([bool; 4]);

impl Index<Direction> for BeamSpot {
    type Output = bool;

    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::Up => &self.0[0],
            Direction::Right => &self.0[1],
            Direction::Down => &self.0[2],
            Direction::Left => &self.0[3],
        }
    }
}

impl IndexMut<Direction> for BeamSpot {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            Direction::Up => &mut self.0[0],
            Direction::Right => &mut self.0[1],
            Direction::Down => &mut self.0[2],
            Direction::Left => &mut self.0[3],
        }
    }
}

pub fn part1(input: &Grid<char>) -> usize {
    count_beams_starting_at((0, 0), Direction::Right, input)
}

fn draw_beams(pos: (usize, usize), dir: Direction, input: &Grid<char>, beams: &mut Grid<BeamSpot>) {
    if input.get(pos.0, pos.1).is_none() {
        return;
    }
    match input[pos] {
        '.' => {
            follow_direction(pos, dir, input, beams);
        }
        '|' => {
            if dir == Direction::Up || dir == Direction::Down {
                follow_direction(pos, dir, input, beams);
            } else {
                follow_direction(pos, Direction::Up, input, beams);
                follow_direction(pos, Direction::Down, input, beams);
            }
        }
        '-' => {
            if dir == Direction::Left || dir == Direction::Right {
                follow_direction(pos, dir, input, beams);
            } else {
                follow_direction(pos, Direction::Left, input, beams);
                follow_direction(pos, Direction::Right, input, beams);
            }
        }
        '/' => {
            let new_dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
            };
            follow_direction(pos, new_dir, input, beams);
        }
        '\\' => {
            let new_dir = match dir {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
            };
            follow_direction(pos, new_dir, input, beams);
        }
        _ => unreachable!(),
    }
}

fn follow_direction(
    pos: (usize, usize),
    dir: Direction,
    input: &Grid<char>,
    beams: &mut Grid<BeamSpot>,
) {
    if !beams[pos][dir] {
        beams[pos][dir] = true;
        draw_beams(dir.apply(pos), dir, input, beams);
    }
}

fn count_beams_starting_at(pos: (usize, usize), dir: Direction, input: &Grid<char>) -> usize {
    let mut beams = Grid::<BeamSpot>::new(input.rows(), input.cols());
    draw_beams(pos, dir, input, &mut beams);

    beams
        .iter()
        .filter(|bs| bs.0[0] || bs.0[1] || bs.0[2] || bs.0[3])
        .count()
}

pub fn part2(input: &Grid<char>) -> usize {
    // right
    (0..input.rows())
        .map(|row| count_beams_starting_at((row, 0), Direction::Right, input))
        .chain(
            // left
            (0..input.rows()).map(|row| {
                count_beams_starting_at((row, input.cols() - 1), Direction::Left, input)
            }),
        )
        .chain(
            // down
            (0..input.cols()).map(|col| count_beams_starting_at((0, col), Direction::Down, input)),
        )
        .chain(
            // up
            (0..input.cols())
                .map(|col| count_beams_starting_at((input.rows() - 1, col), Direction::Up, input)),
        )
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run16() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
