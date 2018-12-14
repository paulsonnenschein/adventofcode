pub fn process_input(input: &str) -> (i32, i32) {
    let mut input = input
        .trim()
        .split_whitespace()
        .map(|split| split.parse::<i32>().unwrap());

    (count_meta(&mut input.clone()), node_value(&mut input))
}

fn count_meta(mut stream: &mut Iterator<Item = i32>) -> i32 {
    let child_nodes = stream.next().unwrap();
    let meta_entries = stream.next().unwrap();
    // todo streamify this
    let mut meta_total = 0;
    for _ in 0..child_nodes {
        meta_total += count_meta(&mut stream);
    }
    for _ in 0..meta_entries {
        meta_total += stream.next().unwrap();
    }

    meta_total
}

fn node_value(mut stream: &mut Iterator<Item = i32>) -> i32 {
    let child_nodes = stream.next().unwrap();
    let meta_entries = stream.next().unwrap();
    // todo streamify this
    let mut total = 0;

    let mut children = vec![];
    for _ in 0..child_nodes {
        children.push(node_value(&mut stream));
    }

    if children.is_empty() {
        for _ in 0..meta_entries {
            total += stream.next().unwrap();
        }
    } else {
        for _ in 0..meta_entries {
            let index = stream.next().unwrap() as usize - 1;

            total += children.get(index).unwrap_or(&0);
        }
    }

    total
}
