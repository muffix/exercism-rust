pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut t = PascalsTriangle { rows: Vec::new() };

        for i in 0..row_count {
            let mut r = Vec::new();
            for j in 0..(i + 1) {
                r.push(binom(i, j))
            }
            t.rows.push(r);
        }
        t
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.to_vec()
    }
}

fn binom(n: u32, mut k: u32) -> u32 {
    let mut res = 1;

    if k > n - k {
        k = n - k;
    }

    for i in 0..k {
        res *= n - i;
        res /= i + 1;
    }

    res
}
