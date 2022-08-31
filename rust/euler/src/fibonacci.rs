pub struct Fibonacci {
    previous: u32,
    current: u32,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self {
            previous: 0,
            current: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.previous + self.current;
        self.previous = self.current;
        self.current = next;
        Some(self.current)
    }
}
