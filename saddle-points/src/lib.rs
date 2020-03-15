// "saddle point" is greater than or equal to every element in its row
// and less than or equal to every element in its column.
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::<(usize, usize)>::new();
    
    if input.is_empty() { return saddle_points; }

    // For each row, find its maximum value and check if it is also
    // the values columns minimum.
    for (row_idx, row) in input.iter().enumerate() {
        let row_max_val = row.iter().max().unwrap();
        let row_max: Vec::<(usize, &u64)> = row
            .iter()
            .enumerate()
            .filter(|(i, x)| *x == row_max_val)
            .collect();
        
        //println!("{:?}", row_max);

        // Keep only those row maximums which are also column minimums.
        // Additionally, replace second element with row index.
        let saddles = row_max
            .iter()
            .filter(|(i, x)| is_min_of_col(i, x))
            .collect();
    }
    
    saddle_points
}