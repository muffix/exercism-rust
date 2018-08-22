pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    Some(collatz_iter(n).take_while(|&n| n != 1).count() as u64)
}

fn collatz_iter(mut start: u64) -> impl Iterator<Item = u64> {
    std::iter::repeat(()).map(move |()| {
        let curr = start;
        start = if start % 2 == 0 { start / 2 } else { 3 * start + 1 };
        curr
    })
}
