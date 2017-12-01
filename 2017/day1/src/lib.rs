use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;

pub struct Input {
    data: String,
}

impl Input {
    pub fn from_str(input: &str) -> Input {
        Input { data: input.to_string() }
    }

    pub fn from_file<T: AsRef<Path>>(input: T) -> Result<Input, Box<Error>> {
        let mut file = File::open(input)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(Input { data: content })
    }
}


pub fn calculate(input: &Input) -> i32 {
    let numbers: Vec<_> =
        input.data.chars().filter_map(|c| c.to_string().parse::<i32>().ok()).collect();

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
