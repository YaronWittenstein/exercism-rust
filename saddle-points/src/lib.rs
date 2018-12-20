pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    let row_max = find_rows_max(input);
    let col_min = find_cols_min(input);

    for (i, row) in input.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let i_max = row_max.get(i).unwrap();
            let j_min = col_min.get(j).unwrap();

            if *cell >= *i_max && *cell <= *j_min {
                res.push((i, j));
            }
        }
    }

    res
}

fn find_rows_max(input: &[Vec<u64>]) -> Vec<u64> {
    let mut res = Vec::with_capacity(input.len());

    for row in input.iter() {
        let mut max = None;

        for cell in row.iter() {
            if let Some(val) = max {
                if val < *cell {
                    max = Some(*cell);
                }
            } else {
                max = Some(*cell);
            }
        }

        if let Some(max_val) = max {
            res.push(max_val);
        }
    }

    res
}

fn find_cols_min(input: &[Vec<u64>]) -> Vec<u64> {
    if input.len() == 0 {
        return Vec::new();
    }

    let cols = input[0].len();
    let mut res = Vec::with_capacity(cols);

    for j in 0..cols {
        let mut min = None;

        for row in input.iter() {
            let cell: &u64 = row.get(j).unwrap();

            if let Some(val) = min {
                if val > *cell {
                    min = Some(*cell);
                }
            } else {
                min = Some(*cell);
            }
        }

        if let Some(min_val) = min {
            res.push(min_val);
        }
    }

    res
}
