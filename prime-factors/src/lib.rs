// Port from https://www.geeksforgeeks.org/print-all-prime-factors-of-a-given-number/
pub fn factors(n: u64) -> Vec<u64> {
    let mut out = Vec::<u64>::new();

    // Make n mutable
    let mut n = n;

    // Return 2s until the number is odd
    while n % 2 == 0 {
        out.push(2_u64);
        n /= 2;
    }

    // sqrt() is a method only for floats
    // range with custom step can be accomplished with "range.step_by(step)"
    for i in (3..=(n as f64).sqrt() as u64).step_by(2) {
        while n % i == 0 {
            out.push(i as u64);
            n /= i;
        }
    }

    if n > 2 {
        out.push(n)
    }

    out
}
