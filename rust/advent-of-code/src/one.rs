use std::fs;

pub fn solve(window_size: usize) -> usize {
    fs::read_to_string("input/01.txt")
        .unwrap()
        .lines()
        .map(|d| d.parse().unwrap())
        .collect::<Vec<u16>>()
        .windows(window_size)
        .map(|window| window.iter().sum())
        .collect::<Vec<u16>>()
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count()
}
