pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    2_u64.pow(s - 1) as u64
}

pub fn total() -> u64 {
    // I can't see any reason not to hardcode it
    // Otherwise this works: (square(64) - 1) * 2 + 1
    18_446_744_073_709_551_615_u64
}
