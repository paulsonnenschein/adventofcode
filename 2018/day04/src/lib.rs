#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

type GuardData = HashMap<i32, i32>;

pub fn process_input(input: &str) -> (i32, i32) {
    let mut entries: Vec<Entry> = input.lines().map(|line| line.parse().unwrap()).collect();

    entries.sort_by(|e1, e2| e1.date.cmp(&e2.date));

    let mut current_id = 0;
    let mut start_sleep = Date::empty();
    let mut data: HashMap<i32, GuardData> = HashMap::new();

    for Entry { date, action } in &entries {
        match action {
            Action::Start(id) => current_id = *id,
            Action::FallAsleep => start_sleep = *date,
            Action::WakeUp => calculate_sleep(
                &start_sleep,
                &date,
                &mut data.entry(current_id).or_insert_with(HashMap::new),
            ),
        }
    }

    let (guard_id, _) = data
        .iter()
        .fold((0, 0), |(best_guard, best), (&guard_id, guard_data)| {
            let sum = guard_data.values().sum::<i32>();
            if sum > best {
                (guard_id, sum)
            } else {
                (best_guard, best)
            }
        });

    let (guard_minute, _max_value) = {
        let x: &GuardData = &data[&guard_id];
        x.iter().max_by(|(_, l), (_, r)| l.cmp(r)).unwrap()
    };

    let part1 = guard_id * guard_minute;

    let (guard_id, guard_minute, _) = data.iter().fold(
        (0, 0, 0),
        |(best_guard, best_minute, max), (&guard_id, guard_data)| {
            let (&guard_best_minute, &guard_max) =
                guard_data.iter().max_by(|(_, l), (_, r)| l.cmp(r)).unwrap();
            if guard_max > max {
                (guard_id, guard_best_minute, guard_max)
            } else {
                (best_guard, best_minute, max)
            }
        },
    );

    let part2 = guard_id * guard_minute;

    (part1, part2)
}

fn calculate_sleep(start_date: &Date, end_date: &Date, data: &mut GuardData) {
    let mut current = (start_date.hour, start_date.minute);
    let target = (end_date.hour, end_date.minute);

    loop {
        if current == target {
            break;
        }
        *data.entry(current.1).or_insert(0) += 1;

        current.1 += 1;
        if current.1 == 60 {
            current = ((current.0 + 1) % 24, 0);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    date: Date,
    action: Action,
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
struct Date {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}

impl Date {
    fn empty() -> Date {
        Date {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Start(i32),
    FallAsleep,
    WakeUp,
}

impl FromStr for Entry {
    type Err = Box<Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref LINE: Regex =
                Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)").unwrap();
        }
        lazy_static! {
            static ref START: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
        }

        let matches = LINE.captures(line).unwrap();

        let action = match &matches[6] {
            "falls asleep" => Action::FallAsleep,
            "wakes up" => Action::WakeUp,
            _ => {
                let matches = START.captures(&matches[6]).unwrap();
                Action::Start(matches[1].parse()?)
            }
        };

        Ok(Entry {
            date: Date {
                year: matches[1].parse()?,
                month: matches[2].parse()?,
                day: matches[3].parse()?,
                hour: matches[4].parse()?,
                minute: matches[5].parse()?,
            },
            action,
        })
    }
}
