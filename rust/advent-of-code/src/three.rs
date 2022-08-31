use std::fs;

pub fn solve() -> u32 {
    let data: Vec<String> = fs::read_to_string("input/03.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut columns = [
        vec![], vec![], vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![], vec![], vec![],
    ];

    for read in &data {
        for (i, b) in read.chars().enumerate() {
            columns[i].push(b.to_digit(10).unwrap());
        }
    }

    let gamma = u32::from_str_radix(&columns.clone()
            .map(|column| column.iter().sum::<u32>())
            .map(|total| if total > 500 { '1' } else { '0' })
            .iter().collect::<String>(), 2).unwrap();

    gamma * (!gamma & 0b111111111111)
}