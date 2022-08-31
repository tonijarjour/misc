pub fn check(n: u64) -> u64 {
    let mut multiple = 1;
    for i in 2..=n {
        multiple = lcm(i, multiple);
    }
    multiple
}

fn lcm(a: u64, b: u64) -> u64 {
    a*b/gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let lo = std::cmp::min(a, b);
    let hi = std::cmp::max(a, b);
    if  hi % lo != 0 {
        return gcd(lo, hi % lo);
    }
    lo
}
