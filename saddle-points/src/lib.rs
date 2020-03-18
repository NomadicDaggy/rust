// "saddle point" is greater than or equal to every element in its row
// and less than or equal to every element in its column.
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::<(usize, usize)>::new();
    
    if input.iter().any(|vec| vec.is_empty()) { return saddle_points; }

    // For each row, find its maximum value and check if it is also
    // the value's column's minimum.
    for (row_idx, row) in input.iter().enumerate() {
        let row_max = row.iter().max().unwrap();
        let row_saddles: Vec::<(usize, usize)> = row
            .iter()
            .enumerate()
            .filter(|(i, x)| *x == row_max && is_col_min(*i, **x, input))
            .map(|(i, _)| (row_idx, i))
            .collect();
        
        for r in row_saddles.iter() { saddle_points.push(*r) }
    }
    saddle_points
}

// Is value x the minimum of column i
fn is_col_min(col_idx: usize, row_max: u64, input: &[Vec<u64>]) -> bool {
    input.iter().all(
        |row| row
            .iter()
            .enumerate()
            .filter(|(i, _)| *i == col_idx)  // If it is the right column
            .all(|(_, x)| row_max <= *x)  // If row maximum is also col minimum
    )
}
