#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Entry {
    pub name: String,
    pub weight: i32,
    pub children: Vec<String>,
}

pub fn str_to_entry(input: &str) -> Option<Entry> {
    lazy_static! {
        static ref RE: Regex = {
            let str = r"^(?P<name>\w+) \((?P<weight>\d+)\)(?: -> (?P<children>\w+(?:, \w+)+))?$";
            Regex::new(str).unwrap()
        };
    }

    let matches = RE.captures(input)?;

    let name = matches["name"].to_string();
    let weight = matches["weight"].to_string().parse::<i32>().ok()?;
    let children: Vec<String> = if let Some(children) = matches.name("children") {
        children.as_str().split(", ").map(String::from).collect()
    } else {
        vec![]
    };

    Some(Entry {
        name,
        weight,
        children,
    })
}

pub fn input_to_vec(input: &str) -> Vec<Entry> {
    input.lines().filter_map(str_to_entry).collect()
}

pub fn find_base_name(input: &[Entry]) -> String {
    let names: HashSet<_> = input.iter().map(|e| &e.name).collect();
    let children_names: HashSet<_> = input.iter().flat_map(|e| &e.children).collect();
    let mut differences = names.difference(&children_names);

    let o = differences.next();
    o.unwrap().to_string()
}


// this is a pretty terrible solution
// should actually construct a proper tree with references to parents / children
pub fn calculate_part2(input: &[Entry], base_name: &str) -> i32 {
    let input = Vec::from(input);
    let mut map: HashMap<String, &Entry> = HashMap::new();
    for entry in &input {
        map.insert(entry.name.clone() /* .as_ref() */, entry);
    }

    fn dos(base_name: &str, map: &HashMap<String, &Entry>) {
        let bottom = map[&base_name.to_string()];
        let mut weights: Vec<(i32, &Entry)> = Vec::new();

        for child in &bottom.children {
            let child_entry = map[&child.to_string()];
            let child_weigth = child_entry.weight;
            weights.push((
                child_weigth + calculate_children_weight(child, map),
                child_entry,
            ));
        }

        for child in &bottom.children {
            dos(child, map);
        }

        if weights.iter().all(|e| e.0 == weights.first().unwrap().0) {
            // alright they are all good
        } else {
            let mut count_map = HashMap::new();
            for weight in &weights {
                count_map.entry(weight.0).or_insert(0);
                let x = count_map[&weight.0] + 1;
                count_map.insert(weight.0, x);
            }
            let mut iter = count_map.iter();
            let first = iter.next().unwrap();
            let second = iter.next().unwrap();
            if *first.1 == 1 {
                let diff = second.0 - first.0;
                let el = weights.iter().find(|e| e.0 == *first.0).unwrap();
                println!("Solution: {}", el.1.weight + diff);
            } else {
                let diff = first.0 - second.0;
                let el = weights.iter().find(|e| e.0 == *second.0).unwrap();
                println!("Solution: {}", el.1.weight + diff);
            }
            //println!("{:?}",  count_map);
            //println!("{:?}",  weights);
        }
    }

    dos(base_name, &map);

    4
}

fn calculate_children_weight(base_name: &str, map: &HashMap<String, &Entry>) -> i32 {
    let mut weight = 0;

    for child in &(map[&base_name.to_string()].children) {
        let child: &Entry = map[&child.to_string()];
        weight += child.weight;
        weight += calculate_children_weight(&child.name, map);
    }

    weight
}
