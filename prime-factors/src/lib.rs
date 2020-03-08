pub fn factors(n: u64) -> Vec<u64> {
    // Get primes up to n/2
    let primes = build_sieve(((n / 2) + 1) as usize);

    let mut out = Vec::<u64>::new();
    let mut n = n;

    for (idx, is_prime) in primes.iter().enumerate() {
        if idx == 0 || idx == 1 {
            continue;
        }
        if *is_prime == true {
            while n % (idx as u64) == 0 {
                n = n / (idx as u64);
                out.push(idx as u64);
            }
            if n < 2 {
                return out;
            }
        }
    }
    out
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
