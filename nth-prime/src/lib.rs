pub fn nth(n: u32) -> Option<u32> {
    if n < 1 {
        return None;
    }

    let mut primes = vec![2, 3, 5];
    let mut next = 7;

    while primes.len() < (n as usize) {
        let mut is_prime = true;
        for p in &primes {
            if p * p > next {
                break;
            }

            if next % p == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(next)
        }
        next += 2;
    }

    return Some(primes[(n - 1) as usize]);
}
