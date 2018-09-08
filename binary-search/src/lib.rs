pub fn find<R, T>(array: R, key: T) -> Option<usize>
where
    R: AsRef<[T]>,
    T: Ord,
{
    let array = array.as_ref();
    let (mut lo, mut hi) = (0, array.len());

    while lo < hi {
        let split = (lo + hi) / 2;

        if array[split] < key {
            lo = split + 1
        } else {
            hi = split
        }
    }

    if lo == array.len() || array[lo] != key {
        return None;
    }

    Some(lo)
}
