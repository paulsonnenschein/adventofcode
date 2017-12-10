

pub fn hash(input: &str, size: usize) -> i32 {
    let lengths: Vec<usize> = input
        .trim()
        .split_terminator(',')
        .filter_map(|num| num.parse::<usize>().ok())
        .collect();

    let mut list: Vec<i32> = (0..(size as i32)).collect();
    let mut position:usize = 0;
    let mut skip_size = 0;

    println!("{:?}", lengths);

    for length in lengths {
        let tmp_vec = list.clone().into_iter().cycle().skip(position).take(length).collect::<Vec<_>>();
        let slice = tmp_vec.iter().rev();

        for (i, value) in slice.enumerate() {
            let index = (position + i) % size;
            list[index] = *value;
        }
        position = (position + length + skip_size) % size;
        skip_size += 1;
        println!("{:?}{:?}{:?}", position, tmp_vec, list);
    }

    list[0] * list[1]
}
