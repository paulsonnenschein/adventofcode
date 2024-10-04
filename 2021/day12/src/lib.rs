use std::collections::HashMap;

pub fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();
    for (from, to) in input
        .trim()
        .lines()
        .map(|line| line.split_once('-').unwrap())
    {
        map.entry(from).or_insert_with(Vec::new).push(to);
        map.entry(to).or_insert_with(Vec::new).push(from);
    }

    map
}

pub fn part1(input: &HashMap<&str, Vec<&str>>) -> usize {
    let mut current_path = vec!["start"];
    let mut all_paths = Vec::new();

    fn visit<'a>(
        field: &'a str,
        input: &HashMap<&str, Vec<&'a str>>,
        current_path: &mut Vec<&'a str>,
        all_paths: &mut Vec<Vec<&'a str>>,
    ) {
        if field == "end" {
            all_paths.push(current_path.clone());
        } else {
            current_path.push(field);

            for neighbor in input.get(field).unwrap() {
                if neighbor.chars().next().unwrap().is_uppercase()
                    || !current_path.contains(neighbor)
                {
                    visit(neighbor, input, current_path, all_paths);
                }
            }

            current_path.pop();
        }
    }

    for neighbor in input.get("start").unwrap() {
        visit(neighbor, input, &mut current_path, &mut all_paths);
    }

    all_paths.len()
}

pub fn part2(input: &HashMap<&str, Vec<&str>>) -> usize {
    let mut current_path = vec!["start"];
    let mut all_paths = Vec::new();

    fn visit<'a>(
        field: &'a str,
        input: &HashMap<&str, Vec<&'a str>>,
        current_path: &mut Vec<&'a str>,
        all_paths: &mut Vec<Vec<&'a str>>,
        used_double_cave: bool,
    ) {
        if field == "end" {
            all_paths.push(current_path.clone());
        } else {
            current_path.push(field);

            for neighbor in input.get(field).unwrap() {
                if neighbor.chars().next().unwrap().is_uppercase() {
                    visit(neighbor, input, current_path, all_paths, used_double_cave);
                } else if current_path.contains(neighbor) {
                    if neighbor != &"start" && neighbor != &"end" && !used_double_cave {
                        visit(neighbor, input, current_path, all_paths, true);
                    }
                } else {
                    visit(neighbor, input, current_path, all_paths, used_double_cave);
                }
            }

            current_path.pop();
        }
    }

    for neighbor in input.get("start").unwrap() {
        visit(neighbor, input, &mut current_path, &mut all_paths, false);
    }

    all_paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run12() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
