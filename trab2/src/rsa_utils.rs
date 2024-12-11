use crate::primes;

pub fn calculate_private_key(e: u64, p: u64, q: u64) -> u64 {
    let phi = (p - 1) * (q - 1);
    mod_inverse(e, phi).expect("Erro ao calcular o inverso modular.")
}

fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    let (mut t, mut new_t): (i64, i64) = (0, 1);
    let (mut r, mut new_r): (i64, i64) = (m as i64, a as i64);

    while new_r != 0 {
        let quotient = r / new_r;

        t = t - quotient * new_t;
        r = r - quotient * new_r;

        std::mem::swap(&mut t, &mut new_t);
        std::mem::swap(&mut r, &mut new_r);
    }

    if r > 1 {
        None
    } else {
        if t < 0 {
            t += m as i64;
        }
        Some(t as u64)
    }
}

pub fn decrypt(encrypted_data: Vec<u64>, d: u64, n: u64) -> Vec<u8> {
    encrypted_data
        .iter()
        .map(|&c| primes::mod_exp(c, d, n) as u8)
        .collect()
}