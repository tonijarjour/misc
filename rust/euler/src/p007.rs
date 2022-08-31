pub fn check(n: f64) -> u64 {
    let lo = (n * n.ln() + n*(n.ln().ln() - 1.0)) as u64;
    let hi = (n * n.ln() + n*(n.ln().ln())) as u64;
    for i in lo..=hi {
        if (3..i).step_by(2).all(|t| i % t != 0) {
            println!("{}", i);
        }
    }
    1
}
