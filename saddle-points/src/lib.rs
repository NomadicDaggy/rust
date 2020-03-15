// "saddle point" is greater than or equal to every element in its row
// and less than or equal to every element in its column.
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() {
        return Vec::<(usize, usize)>::new();
    }

    let mut res = Vec::<(usize, usize)>::new();

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
        // For each maximum row value, see if minimum coincides
        for i in rowmax_idxs {
            // If a cell is smaller than this and not in column position i,
            // then there is no saddle point in this row.
            // Holds value of row maximum.
            let value_to_beat = input[row_idx][i as usize];
            let mut no_saddle_flag = false;
            let mut saddle_rows = Vec::<usize>::new();
            // For each row
            for (idx, row) in input.iter().enumerate() {
                println!("{:?} {}", row, idx);
                // we have the right column
                for (i2, elem) in row.iter().enumerate() {
                    if i2 == i as usize {
                        // new minimum found
                        if *elem < value_to_beat {
                            no_saddle_flag = true;
                        } else {
                            saddle_rows.push(idx);
                            //res.push((idx, i as usize));
                        }
                        println!("i:{} idx:{} flag:{}", i, idx, no_saddle_flag);
                    }
                }
            }

            if !no_saddle_flag {
                for j in saddle_rows {
                    res.push((j as usize, i as usize));
                }
            }
        }
    }
    res
}
