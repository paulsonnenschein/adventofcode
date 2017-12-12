use std::collections::HashMap;
use std::collections::HashSet;

pub fn count_programs(input: &str) -> (i32, i32) {
    let inputs = input
        .trim()
        .lines()
        .map(|line| {
            let mut line_iter = line.split_terminator(" <-> ");

            let program = line_iter.next().unwrap().parse::<i32>().unwrap();
            let connections = line_iter
                .next()
                .unwrap()
                .split_terminator(", ")
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (program, connections)
        })
        .collect::<HashMap<i32, Vec<i32>>>();

    let mut visited = HashSet::new();

    visited.insert(0);
    let count = 1 + count_connections_for_programm(0, &inputs, &mut visited);

    let mut groups = 1; // group 0 is first
    for i in 1..1999 {
        if visited.contains(&i) {
            continue;
        }
        let _ = count_connections_for_programm(i, &inputs, &mut visited);
        groups += 1;
    }

    (count, groups)
}

fn count_connections_for_programm(
    program: i32,
    map: &HashMap<i32, Vec<i32>>,
    mut visited: &mut HashSet<i32>,
) -> i32 {
    let mut counter = 0;

    for connected_program in &map[&program] {
        if visited.contains(connected_program) {
            // do nothing
        } else {
            visited.insert(*connected_program);
            counter += 1;
            counter += count_connections_for_programm(*connected_program, map, &mut visited);
        }
    }

    counter
}
