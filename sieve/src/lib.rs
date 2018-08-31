pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut vec: Vec<_> = (2..upper_bound + 1).collect();

    for p in 2..(upper_bound + 1) {
        vec.retain(|&x| x <= p || x % p != 0);
    }

    vec
}
