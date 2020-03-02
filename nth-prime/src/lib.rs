// Works with primes that are smaller than 4'000'000.
// I could make it do the whole u32 limit slowly, but then
// I would need to know a number that is closely above the nth prime.
// Searched around a bit for that and couldn't find anything good,
// so lets leave it like this for now.

pub fn nth(n: u32) -> u32 {
    let n = n as usize;

    // 4 000 000 is a random number. 40mil was way too slow,
    // not even talking about u32 limit...
    // If you want to make it run with full u32, change this to 2_147_483_647
    let sieve = build_sieve(4_000_000usize);

    // i32 for later when I want to make it negative
    let mut m = n as i32;

    // Find nth prime
    for (idx, val) in sieve.iter().enumerate() {
        // I'm not entirely clear on why you can't compare a
        // reference to a value. Is it not obvious that I want
        // the value that the reference is referencing to?
        if *val == true {
            m -= 1;
        }

        // Putting -3 here is kind of a hacky solution to primes
        // starting at 2, but index at 0.
        if m == -3 {
            return idx as u32;
        }
    }

    // Wonder what the correct thing to do here is...
    1
}

fn build_sieve(n: usize) -> Vec<bool> {
    // I'm going to try and implement the Sieve of Eratosthenes

    // initiallize with n+1 of true
    let mut sieve = vec![true; n + 1];

    // Compilator complained about indexing with u32, so I changed it here
    let mut prime = 2_usize;

    // Sieve through
    while prime * prime <= n {
        if sieve[prime] == true {
            // Update multiples of p
            for i in (prime * prime..n + 1).step_by(prime) {
                sieve[i] = false;
            }
        }
        prime += 1;
    }

    // returns boolean array where sieve[i] holds the info:
    // "Is i-1 a prime"
    sieve
}
