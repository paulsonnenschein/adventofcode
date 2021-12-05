#[derive(Debug)]
pub struct Input {
    numbers: Vec<u32>,
    boards: Vec<Vec<Vec<u32>>>,
}

pub fn parse(input: &str) -> Input {
    let mut parts = input.split("\n\n");

    let numbers: Vec<u32> = parts
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u32>().unwrap())
        .collect();

    let boards = parts
        .map(|part| {
            part.lines()
                .map(|line| {
                    line.split_whitespace()
                        .filter(|part| !part.trim().is_empty())
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .collect();

    Input { numbers, boards }
}

pub fn solve(input: &Input) -> (u32, u32) {
    let mut boards = input.boards.clone();
    let board_count = boards.len();

    let mut solutions: Vec<u32> = vec![];
    let mut done_boards: Vec<usize> = vec![];

    'outer: for num in &input.numbers {
        for (i_board, board) in boards.iter_mut().enumerate() {
            if done_boards.contains(&i_board) {
                continue;
            }
            let mut found_one = false;
            'inner: for row in board.iter_mut() {
                for board_num in row.iter_mut() {
                    if board_num == num {
                        *board_num = u32::MAX;
                        found_one = true;
                        break 'inner;
                    }
                }
            }
            if found_one && has_bingo(board) {
                let sum: u32 = board
                    .iter()
                    .map(|row| row.iter().filter(|&&x| x != u32::MAX).sum::<u32>())
                    .sum();
                solutions.push(sum * num);
                done_boards.push(i_board);
                if done_boards.len() == board_count {
                    break 'outer;
                }
            }
        }
    }

    (*solutions.first().unwrap(), *solutions.last().unwrap())
}

fn has_bingo(board: &[Vec<u32>]) -> bool {
    if board.iter().any(|row| row.iter().all(|&x| x == u32::MAX)) {
        return true;
    }

    for col in 0..5usize {
        let mut count = 0;
        for row in board {
            if row[col] == u32::MAX {
                count += 1;
            }
        }
        if count == 5 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run04() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", solve(&parsed));
    }
}
