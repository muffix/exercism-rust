pub fn raindrops(n: usize) -> String {
    let mut sound = String::from("");

    if n % 3 == 0 {
        sound = format!("{}Pling", sound);
    }

    if n % 5 == 0 {
        sound = format!("{}Plang", sound);
    }

    if n % 7 == 0 {
        sound = format!("{}Plong", sound);
    }

    if sound == "" {
        sound = format!("{}", n);
    }

    return sound;
}
