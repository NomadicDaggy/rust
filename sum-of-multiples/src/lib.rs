/*
 * Given a number, find the sum of all the unique multiples
 * of particular numbers up to but not including that number.
 * If we list all the natural numbers below 20 that are multiples
 * of 3 or 5, we get 3, 5, 6, 9, 10, 12, 15, and 18.
 * The sum of these multiples is 78.
 */

use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // HashSet should keep the multiples unique
    let mut unique_multiples = HashSet::new();

    for factor in factors.iter() {
        // So we don't divide by 0 next
        if *factor == 0 {
            continue;
        }

        // limit / factor will floor to an integer
        for multiplier in 1..=(limit / factor) {
            unique_multiples.insert(factor * multiplier);
        }
    }

    // Remove limit if it was added
    if unique_multiples.contains(&limit) {
        unique_multiples.remove(&limit);
    }

    unique_multiples.iter().sum()
}
