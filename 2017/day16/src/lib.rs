pub fn process_input(input: &str) -> (String, String) {
    let mut programs = vec![
        'a',
        'b',
        'c',
        'd',
        'e',
        'f',
        'g',
        'h',
        'i',
        'j',
        'k',
        'l',
        'm',
        'n',
        'o',
        'p',
    ];

    for mut line in input.trim().split_terminator(',').map(|line| line.chars()) {
        match line.next().unwrap() {
            's' => spin(&mut programs, &line.collect::<String>()),
            'x' => exchange(&mut programs, &line.collect::<String>()),
            'p' => partner(&mut programs, &line.collect::<String>()),
            _ => unreachable!(),
        }
    }


    let part1 = programs.iter().collect();


    for _i in 1..(1_000_000_000 % 30 /* cycle length */) {
        for mut line in input.trim().split_terminator(',').map(|line| line.chars()) {
            match line.next().unwrap() {
                's' => spin(&mut programs, &line.collect::<String>()),
                'x' => exchange(&mut programs, &line.collect::<String>()),
                'p' => partner(&mut programs, &line.collect::<String>()),
                _ => unreachable!(),
            }
        }
        /*
        Find cycle length (https://www.reddit.com/r/adventofcode/comments/7k68bb/respect_to_all_the_math_geniuses_on_here/drby1lr/)
        if programs == vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'] {
            println!("yo: {}", _i);
        }
        */
    }

    let part2 = programs.iter().collect();

    (part1, part2)
}

fn spin(programs: &mut Vec<char>, command: &str) {
    let count: usize = command.parse().unwrap();
    let len = programs.len();


    let copy = programs
        .clone()
        .into_iter()
        .cycle()
        .skip(len - count)
        .take(len);

    for (i, value) in copy.enumerate() {
        programs[i] = value;
    }
}

fn exchange(mut programs: &mut Vec<char>, command: &str) {
    let mut command_parts = command.split_terminator('/');
    let first: usize = command_parts.next().unwrap().parse().unwrap();
    let second: usize = command_parts.next().unwrap().parse().unwrap();

    swap_at_positions(&mut programs, first, second);
}

fn partner(mut programs: &mut Vec<char>, command: &str) {
    let mut command_parts = command.chars();
    let first_name = command_parts.next().unwrap();
    let _ = command_parts.next();
    let second_name = command_parts.next().unwrap();

    let first = programs.iter().position(|&e| e == first_name).unwrap();
    let second = programs.iter().position(|&e| e == second_name).unwrap();

    swap_at_positions(&mut programs, first, second);
}

fn swap_at_positions(programs: &mut Vec<char>, first: usize, second: usize) {
    programs.swap(first, second);
}
