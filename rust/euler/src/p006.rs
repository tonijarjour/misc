pub fn check(n: u32) -> u32 {
    let mut sum = 1;
    let sqs = (2..=n).fold(1, |acc, i| {
        sum += i;
        acc + i * i
    });
    sum * sum - sqs
}
