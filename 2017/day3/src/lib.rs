use std::collections::HashMap;

#[derive(Debug, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn get_next_insert(x: i32, y: i32, map: &HashMap<(i32, i32), i32>) -> i32 {
    map.get(&(x + 1, y - 1)).unwrap_or(&0) + map.get(&(x + 1, y)).unwrap_or(&0)
        + map.get(&(x + 1, y + 1)).unwrap_or(&0) + map.get(&(x, y - 1)).unwrap_or(&0)
        + map.get(&(x, y + 1)).unwrap_or(&0) + map.get(&(x - 1, y - 1)).unwrap_or(&0)
        + map.get(&(x - 1, y)).unwrap_or(&0) + map.get(&(x - 1, y + 1)).unwrap_or(&0)
}

impl Point {
    pub fn from_number(input: i32) -> Point {
        let mut moves: Vec<(i32, i32)> = Vec::new();
        {
            let mut x = 0;
            let mut y = 0;
            let mut ring = 1;

            moves.push((x, y));

            while moves.len() < input as usize {
                ring += 1;
                let side_length = ring * 2 - 1;

                // move to next ring
                x += 1;
                moves.push((x, y));

                for _ in 1..(side_length - 1) {
                    y += 1;
                    moves.push((x, y));
                }

                for _ in 1..side_length {
                    x -= 1;
                    moves.push((x, y));
                }

                for _ in 1..side_length {
                    y -= 1;
                    moves.push((x, y));
                }

                for _ in 1..side_length {
                    x += 1;
                    moves.push((x, y));
                }
            }
        }
        let (x, y) = moves[(input - 1) as usize];

        Point { x, y }
    }
    pub fn from_number2(input: i32) -> Point {
        let mut map = HashMap::new();
        let mut moves = Vec::new();
        {
            let mut x = 0;
            let mut y = 0;
            let mut ring = 1;

            let insert = 1;
            map.insert((x, y), insert);
            moves.push((x, y, insert));

            while moves.last().map(|x| x.2).unwrap_or(0) < input {
                println!("ring{}", ring);
                //println!("{:?}", moves);
                ring += 1;
                let side_length = ring * 2 - 1;

                // move to next ring
                x += 1;
                let insert = get_next_insert(x, y, &map);
                map.insert((x, y), insert);
                moves.push((x, y, insert));

                for _ in 1..(side_length - 1) {
                    y += 1;
                    let insert = get_next_insert(x, y, &map);
                    map.insert((x, y), insert);
                    moves.push((x, y, insert));
                }

                for _ in 1..side_length {
                    x -= 1;
                    let insert = get_next_insert(x, y, &map);
                    map.insert((x, y), insert);
                    moves.push((x, y, insert));
                }

                for _ in 1..side_length {
                    y -= 1;
                    let insert = get_next_insert(x, y, &map);
                    map.insert((x, y), insert);
                    moves.push((x, y, insert));
                }

                for _ in 1..side_length {
                    x += 1;
                    let insert = get_next_insert(x, y, &map);
                    map.insert((x, y), insert);
                    moves.push((x, y, insert));
                }
            }
        }
        for (x, y, value) in moves {
            if value > input {
                println!("{}", value);
                return Point { x, y };
            }
        }
        panic!("Invalid input!");
    }
}
