// "saddle point" is greater than or equal to every element in its row
// and less than or equal to every element in its column.
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() {
        return Vec::<(usize, usize)>::new();
    }

    //let row_count = input.len();
    //let col_count = input[0].len();

    //let mut min_of_cols = Vec::<u64>::new();
    let mut max_of_rows = Vec::<Vec<u64>>::new();
    // Find max idx of each row (there can be more than one)
    for (row_idx, row) in input.iter().enumerate() {
        let mut rowmax_val = 0_u64;
        let mut rowmax_idxs = Vec::<u64>::new();

        for (i, val) in row.iter().enumerate() {
            if *val > rowmax_val {
                rowmax_idxs = Vec::<u64>::new();
                rowmax_val = *val;
                rowmax_idxs.push(i as u64);
            } else if *val == rowmax_val {
                rowmax_idxs.push(i as u64);
            }
        }
        //print!("{:?}\n", rowmax_idxs);
        max_of_rows.push(rowmax_idxs);
        //print!("{} {:?}", row_idx, row);
    } // After this, max_of_rows has indices for
      //each row where its max can be found
      //println!("{:?}", max_of_rows);
    vec![(0, 0)]
}
