pub fn process_input(input: usize) -> (i32, i32) {
    let mut position: usize = 0;
    let mut buffer = vec![0];

    for i in 1..2018 {
        let next_position = (position + input) % buffer.len();

        buffer.insert(next_position + 1, i);

        position = next_position + 1;
    }

    let part1 = buffer[position + 1];

    /*
        For part 2 you only need to keep track of the number after zero.
        So i keep numbers whoose insert position is 0.
    */

    let mut number_after_zero = 0;
    let mut buffer_len = buffer.len();

    for i in 2018..50_000_001 {
        let next_position = (position + input) % buffer_len;
        buffer_len += 1;

        if next_position == 0 {
            number_after_zero = i;
        }

        position = next_position + 1;
    }

    let part2 = number_after_zero;

    (part1, part2)
}
