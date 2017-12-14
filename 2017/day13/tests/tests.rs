extern crate day13;
use day13::*;

#[test]
fn test_sample_get_posiion1() {
    assert_eq!(0, get_position(0, 4));
    assert_eq!(1, get_position(1, 4));
    assert_eq!(2, get_position(2, 4));
    assert_eq!(3, get_position(3, 4));
    assert_eq!(2, get_position(4, 4));
    assert_eq!(1, get_position(5, 4));
    assert_eq!(0, get_position(6, 4));
    assert_eq!(1, get_position(7, 4));
    assert_eq!(2, get_position(8, 4));
    assert_eq!(3, get_position(9, 4));
    assert_eq!(2, get_position(10, 4));
    assert_eq!(1, get_position(11, 4));
}