extern crate day10;
use day10::*;

#[test]
fn test_sample_input1() {
    assert_eq!(12, hash("3,4,1,5", 5))
}

#[test]
fn test_sample_input2() {
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d".to_string(), hash_part2("1,2,3"))
}