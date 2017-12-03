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
}
