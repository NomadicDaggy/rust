// "saddle point" is greater than or equal to every element in its row
// and less than or equal to every element in its column.
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::<(usize, usize)>::new();
    
    if input.is_empty() { return saddle_points; }

    // For each row, find its maximum value and check if it is also
    // the values columns minimum.
    for (row_idx, row) in input.iter().enumerate() {
        let row_max = row.iter().max().unwrap();
        let row_saddles: Vec::<(usize, usize)> = row
            .iter()
            .enumerate()
            .filter(|(i, x)| *x == row_max && is_col_min(*i, **x, input))
            .map(|(i, x)| format_indices(i, x, input))
            .collect();
        
        for r in row_saddles.iter() { saddle_points.push(*r) }
    }
    println!("{:?}", saddle_points); 

    saddle_points
}

// Is value x the minimum of column i
fn is_col_min(i: usize, x: u64, input: &[Vec<u64>]) -> bool {
    input.iter().any(|vec| x < vec[i])
}

// Goes from (col_index, value) to (row_index, col_index)
fn format_indices(i: usize, x: &u64, input: &[Vec<u64>]) -> (usize, usize) {
    (i, i)
}