// Math from: https://stackoverflow.com/questions/15593428/function-to-calculate-the-difference-between-sum-of-squares-and-square-of-sums

pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    //((3*n).pow(2) + 2*n) * (1 - n.pow(2)) / 12
    square_of_sum(n) - sum_of_squares(n)
}
