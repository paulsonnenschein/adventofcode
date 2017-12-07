extern crate day6;

use day6::*;

#[test]
fn test_sample_input1() {
    let mut input = vec![0, 2, 7, 0];
    balance_banks(&mut input);
    assert_eq!(vec![2, 4, 1, 2], input);
}
#[test]
fn test_sample_input2() {
    let mut input = vec![2, 4, 1, 2];
    balance_banks(&mut input);
    assert_eq!(vec![3, 1, 2, 3], input);
}
#[test]
fn test_sample_input3() {
    let mut input = vec![3, 1, 2, 3];
    balance_banks(&mut input);
    assert_eq!(vec![0, 2, 3, 4], input);
}
#[test]
fn test_sample_input4() {
    let mut input = vec![0, 2, 3, 4];
    balance_banks(&mut input);
    assert_eq!(vec![1, 3, 4, 1], input);
}
#[test]
fn test_sample_input5() {
    let mut input = vec![1, 3, 4, 1];
    balance_banks(&mut input);
    assert_eq!(vec![2, 4, 1, 2], input);
}
#[test]
fn test_sample_input6() {
    let input = vec![0, 2, 7, 0];

    assert_eq!(5, count_iterations(&input));
}