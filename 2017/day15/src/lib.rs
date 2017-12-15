struct Generator {
    multiplier: u64,
    last_value: u64,
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = (self.last_value * self.multiplier) % 2_147_483_647;

        self.last_value = next_value;

        Some(next_value)
    }
}


pub fn process_input(gen_a_start: u64, gen_b_start: u64) -> usize {
    let gen_a = Generator {
        last_value: gen_a_start,
        multiplier: 16_807,
    };
    let gen_b = Generator {
        last_value: gen_b_start,
        multiplier: 48_271,
    };

    let iterator = gen_a.into_iter().zip(gen_b.into_iter());

    iterator
        .take(40_000_000)
        .filter_map(|(a, b)| {
            if (a as u16) == (b as u16) {
                Some(())
            } else {
                None
            }
        })
        .count()
}
