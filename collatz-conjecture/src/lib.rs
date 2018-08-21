struct Collatz {
    curr: u64,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let res = self.curr;
        self.curr = match self.curr {
            n if n == 1 => 1,
            n if n % 2 == 0 => n / 2,
            n => n * 3 + 1,
        };
        Some(res)
    }
}

impl Collatz {
    pub fn new(start: u64) -> Collatz {
        Collatz { curr: start }
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    Some(Collatz::new(n).take_while(|&n| n != 1).count() as u64)
}
