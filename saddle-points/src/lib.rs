pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();

    for (r, row) in input.iter().enumerate() {
        for (c, &element) in row.iter().enumerate() {
            let row_slice = row.iter().map(|&el| el);
            let col_slice = input.iter().map(|row| row[c]);

            if element == row_slice.max().unwrap() && element == col_slice.min().unwrap() {
                saddle_points.push((r, c))
            }
        }
    }
    return saddle_points;
}
