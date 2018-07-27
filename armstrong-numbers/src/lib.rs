pub fn is_armstrong_number(num: u32) -> bool {
    let digits = count_digits(num);
    let mut rem = num;
    let mut sum = 0;

    while rem > 0 {
        sum += (rem % 10).pow(digits);
        rem /= 10;
    }

    println!("Digits: {}, sum: {}, num: {}", digits, sum, num);

    return num == sum;
}

fn count_digits(number: u32) -> u32 {
    let mut n = number;
    let mut i = 1;

    if n >= 100_000_000 {
        i += 8;
        n /= 100_000_000;
    }

    if n >= 10_000 {
        i += 4;
        n /= 10_000;
    }

    if n >= 100 {
        i += 2;
        n /= 100;
    }

    if n >= 10 {
        i += 1;
    }

    return i;
}
