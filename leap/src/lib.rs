pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 {
        // we still have to check if its not divisible by 100
        if year % 400 == 0 {
            // if div by 400, its definitely a leap year
            return true;
        } else if year % 100 == 0 {
            // if div by 100, definitely not a leap y
            return false;
        }

        return true;
    } else {
        // If year not divisible by 4, its definitely not a leap year
        return false;
    }
}
