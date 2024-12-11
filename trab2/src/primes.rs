use rand::Rng;

pub fn find_primes(n: u64) -> Option<(u64, u64)> {
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            let q = n / i;
            if is_prime(i, 5) && is_prime(q, 5) {
                return Some((i, q));
            } else {
                panic!("Os números encontrados não são primos!");
            }
        }
    }
    None
}

/// Miller-Rabin: O(log(N))
pub fn is_prime(n: u64, k: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }

    let mut rng = rand::thread_rng();

    'outer: for _ in 0..k {
        let a = rng.gen_range(2..n - 1);
        let mut x = mod_exp(a, d, n);

        if x == 1 || x == n - 1 {
            continue;
        }

        let mut d_temp = d;
        while d_temp != n - 1 {
            x = mod_exp(x, 2, n);
            d_temp *= 2;

            if x == 1 {
                return false;
            }
            if x == n - 1 {
                continue 'outer;
            }
        }

        return false;
    }
    true
}

pub fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp /= 2;
        base = (base * base) % modulus;
    }

    result
}
