pub fn hash(input: &str, size: usize) -> i32 {
    let lengths: Vec<usize> = input
        .trim()
        .split_terminator(',')
        .filter_map(|num| num.parse::<usize>().ok())
        .collect();

    let mut list: Vec<i32> = (0..(size as i32)).collect();
    let mut position: usize = 0;
    let mut skip_size = 0;

    for length in lengths {
        let tmp_vec = list.clone()
            .into_iter()
            .cycle()
            .skip(position)
            .take(length)
            .collect::<Vec<_>>();
        let slice = tmp_vec.iter().rev();

        for (i, value) in slice.enumerate() {
            let index = (position + i) % size;
            list[index] = *value;
        }
        position = (position + length + skip_size) % size;
        skip_size += 1;
    }

    list[0] * list[1]
}

pub fn hash_part2(input: &str) -> String {
    let size: usize = 256;
    let mut lengths: Vec<u8> = input.trim().chars().map(|c| c as u8).collect();

    lengths.extend(vec![17, 31, 73, 47, 23]);

    let mut list: Vec<u8> = (0..size).map(|e| e as u8).collect();
    let mut position: usize = 0;
    let mut skip_size = 0;


    for _ in 0..64 {
        for &length in &lengths {
            let slice = list.clone()
                .into_iter()
                .cycle()
                .skip(position)
                .take(length as usize)
                .collect::<Vec<_>>()
                .into_iter()
                .rev();

            for (i, value) in slice.enumerate() {
                let index = (position + i) % size;
                list[index] = value;
            }
            position = (position + length as usize + skip_size) % size;
            skip_size += 1;
        }
    }

    list.chunks(16)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &x| acc ^ x))
        .map(|num| format!("{:02x}", num))
        .collect()
}
