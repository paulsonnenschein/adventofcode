use std::collections::HashMap;
use std::ops::RangeInclusive;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, line_ending, u64 as parse_u64},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
pub enum StepTarget {
    Label(&'static str),
    A,
    R,
}

#[derive(Debug)]
pub enum WorkflowStep {
    Gt(&'static str, u64, StepTarget),
    Lt(&'static str, u64, StepTarget),
    J(StepTarget),
}

#[derive(Debug)]
pub struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug)]
pub struct Input {
    workflows: HashMap<&'static str, Vec<WorkflowStep>>,
    parts: Vec<Part>,
}

fn parse_target(i: &'static str) -> IResult<&str, StepTarget> {
    alt((
        map(tag("A"), |_| StepTarget::A),
        map(tag("R"), |_| StepTarget::R),
        map(alpha0, StepTarget::Label),
    ))(i)
}

fn parse_step(i: &'static str) -> IResult<&str, WorkflowStep> {
    alt((
        map(
            tuple((alpha0, tag(">"), parse_u64, tag(":"), parse_target)),
            |(key, _, num, _, target)| WorkflowStep::Gt(key, num, target),
        ),
        map(
            tuple((alpha0, tag("<"), parse_u64, tag(":"), parse_target)),
            |(key, _, num, _, target)| WorkflowStep::Lt(key, num, target),
        ),
        map(parse_target, WorkflowStep::J),
    ))(i)
}

fn parse_workflow(i: &'static str) -> IResult<&str, (&str, Vec<WorkflowStep>)> {
    tuple((
        alpha0,
        delimited(tag("{"), separated_list0(tag(","), parse_step), tag("}")),
    ))(i)
}

fn parse_workflows(i: &'static str) -> IResult<&str, HashMap<&str, Vec<WorkflowStep>>> {
    let (i, workflows) = separated_list0(line_ending, parse_workflow)(i)?;
    Ok((i, workflows.into_iter().collect()))
}

fn parse_parts(i: &'static str) -> IResult<&str, Vec<Part>> {
    separated_list0(
        line_ending,
        map(
            tuple((
                tag("{x="),
                parse_u64,
                tag(",m="),
                parse_u64,
                tag(",a="),
                parse_u64,
                tag(",s="),
                parse_u64,
                tag("}"),
            )),
            |(_, x, _, m, _, a, _, s, _)| Part { x, m, a, s },
        ),
    )(i)
}

fn parse_input(i: &'static str) -> IResult<&str, Input> {
    let (i, (workflows, parts)) = separated_pair(parse_workflows, tag("\n\n"), parse_parts)(i)?;

    Ok((i, Input { workflows, parts }))
}

pub fn parse(input: &'static str) -> Input {
    all_consuming(parse_input)(input.trim()).unwrap().1
}

pub fn part1(input: &Input) -> u64 {
    input
        .parts
        .iter()
        .filter(|p| is_accepted(p, &input.workflows, "in"))
        .map(|p| p.x + p.m + p.a + p.s)
        .sum()
}

fn is_accepted(
    part: &Part,
    workflows: &HashMap<&'static str, Vec<WorkflowStep>>,
    workflow: &'static str,
) -> bool {
    let apply_target = |target: &StepTarget| -> bool {
        match target {
            StepTarget::A => true,
            StepTarget::R => false,
            StepTarget::Label(label) => is_accepted(part, workflows, label),
        }
    };
    for step in &workflows[workflow] {
        match step {
            WorkflowStep::Gt("x", val, label) if part.x > *val => return apply_target(label),
            WorkflowStep::Gt("m", val, label) if part.m > *val => return apply_target(label),
            WorkflowStep::Gt("a", val, label) if part.a > *val => return apply_target(label),
            WorkflowStep::Gt("s", val, label) if part.s > *val => return apply_target(label),
            WorkflowStep::Lt("x", val, label) if part.x < *val => return apply_target(label),
            WorkflowStep::Lt("m", val, label) if part.m < *val => return apply_target(label),
            WorkflowStep::Lt("a", val, label) if part.a < *val => return apply_target(label),
            WorkflowStep::Lt("s", val, label) if part.s < *val => return apply_target(label),
            WorkflowStep::J(target) => return apply_target(target),
            _ => {}
        }
    }
    unreachable!()
}

#[derive(Clone, Debug)]
struct PartRange {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}

impl PartRange {
    fn split(&self, field: &str, gt: bool, val: u64) -> (PartRange, PartRange) {
        let mut satisfy = self.clone();
        let mut doesnt = self.clone();
        match field {
            "x" => {
                if gt {
                    satisfy.x = (val + 1)..=*self.x.end();
                    doesnt.x = *self.x.start()..=val;
                } else {
                    satisfy.x = *self.x.start()..=(val - 1);
                    doesnt.x = val..=*self.x.end();
                }
            }
            "m" => {
                if gt {
                    satisfy.m = (val + 1)..=*self.m.end();
                    doesnt.m = *self.m.start()..=val;
                } else {
                    satisfy.m = *self.m.start()..=(val - 1);
                    doesnt.m = val..=*self.m.end();
                }
            }
            "a" => {
                if gt {
                    satisfy.a = (val + 1)..=*self.a.end();
                    doesnt.a = *self.a.start()..=val;
                } else {
                    satisfy.a = *self.a.start()..=(val - 1);
                    doesnt.a = val..=*self.a.end();
                }
            }
            "s" => {
                if gt {
                    satisfy.s = (val + 1)..=*self.s.end();
                    doesnt.s = *self.s.start()..=val;
                } else {
                    satisfy.s = *self.s.start()..=(val - 1);
                    doesnt.s = val..=*self.s.end();
                }
            }
            _ => unreachable!(),
        };

        (satisfy, doesnt)
    }
}

pub fn part2(input: &Input) -> u64 {
    let init = PartRange {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };
    num_acceptable(init, &input.workflows, "in")
}

fn num_acceptable(
    mut current_range: PartRange,
    workflows: &HashMap<&str, Vec<WorkflowStep>>,
    workflow: &str,
) -> u64 {
    let mut combinations = 0;

    let apply_target = |target: &StepTarget, range: PartRange| -> u64 {
        match target {
            StepTarget::A => {
                (range.x.count() * range.m.count() * range.a.count() * range.s.count()) as u64
            }
            StepTarget::R => 0,
            StepTarget::Label(label) => num_acceptable(range, workflows, label),
        }
    };
    for step in &workflows[workflow] {
        match step {
            WorkflowStep::Gt(field, val, target) => {
                let (satisfy, doesnt) = current_range.split(field, true, *val);
                combinations += apply_target(target, satisfy);
                current_range = doesnt;
            }
            WorkflowStep::Lt(field, val, target) => {
                let (satisfy, doesnt) = current_range.split(field, false, *val);
                combinations += apply_target(target, satisfy);
                current_range = doesnt;
            }
            WorkflowStep::J(target) => combinations += apply_target(target, current_range.clone()),
        }
    }

    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run19() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
