pub fn spiral_matrix(size: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::with_capacity(size);

    if size == 0 {
        return res;
    }

    for i in 0..size {
        res.push(Vec::with_capacity(size));
        for _ in 0..size {
            res[i].push(0);
        }
    }

    let (mut left, mut top, mut right, mut bottom) = (0, 0, size - 1, size - 1);
    let mut i = 1;

    while left < right {
        for c in left..right + 1 {
            res[top][c] = i;
            i += 1;
        }
        top += 1;

        for r in top..bottom + 1 {
            res[r][right] = i;
            i += 1;
        }
        right -= 1;

        for c in (left..right + 1).rev() {
            res[bottom][c] = i;
            i += 1;
        }
        bottom -= 1;

        for r in (top..bottom + 1).rev() {
            res[r][left] = i;
            i += 1;
        }
        left += 1;
    }

    if size % 2 == 1 {
        res[top][left] = i;
    }

    res
}
