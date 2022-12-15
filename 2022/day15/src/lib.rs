use nom::{
    bytes::complete::tag,
    character::complete::{i32 as parse_i32, line_ending},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::ops::RangeInclusive;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point(i32, i32);

pub struct Sensor {
    position: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn positions_in_range_at_row(&self, row: i32) -> Option<RangeInclusive<i32>> {
        let dist_beacon = self.radius();
        let dist_vert = self.position.0.abs_diff(row);
        if dist_vert <= dist_beacon {
            let diff = (dist_beacon - dist_vert) as i32; // how many tiles gan we go left/right
            Some((self.position.1 - diff)..=(self.position.1 + diff))
        } else {
            None
        }
    }

    fn radius(&self) -> u32 {
        self.position.0.abs_diff(self.closest_beacon.0)
            + self.position.1.abs_diff(self.closest_beacon.1)
    }
}

trait RangeActions {
    fn merge_if_possible(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn is_contained_within(&self, other: &Self) -> bool;
    fn intersects(&self, other: &Self) -> bool;
    fn touches(&self, other: &Self) -> bool;
    fn my_size(&self) -> u32;
}

impl RangeActions for RangeInclusive<i32> {
    fn merge_if_possible(&self, other: &Self) -> Option<Self> {
        if self.is_contained_within(other) {
            Some(other.clone())
        } else if other.is_contained_within(self) {
            Some(self.clone())
        } else if self.intersects(other) || self.touches(other) {
            Some(((*self.start()).min(*other.start()))..=((*self.end()).max(*other.end())))
        } else {
            None
        }
    }
    fn is_contained_within(&self, other: &Self) -> bool {
        other.contains(self.start()) && other.contains(self.end())
    }

    fn intersects(&self, other: &Self) -> bool {
        other.contains(self.start()) || other.contains(self.end())
    }

    fn touches(&self, other: &Self) -> bool {
        self.end() + 1 == *other.start() || other.end() + 1 == *self.start()
    }

    fn my_size(&self) -> u32 {
        self.start().abs_diff(*self.end())
    }
}

fn parse_point(i: &str) -> IResult<&str, Point> {
    map(
        tuple((tag("x="), parse_i32, tag(", y="), parse_i32)),
        |(_, col, _, row)| Point(row, col),
    )(i)
}

fn parse_sensor(i: &str) -> IResult<&str, Sensor> {
    map(
        tuple((
            tag("Sensor at "),
            parse_point,
            tag(": closest beacon is at "),
            parse_point,
        )),
        |(_, position, _, closest_beacon)| Sensor {
            position,
            closest_beacon,
        },
    )(i)
}

pub fn parse(input: &str) -> Vec<Sensor> {
    all_consuming(separated_list1(line_ending, parse_sensor))(input.trim())
        .unwrap()
        .1
}

fn detected_ranges_in_row(input: &[Sensor], row: i32) -> Vec<RangeInclusive<i32>> {
    let matched_ranges = input
        .iter()
        .filter_map(|sensor| sensor.positions_in_range_at_row(row));

    // merge intersecting ranges
    let mut filtered_ranges = Vec::<RangeInclusive<i32>>::new();

    for range in matched_ranges {
        let mut current_range = range;
        // extend new range with intersecting / touching ranges
        let mut did_change = true;
        let mut merged_ranges = vec![false; filtered_ranges.len()];
        while did_change {
            did_change = false;
            for (i, matched_range) in filtered_ranges.iter_mut().enumerate() {
                if merged_ranges[i] {
                    // skip allready merged ranges
                } else if let Some(merged) = current_range.merge_if_possible(matched_range) {
                    current_range = merged;
                    did_change = true;
                    merged_ranges[i] = true;
                }
            }
        }

        filtered_ranges.retain(|matched_range| !matched_range.is_contained_within(&current_range));
        filtered_ranges.push(current_range);
    }

    filtered_ranges
}

pub fn part1(input: &[Sensor], target_row: i32) -> usize {
    let ranges = detected_ranges_in_row(input, target_row);

    ranges
        .into_iter()
        .map(|range| range.my_size() as usize)
        .sum()
}

pub fn part2(input: &[Sensor], target_row: i32) -> i64 {
    (0..target_row)
        .find_map(|row| {
            let mut ranges = detected_ranges_in_row(input, row);
            if ranges.len() > 1 {
                ranges.sort_by_key(|range| *range.start());
                Some(i64::from(ranges[0].end() + 1) * 4_000_000_i64 + i64::from(row))
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run15() {
        let (input, target_row) = (include_str!("./input.txt"), 2000000);
        let parsed = parse(input);
        println!("{:?}", part1(&parsed, target_row));
        println!("{:?}", part2(&parsed, target_row * 2));
    }

    #[test]
    fn test_distance_calc() {
        let sensor = Sensor {
            position: Point(7, 8),
            closest_beacon: Point(10, 2),
        };

        assert_eq!(9, sensor.radius());
        assert_eq!(Some(2..=14), sensor.positions_in_range_at_row(10));
    }
}
