use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;

pub struct Input {
    data: String,
}

impl Input {
    pub fn from_string(input: &str) -> Input {
        Input { data: input.to_string() }
    }

    pub fn from_file<T: AsRef<Path>>(input: T) -> Result<Input, Box<Error>> {
        let mut file = File::open(input)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(Input { data: content })
    }

    fn to_vec(&self) -> Vec<i32> {
        self.data.chars().filter_map(|c| c.to_string().parse::<i32>().ok()).collect()
    }
}


pub fn calculate_part1(input: &Input) -> i32 {
    let numbers = input.to_vec();

    let mut total = 0;
    let mut last = *numbers.last().unwrap();

    for number in numbers {
        if number == last {
            total += number;
        }
        last = number;
    }

    total
}


pub fn calculate_part2(input: &Input) -> i32 {
    let numbers = input.to_vec();
    let length = numbers.len();
    let position = length / 2;

    let mut total = 0;

    for (i, &number) in numbers.iter().enumerate() {
        let other = if i + position >= length {
            numbers[i - position]
        } else {
            numbers[i + position]
        };
        if number == other {
            total += number;
        }
    }

    total
}
