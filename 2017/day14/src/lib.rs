#![feature(i128_type)]
extern crate day10;

pub fn count(input: &str) -> (i32, i32) {
    let mut grid = create_grid_from_input(input);
    let sum = grid.iter()
        .map(|row| row.iter().filter(|e| **e).count() as i32)
        .sum();

    let mut groups = 0;
    for y in 0..128 {
        for x in 0..128 {
            if grid[y][x] {
                groups += 1;
                remove_group(x, y, &mut grid);
            }
        }
    }

    (sum, groups)
}

fn create_grid_from_input(input: &str) -> Vec<Vec<bool>> {
    (0..128)
        .map(|i| {
            let string = format!("{}-{}", input, i);
            let hash = day10::hash_part2(&string);

            let hash_number = u128::from_str_radix(&hash, 16).unwrap();
            format!("{:0128b}", hash_number)
                .chars()
                .map(|c| c == '1')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn remove_group(x: usize, y: usize, grid: &mut Vec<Vec<bool>>) {
    let mut to_remove = vec![(x, y)];

    while let Some((x_, y_)) = to_remove.pop() {
        if x_ < 127 && grid[y_][x_ + 1] {
            to_remove.push((x_ + 1, y_));
        }
        if y_ < 127 && grid[y_ + 1][x_] {
            to_remove.push((x_, y_ + 1));
        }
        if x_ > 0 && grid[y_][x_ - 1] {
            to_remove.push((x_ - 1, y_));
        }
        if y_ > 0 && grid[y_ - 1][x_] {
            to_remove.push((x_, y_ - 1));
        }
        grid[y_][x_] = false;
    }
}
