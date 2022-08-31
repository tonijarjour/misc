pub fn check(n: u32) -> u32 {
    let mut largest = 1;
    let max = 10_u32.pow(n);
    let min = (max + 1) / 10;
    for i in (min..max).rev() {
        for r in (min..=i).rev() {
            if i * r <= largest || (i * r) % 11 != 0 { continue; }
            if (i * r).to_string().chars()
                .eq((i * r).to_string().chars().rev()) {
                largest = i * r;
            }
        }
    }
    largest
}
