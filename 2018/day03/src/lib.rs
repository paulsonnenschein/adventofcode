extern crate regex;

use regex::Regex;

const SIZE: usize = 1000;

#[derive(Debug)]
struct Rect {
    id: i32,
    left: usize,
    top: usize,
    width: usize,
    heigth: usize,
}

pub fn process_input(input: &str) -> (usize, i32) {
    let mut area = [[0; SIZE]; SIZE];

    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    let rects = input.lines().map(|line| {
        let matches = re.captures(line).unwrap();

        Rect {
            id: matches[1].parse().unwrap(),
            left: matches[2].parse().unwrap(),
            top: matches[3].parse().unwrap(),
            width: matches[4].parse().unwrap(),
            heigth: matches[5].parse().unwrap(),
        }
    });

    for rect in rects.clone() {
        for x in rect.left..(rect.left + rect.width) {
            for y in rect.top..(rect.top + rect.heigth) {
                area[y][x] += 1;
            }
        }
    }

    let result1 = area
        .iter()
        .map(|row| row.iter().filter(|&&v| v > 1).count())
        .sum();

    let mut result2 = 0;

    for rect in rects {
        let mut overlap_found = false;
        for x in rect.left..(rect.left + rect.width) {
            for y in rect.top..(rect.top + rect.heigth) {
                if area[y][x] > 1 {
                    overlap_found = true;
                }
            }
        }
        if !overlap_found {
            result2 = rect.id;
            break;
        }
    }

    (result1, result2)
}
