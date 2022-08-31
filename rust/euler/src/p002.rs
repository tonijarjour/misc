use crate::fibonacci;
pub fn check(n: u32) -> u32 {
    fibonacci::Fibonacci::new()
        .take_while(|i| *i <= n)
        .filter(|i| i % 2 == 0)
        .sum()
}
