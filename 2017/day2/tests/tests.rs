extern crate day2;

use day2::*;
use std::fs::File;
use std::io::Read;

fn get_input() -> String {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input);
    input
}

#[test]
fn test_sample_row1() {
    let input = vec![5, 1, 9, 5];
    let expected = 8;
    assert_eq!(calculate_row_minmax(&input), expected);
}

#[test]
fn test_sample_row2() {
    let input = vec![7, 5, 3];
    let expected = 4;
    assert_eq!(calculate_row_minmax(&input), expected);
}

#[test]
fn test_sample_row3() {
    let input = vec![2, 4, 6, 8];
    let expected = 6;
    assert_eq!(calculate_row_minmax(&input), expected);
}

#[test]
fn test_sample_input1() {
    let input = "5 1 9 5\n7 5 3\n2 4 6 8";
    let expected = 18;
    assert_eq!(calculate_part1(input), expected);
}

#[test]
fn test_part1() {
    let input = get_input();
    let expected = 41_887;
    assert_eq!(calculate_part1(&input), expected);
}

#[test]
fn test_sample_row4() {
    let input = vec![5, 9, 2, 8];
    let expected = 4;
    assert_eq!(calculate_row_divisible(&input), expected);
}

#[test]
fn test_sample_row5() {
    let input = vec![9, 4, 7, 3];
    let expected = 3;
    assert_eq!(calculate_row_divisible(&input), expected);
}

#[test]
fn test_sample_row6() {
    let input = vec![3, 8, 6, 5];
    let expected = 2;
    assert_eq!(calculate_row_divisible(&input), expected);
}

#[test]
fn test_sample_input2() {
    let input = "5 9 2 8\n9 4 7 3\n3 8 6 5";
    let expected = 9;
    assert_eq!(calculate_part2(input), expected);
}

#[test]
fn test_part2() {
    let input = get_input();
    let expected = 226;
    assert_eq!(calculate_part2(&input), expected);
}
