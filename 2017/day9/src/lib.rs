use std::str::Chars;

pub fn count_score(input: &str) -> (i32, i32) {
    let mut stream = input.chars();

    let mut score = 0;
    let mut depth = 0;
    let mut garbage_counter = 0;

    while let Some(character) = stream.next() {
        match character {
            '{' => {
                depth += 1;
                score += depth;
            }
            '}' => depth -= 1,
            '<' => garbage_counter += consume_garbage(&mut stream),
            '!' => {
                stream.next();
            }
            _ => (),
        };
    }

    (score, garbage_counter)
}

fn consume_garbage(stream: &mut Chars) -> i32 {
    let mut counter = 0;
    while let Some(character) = stream.next() {
        if character == '>' {
            break;
        }
        if character == '!' {
            stream.next();
            continue;
        }
        counter += 1;
    }

    counter
}
