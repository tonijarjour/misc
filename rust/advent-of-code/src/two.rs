use std::fs;

pub fn solve() -> (u32, u32) {
    let (aim, depth, horizontal) = fs::read_to_string("input/02.txt")
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|command| {
            (
                command[0].chars().next().unwrap(),
                command[1].parse().unwrap(),
            )
        })
        .collect::<Vec<(char, u32)>>()
        .iter()
        .fold(
            (0, 0, 0),
            |(mut aim, mut depth, mut horizontal), (command, delta)| {
                match command {
                    'f' => {
                        horizontal += delta;
                        depth += aim * delta
                    }
                    'd' => aim += delta,
                    'u' => aim -= delta,
                    _ => (),
                }
                (aim, depth, horizontal)
            },
        );

    (horizontal * aim, horizontal * depth)
}
