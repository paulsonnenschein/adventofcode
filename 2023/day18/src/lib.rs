type Instruction = (Dir, i64, String);

#[derive(Copy, Clone)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

impl TryFrom<char> for Dir {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Dir::U),
            'D' => Ok(Dir::D),
            'L' => Ok(Dir::L),
            'R' => Ok(Dir::R),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
struct Point(i64, i64);

impl Point {
    fn step(&self, dir: Dir, steps: i64) -> Point {
        match dir {
            Dir::U => Point(self.0 - steps, self.1),
            Dir::D => Point(self.0 + steps, self.1),
            Dir::L => Point(self.0, self.1 - steps),
            Dir::R => Point(self.0, self.1 + steps),
        }
    }
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut split = line.split(' ');

            (
                split
                    .next()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap()
                    .try_into()
                    .unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().chars().skip(2).take(6).collect(),
            )
        })
        .collect()
}

pub fn part1(input: &[Instruction]) -> i64 {
    let mut current = Point(0, 0);
    let mut points = Vec::with_capacity(input.len() + 1);
    points.push(current);

    let mut trench_len = 0;
    for (dir, steps, _color) in input {
        current = current.step(*dir, *steps);
        points.push(current);
        trench_len += *steps;
    }

    shoelace_formula(&points) + (trench_len / 2 + 1)
}
pub fn part2(input: &[Instruction]) -> i64 {
    let mut current = Point(0, 0);
    let mut points = Vec::with_capacity(input.len() + 1);
    points.push(current);

    let mut trench_len = 0;
    for (_dir, _steps, color) in input {
        let (dir, steps) = parse_color(color);
        current = current.step(dir, steps);
        points.push(current);
        trench_len += steps;
    }

    shoelace_formula(&points) + (trench_len / 2 + 1)
}

fn shoelace_formula(points: &[Point]) -> i64 {
    let area: i64 = points
        .windows(2)
        .map(|win| (win[0].0 * win[1].1) - (win[0].1 * win[1].0))
        .sum();

    area.abs() / 2
}

fn parse_color(color: &str) -> (Dir, i64) {
    let mut chars = color.chars();

    let steps_str = chars.clone().take(5).collect::<String>();
    let steps = i64::from_str_radix(&steps_str, 16).unwrap();

    let dir = match chars.nth(5).unwrap() {
        '0' => Dir::R,
        '1' => Dir::D,
        '2' => Dir::L,
        '3' => Dir::U,
        _ => unreachable!(),
    };
    (dir, steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run18() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
