pub fn check(number: u32) -> u32 {
    (1..number)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum()
}
