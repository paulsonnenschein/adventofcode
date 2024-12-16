type Item = (i32, i32, i32, i32);
pub fn parse(input: &str) -> Vec<Item> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (left, right) = line[2..].split_once(" v=").unwrap();

            let (p_col, p_row) = left.split_once(",").unwrap();
            let (v_col, v_row) = right.split_once(",").unwrap();

            (
                p_col.parse().unwrap(),
                p_row.parse().unwrap(),
                v_col.parse().unwrap(),
                v_row.parse().unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &[Item], size: (i32, i32)) -> i32 {
    calc_safety(input, size, 100)
}

fn calc_pos(item: &Item, size: (i32, i32), seconds: i32) -> (i32, i32) {
    let dst_col = (item.0 + (item.2 * seconds)).rem_euclid(size.0);
    let dst_row = (item.1 + (item.3 * seconds)).rem_euclid(size.1);
    (dst_col, dst_row)
}

fn calc_safety(input: &[Item], size: (i32, i32), seconds: i32) -> i32 {
    let middle = (size.0 / 2, size.1 / 2);
    let acc = input.iter().map(|item| calc_pos(item, size, seconds)).fold(
        [0, 0, 0, 0],
        |mut acc, pos| {
            if pos.0 == middle.0 || pos.1 == middle.1 {
                acc
            } else {
                if pos.0 < middle.0 {
                    if pos.1 < middle.1 {
                        acc[0] += 1;
                    } else {
                        acc[1] += 1;
                    }
                } else if pos.1 < middle.1 {
                    acc[2] += 1;
                } else {
                    acc[3] += 1;
                }
                acc
            }
        },
    );

    acc.iter().product()
}

pub fn part2(input: &[Item], size: (i32, i32)) -> i32 {
    (1..)
        .take((size.0 * size.1) as usize)
        .min_by_key(|iters| calc_safety(input, size, *iters))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn run14() {
        let input = include_str!("./input.txt");
        let size = (101, 103);
        let s = Instant::now();
        let parsed = parse(input);
        let p1 = part1(&parsed, size);
        let p2 = part2(&parsed, size);
        let duration = s.elapsed();
        println!("part1: {:?}", p1);
        println!("part2: {:?}", p2);
        println!("duration: {:?}", duration);
    }
}
