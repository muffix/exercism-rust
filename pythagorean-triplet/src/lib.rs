pub fn find() -> Option<u32> {
    let max = 1000;

    for a in 1..max {
        for b in a..max {
            let candidate = a*a+b*b;
            let root = (candidate as f32).sqrt() as u32;
            
            if root*root == candidate && a+b+root == 1000 {
                return Some(a*b*root);
            }
        }
    }

    return None
}
