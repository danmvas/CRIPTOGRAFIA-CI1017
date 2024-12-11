mod rsa_utils;
mod primes;
mod io_utils;

use std::time::Instant;

fn main() {
    let start_total = Instant::now();

    // Início do cálculo da chave privada
    let start_key_calc = Instant::now();
    let (e, n) = io_utils::read_public_key("test_files/public_key.txt");
    let (p, q) = primes::find_primes(n).expect("Erro ao encontrar os primos.");

    println!("Primos encontrados: p = {}, q = {}", p, q);
    if !primes::is_prime(p, 5) || !primes::is_prime(q, 5) {
        panic!("Os números p ou q não são primos!");
    }

    let d = rsa_utils::calculate_private_key(e, p, q);
    let elapsed_key_calc = start_key_calc.elapsed();

    println!("Chave privada d encontrada: {}", d);
    println!("Tempo para cálculo da chave privada: {:.2?}", elapsed_key_calc);

    // Início da decriptação
    let start_decrypt = Instant::now();
    let encrypted_data = io_utils::read_encrypted_file("test_files/encrypted.txt");
    let decrypted_data = rsa_utils::decrypt(encrypted_data, d, n);
    io_utils::write_decrypted_file("test_files/decrypted.txt", decrypted_data);
    let elapsed_decrypt = start_decrypt.elapsed();

    println!("Tempo para decriptação: {:.2?}", elapsed_decrypt);

    // Tempo total
    let elapsed_total = start_total.elapsed();
    println!("Tempo total de execução: {:.2?}", elapsed_total);
}
