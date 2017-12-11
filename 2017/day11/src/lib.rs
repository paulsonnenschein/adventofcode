use std::cmp::max;

// https://www.redblobgames.com/grids/hexagons/

pub fn process_input(input: &str) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;

    let mut max_distance = 0;

    for instruction in input.trim().split_terminator(',') {
        match instruction {
            "n" => {
                y += 1;
                z -= 1;
            }
            "ne" => {
                x += 1;
                z -= 1;
            }
            "se" => {
                x += 1;
                y -= 1;
            }
            "s" => {
                z += 1;
                y -= 1;
            }
            "sw" => {
                z += 1;
                x -= 1;
            }
            "nw" => {
                y += 1;
                x -= 1;
            }
            _ => unreachable!(),
        };
        max_distance = max(max_distance, distance(x, y, z))
    }

    (distance(x, y, z), max_distance)
}

fn distance(x: i32, y: i32, z: i32) -> i32 {
    max(x.abs(), max(y.abs(), z.abs()))
}
