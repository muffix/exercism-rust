pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    while n % 2 == 0 {
        factors.push(2);
        n >>= 1
    }

    let mut f = 3;

    while n > 1 {
        while n % f == 0 {
            factors.push(f);
            n /= f;
        }
        f += 2;
    }

    factors
}
