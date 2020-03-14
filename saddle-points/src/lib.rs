// "saddle point" is greater than or equal to every element in its row
// and less than or equal to every element in its column.
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() { return Vec::<(usize, usize)>::new() }

    let row_count = input.len();
    let col_count = input[0].len();

    let mut max_of_rows = Vec::<u32>::new();
    let mut min_of_cols = Vec::<u32>::new();

    for (row_idx, row) in input.iter().enumerate() {
        print!("{} {:?} \n", row_idx, row);

    }

    vec![(0, 0)]
}
