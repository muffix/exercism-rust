#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {
        return None;
    }

    let aliquot_sum: u64 = (1..(num / 2 + 1))
        .filter(|n| num % n == 0)
        .sum();

    if aliquot_sum < num {
        return Some(Classification::Deficient);
    }

    if aliquot_sum > num {
        return Some(Classification::Abundant);
    }

    Some(Classification::Perfect)
}
