pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;

    for n in 1..limit {
        if factors.iter().any(|&f| n % f == 0) {
            sum += n;
        }
    }

    sum
}
