pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    
    let sum = digits
        .iter()
        .map(|d| d.pow(digits.len() as u32))
        .sum();

    num == sum
}
