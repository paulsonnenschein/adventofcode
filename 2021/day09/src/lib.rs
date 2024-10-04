use pathfinding::prelude::{bfs_reach, Matrix};

pub fn parse(input: &str) -> Matrix<u32> {
    Matrix::from_iter(
        input
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap())),
    )
}

pub fn find_low_points(input: &Matrix<u32>) -> Vec<(usize, usize)> {
    input
        .keys()
        .filter(|idx| {
            let here = input[*idx];
            input
                .neighbours(*idx, false)
                .all(|neighbor| input[neighbor] > here)
        })
        .collect()
}

pub fn part1(input: &Matrix<u32>, low_points: &[(usize, usize)]) -> u32 {
    low_points.iter().map(|idx| input[*idx] + 1).sum()
}

pub fn part2(input: &Matrix<u32>, low_points: &[(usize, usize)]) -> u32 {
    let mut basin_sizes: Vec<u32> = low_points
        .iter()
        .map(|idx| {
            bfs_reach(*idx, |i| {
                input
                    .neighbours(*i, false)
                    .filter(|neighbor| input[*neighbor] < 9)
            })
            .count() as u32
        })
        .collect();

    basin_sizes.sort_by(|a, b| a.cmp(b).reverse());

    basin_sizes.iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run09() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        let low_points = find_low_points(&parsed);
        println!("{:?}", part1(&parsed, &low_points));
        println!("{:?}", part2(&parsed, &low_points));
    }
}
