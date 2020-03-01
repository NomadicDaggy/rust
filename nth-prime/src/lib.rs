// DOENST WORK, SHOULD CONT WITH https://www.geeksforgeeks.org/program-to-find-the-nth-prime-number/
pub fn nth(n: u32) -> u32 {
    // I'm going to try and implement the Sieve of Eratosthenes

    // initiallize with n+1 of true
    let mut sieve = vec![true; m + 1];

    let mut prime = 2_usize;

    // Sieve through
    while prime * prime <= m {
        if sieve[prime] == true {
            // Update multiples of p
            for i in (prime * prime..m + 1).step_by(prime) {
                sieve[i] = false;
            }
        }
        prime += 1;
    }

    println!("{:#?}", sieve);

    // Find nth prime
    let mut counter = 0;
    for i in 0..m {
        if sieve[i] == true {
            counter += 1;
        }

        if counter == n {
            return i as u32;
        }
    }

    0
}
