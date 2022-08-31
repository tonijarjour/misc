pub fn check(number: u64) -> u64 {
    for n in (2..(number as f64).sqrt() as u64).rev() {
        if number % n == 0 && (2..n).all(|i| n % i != 0) {
            return n;
        }
    }
    number
}
