use pathfinding::prelude::{astar, directions, Matrix};

pub fn parse(input: &str) -> Matrix<u32> {
    Matrix::from_rows(
        input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node((usize, usize), Dir);

impl Node {
    fn distance(&self, other: &(usize, usize)) -> u32 {
        (self.0 .0.abs_diff(other.0) + self.0 .1.abs_diff(other.1)) as u32
    }

    fn successors_p1(&self, grid: &Matrix<u32>) -> Vec<(Node, u32)> {
        let mut result = Vec::new();

        if self.0 == (0, 0) {
            // east
            let mut cost = 0;
            for pos in grid.in_direction(self.0, directions::E).take(3) {
                cost += grid[pos];
                result.push((Node(pos, Dir::Horizontal), cost));
            }
            // south
            cost = 0;
            for pos in grid.in_direction(self.0, directions::S).take(3) {
                cost += grid[pos];
                result.push((Node(pos, Dir::Vertical), cost));
            }
        } else if self.1 == Dir::Horizontal {
            // north
            let mut cost = 0;
            for pos in grid.in_direction(self.0, directions::N).take(3) {
                cost += grid[pos];
                result.push((Node(pos, Dir::Vertical), cost));
            }
            // south
            cost = 0;
            for pos in grid.in_direction(self.0, directions::S).take(3) {
                cost += grid[pos];
                result.push((Node(pos, Dir::Vertical), cost));
            }
        } else {
            // east
            let mut cost = 0;
            for pos in grid.in_direction(self.0, directions::E).take(3) {
                cost += grid[pos];
                result.push((Node(pos, Dir::Horizontal), cost));
            }
            // west
            cost = 0;
            for pos in grid.in_direction(self.0, directions::W).take(3) {
                cost += grid[pos];
                result.push((Node(pos, Dir::Horizontal), cost));
            }
        }

        result
    }

    fn successors_p2(&self, grid: &Matrix<u32>) -> Vec<(Node, u32)> {
        let mut result = Vec::new();

        if self.0 == (0, 0) {
            // east
            let mut cost = 0;
            for (i, pos) in grid
                .in_direction(self.0, directions::E)
                .take(10)
                .enumerate()
            {
                cost += grid[pos];
                if i > 2 {
                    result.push((Node(pos, Dir::Horizontal), cost));
                }
            }
            // south
            cost = 0;
            for (i, pos) in grid
                .in_direction(self.0, directions::S)
                .take(10)
                .enumerate()
            {
                cost += grid[pos];
                if i > 2 {
                    result.push((Node(pos, Dir::Vertical), cost));
                }
            }
        } else if self.1 == Dir::Horizontal {
            // north
            let mut cost = 0;
            for (i, pos) in grid
                .in_direction(self.0, directions::N)
                .take(10)
                .enumerate()
            {
                cost += grid[pos];
                if i > 2 {
                    result.push((Node(pos, Dir::Vertical), cost));
                }
            }
            // south
            cost = 0;
            for (i, pos) in grid
                .in_direction(self.0, directions::S)
                .take(10)
                .enumerate()
            {
                cost += grid[pos];
                if i > 2 {
                    result.push((Node(pos, Dir::Vertical), cost));
                }
            }
        } else {
            // east
            let mut cost = 0;
            for (i, pos) in grid
                .in_direction(self.0, directions::E)
                .take(10)
                .enumerate()
            {
                cost += grid[pos];
                if i > 2 {
                    result.push((Node(pos, Dir::Horizontal), cost));
                }
            }
            // west
            cost = 0;
            for (i, pos) in grid
                .in_direction(self.0, directions::W)
                .take(10)
                .enumerate()
            {
                cost += grid[pos];
                if i > 2 {
                    result.push((Node(pos, Dir::Horizontal), cost));
                }
            }
        }

        result
    }
}

pub fn part1(input: &Matrix<u32>) -> u32 {
    let start = (0, 0);
    let goal = (input.rows - 1, input.columns - 1);

    let result = astar(
        &Node(start, Dir::Horizontal),
        |p| p.successors_p1(input),
        |p| p.distance(&goal),
        |p| p.0 == goal,
    )
    .unwrap();

    result.1
}

pub fn part2(input: &Matrix<u32>) -> u32 {
    let start = (0, 0);
    let goal = (input.rows - 1, input.columns - 1);

    let result = astar(
        &Node(start, Dir::Horizontal),
        |p| p.successors_p2(input),
        |p| p.distance(&goal),
        |p| p.0 == goal,
    )
    .unwrap();

    result.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run17() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
