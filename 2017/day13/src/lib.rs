use std::collections::HashMap;

pub fn process_input(input: &str) -> (i32, i32) {
    let layers: HashMap<i32, i32> = input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_terminator(": ");
            let depth = parts.next().unwrap().parse::<i32>().unwrap();
            let range = parts.next().unwrap().parse::<i32>().unwrap();
            (depth, range)
        })
        .collect();


    let layer_count = *layers.keys().max().unwrap() + 1; // add one for layer 0

    let counter = {
        let mut firewall: Vec<i32> = vec![0; layer_count as usize];

        let mut counter = 0;
        for i in 0..layer_count {
            apply_tick(i, &mut firewall, &layers);
            // println!("d{}: {:?}", i, firewall);
            if firewall[i as usize] == 0 && layers.contains_key(&i) {
                counter += i * layers[&i];
            }
        }

        counter
    };

    let mut delay = 0;
    // start at 0, but check the output for "checkpoints"
    for i in 3_873_000.. {
        if count_for_starting_iterations(i, layer_count, &layers) {
            delay = i;
            break;
        }

        if i % 1000 == 0 {
            println!("i: {}", i);
        }
    }

    (counter, delay)
}

fn count_for_starting_iterations(
    start_iteration: i32,
    layer_count: i32,
    layers: &HashMap<i32, i32>,
) -> bool {
    let mut firewall: Vec<i32> = vec![0; layer_count as usize];

    let mut iteration = start_iteration;
    for i in 0..layer_count {
        apply_tick(iteration, &mut firewall, layers);
        // println!("d{}: {:?}", i, firewall);
        if firewall[i as usize] == 0 && layers.contains_key(&i) {
            return false;
        }
        iteration += 1;
    }

    true
}

fn apply_tick(iteration: i32, firewall: &mut Vec<i32>, layers: &HashMap<i32, i32>) {
    for (i, layer) in firewall.iter_mut().enumerate() {
        if layers.contains_key(&(i as i32)) {
            *layer = get_position(iteration, layers[&(i as i32)]);
        }
    }
}

pub fn get_position(iteration: i32, size: i32) -> i32 {
    let v = iteration % (size * 2 - 2);

    if v >= size {
        (size * 2) - (v + 2)
    } else {
        v
    }
}
