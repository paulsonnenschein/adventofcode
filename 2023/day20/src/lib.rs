use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, newline},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum WiringType {
    Broadcaster,
    FlipFlop,
    Conj,
}

#[derive(Debug)]
pub struct WiringDesc {
    t: WiringType,
    name: &'static str,
    targets: Vec<&'static str>,
}

pub enum Wiring {
    Broadcaster {
        name: &'static str,
        targets: Vec<&'static str>,
    },
    FlipFlop {
        name: &'static str,
        targets: Vec<&'static str>,
        is_on: bool,
    },
    Conj {
        name: &'static str,
        targets: Vec<&'static str>,
        source_last_sent_high: HashMap<&'static str, bool>,
    },
}

impl Wiring {
    fn receive(&mut self, t: HiLo, source: &str, emit_queue: &mut VecDeque<(&str, HiLo, &str)>) {
        match self {
            Wiring::Broadcaster { name, targets } => {
                for target in targets {
                    emit_queue.push_back((name, t, target));
                }
            }
            Wiring::FlipFlop {
                name,
                targets,
                ref mut is_on,
            } => {
                if t == HiLo::Lo {
                    *is_on = !*is_on;
                    let send_t = if *is_on { HiLo::Hi } else { HiLo::Lo };
                    for target in targets {
                        emit_queue.push_back((name, send_t, target));
                    }
                }
            }
            Wiring::Conj {
                name,
                targets,
                ref mut source_last_sent_high,
            } => {
                *source_last_sent_high.get_mut(source).unwrap() = t == HiLo::Hi;

                let send_t = if source_last_sent_high.values().all(|v| *v) {
                    HiLo::Lo
                } else {
                    HiLo::Hi
                };
                for target in targets {
                    emit_queue.push_back((name, send_t, target));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum HiLo {
    Hi,
    Lo,
}

fn parse_wiring(i: &'static str) -> IResult<&str, WiringDesc> {
    let (i, (t, name)) = alt((
        map(tag("broadcaster"), |b| (WiringType::Broadcaster, b)),
        map(preceded(tag("%"), alpha0), |name| {
            (WiringType::FlipFlop, name)
        }),
        map(preceded(tag("&"), alpha0), |name| (WiringType::Conj, name)),
    ))(i)?;
    let (i, targets) = preceded(tag(" -> "), separated_list0(tag(", "), alpha0))(i)?;

    Ok((i, WiringDesc { t, name, targets }))
}

pub fn parse(input: &'static str) -> Vec<WiringDesc> {
    all_consuming(separated_list0(newline, parse_wiring))(input.trim())
        .unwrap()
        .1
}

pub fn part1(input: &[WiringDesc]) -> u32 {
    // build map
    let mut map = build_map(input);

    // send pulses
    let mut his = 0;
    let mut los = 0;
    for _ in 0..1000 {
        let mut pulse_queue = VecDeque::new();
        pulse_queue.push_front(("button", HiLo::Lo, "broadcaster"));

        while let Some((source, hilo, target)) = pulse_queue.pop_front() {
            match hilo {
                HiLo::Hi => his += 1,
                HiLo::Lo => los += 1,
            };

            if let Some(w) = map.get_mut(target) {
                w.receive(hilo, source, &mut pulse_queue);
            }
        }
    }

    his * los
}

fn build_map(input: &[WiringDesc]) -> HashMap<&str, Wiring> {
    let mut map = HashMap::new();
    for w in input {
        match &w.t {
            WiringType::Broadcaster => {
                map.insert(
                    w.name,
                    Wiring::Broadcaster {
                        name: w.name,
                        targets: w.targets.clone(),
                    },
                );
            }
            WiringType::FlipFlop => {
                map.insert(
                    w.name,
                    Wiring::FlipFlop {
                        name: w.name,
                        targets: w.targets.clone(),
                        is_on: false,
                    },
                );
            }
            WiringType::Conj => {
                let source_last_sent_high = input
                    .iter()
                    .filter(|wd| wd.targets.iter().any(|target| target == &w.name))
                    .map(|wd| (wd.name, false))
                    .collect();
                map.insert(
                    w.name,
                    Wiring::Conj {
                        name: w.name,
                        targets: w.targets.clone(),
                        source_last_sent_high,
                    },
                );
            }
        }
    }
    map
}

pub fn part2(input: &[WiringDesc]) -> u64 {
    // build map
    let mut map = build_map(input);

    // node that leads to rx
    let target_name = "jm";

    // find which nodes lead to target
    let mut target_hi_interval = if let Wiring::Conj {
        source_last_sent_high,
        ..
    } = &map[target_name]
    {
        source_last_sent_high
            .keys()
            .map(|k| (*k, (0u64, 0u64)))
            .collect::<HashMap<_, _>>()
    } else {
        unreachable!()
    };

    // send pulses
    'outer: for i in 0.. {
        let mut pulse_queue = VecDeque::new();
        pulse_queue.push_front(("button", HiLo::Lo, "broadcaster"));

        while let Some((source, hilo, target)) = pulse_queue.pop_front() {
            if let Some(w) = map.get_mut(target) {
                w.receive(hilo, source, &mut pulse_queue);
            }

            if target == target_name && hilo == HiLo::Hi {
                if target_hi_interval[source].0 == 0 {
                    target_hi_interval.get_mut(source).unwrap().0 = i;
                } else if target_hi_interval[source].1 == 0 {
                    target_hi_interval.get_mut(source).unwrap().1 =
                        i - target_hi_interval[source].0;
                }

                if target_hi_interval.values().all(|v| v.1 > 0) {
                    break 'outer;
                }
            }
        }
    }

    target_hi_interval
        .values()
        .map(|(_, v)| *v)
        .fold(1u64, num::integer::lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run20() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
