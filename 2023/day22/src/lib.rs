use std::collections::{HashMap, HashSet};
use std::iter::repeat;
use std::mem::swap;

#[derive(Debug, Clone)]
pub struct Brick {
    id: usize,
    start: Pos,
    end: Pos,
}

impl Brick {
    fn down(&self) -> Self {
        Brick {
            id: self.id,
            start: self.start.down(),
            end: self.end.down(),
        }
    }

    fn inside_ground(&self) -> bool {
        self.start.2 <= 0 || self.end.2 <= 0
    }

    fn positions(&self) -> impl Iterator<Item = Pos> {
        let sub = self.end.sub(self.start);
        let len = sub.0.abs() + sub.1.abs() + sub.2.abs() + 1;
        let step = sub.my_norm();
        repeat(self.start)
            .take(len as usize)
            .enumerate()
            .map(move |(i, pos)| pos.add_n(step, i as i64))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pos(i64, i64, i64);

impl Pos {
    fn down(&self) -> Self {
        Pos(self.0, self.1, self.2 - 1)
    }

    fn sub(&self, other: Self) -> Self {
        Pos(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    fn add_n(&self, other: Self, n: i64) -> Self {
        Pos(
            self.0 + other.0 * n,
            self.1 + other.1 * n,
            self.2 + other.2 * n,
        )
    }

    fn my_norm(&self) -> Self {
        Pos(
            if self.0 == 0 {
                0
            } else {
                self.0 / self.0.abs()
            },
            if self.1 == 0 {
                0
            } else {
                self.1 / self.1.abs()
            },
            if self.2 == 0 {
                0
            } else {
                self.2 / self.2.abs()
            },
        )
    }
}

pub fn parse(input: &str) -> Vec<Brick> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let (start, end) = line.split_once('~').unwrap();
            let mut split_start = start.split(',');
            let mut split_end = end.split(',');
            Brick {
                id,
                start: Pos(
                    split_start.next().unwrap().parse().unwrap(),
                    split_start.next().unwrap().parse().unwrap(),
                    split_start.next().unwrap().parse().unwrap(),
                ),
                end: Pos(
                    split_end.next().unwrap().parse().unwrap(),
                    split_end.next().unwrap().parse().unwrap(),
                    split_end.next().unwrap().parse().unwrap(),
                ),
            }
        })
        .collect()
}

pub fn part1(input: &[Brick]) -> usize {
    let mut moving_pieces = input.to_vec();
    let mut moving_pieces_next = Vec::with_capacity(input.len());
    let mut rested_pieces = HashMap::new();
    let mut required_pieces = HashSet::new();

    while !moving_pieces.is_empty() {
        // for each piece: check if moving it down would intersect with any rested piece?
        // if yes -> make it rest (store each coord -> id in rested_pieces)
        //         & if it would intersect with exactly one, remember that id in required_pieces

        // first check ground
        for brick in moving_pieces.drain(..) {
            let brick_down = brick.down();
            if brick_down.inside_ground() {
                println!("piece {} resting", brick.id);
                brick.positions().for_each(|pos| {
                    rested_pieces.insert(pos, brick.id);
                });
            } else {
                moving_pieces_next.push(brick);
            }
        }
        swap(&mut moving_pieces, &mut moving_pieces_next);

        moving_pieces.sort_by_key(|brick| brick.start.2.min(brick.end.2));

        // second check other pieces
        for brick in moving_pieces.drain(..) {
            let brick_down = brick.down();
            let intersecting_brick_ids = brick_down
                .positions()
                .filter_map(|pos| rested_pieces.get(&pos).copied())
                .collect::<HashSet<_>>();
            if !intersecting_brick_ids.is_empty() {
                if intersecting_brick_ids.len() == 1 {
                    required_pieces.insert(*intersecting_brick_ids.iter().next().unwrap());
                }

                println!("piece {} resting", brick.id);
                brick.positions().for_each(|pos| {
                    rested_pieces.insert(pos, brick.id);
                });
            } else {
                moving_pieces_next.push(brick_down);
            }
        }

        swap(&mut moving_pieces, &mut moving_pieces_next);
    }

    input.len() - required_pieces.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run22() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        //println!("{:?}", part2(&parsed));
    }

    #[test]
    fn test_brick() {
        let brick = Brick {
            id: 0,
            start: Pos(2, 2, 2),
            end: Pos(2, 2, 2),
        };
        assert_eq!(brick.positions().collect::<Vec<_>>(), vec![Pos(2, 2, 2)]);
        let brick = Brick {
            id: 1,
            start: Pos(2, 2, 2),
            end: Pos(2, 2, 3),
        };
        assert_eq!(
            brick.positions().collect::<Vec<_>>(),
            vec![Pos(2, 2, 2), Pos(2, 2, 3)]
        );
        let brick = Brick {
            id: 2,
            start: Pos(2, 2, 2),
            end: Pos(0, 2, 2),
        };
        assert_eq!(
            brick.positions().collect::<HashSet<_>>(),
            HashSet::from([Pos(0, 2, 2), Pos(1, 2, 2), Pos(2, 2, 2)])
        );
    }
}
