extern crate rand;

use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range::<u64>(2, p)
}

pub fn public_key(p: u64, mut g: u64, mut a: u64) -> u64 {
    if a == 1 {
        return 0;
    }

    let mut res = 1;

    while a > 0 {
        if a % 2 == 1 {
            res = (res * g) % p;
        }
        a >>= 1;
        g = g.pow(2) % p;
    }
    res
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key(p, b_pub, a)
}
