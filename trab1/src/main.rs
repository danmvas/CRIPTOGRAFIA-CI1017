mod aes;
mod openssl;
// use std::env;

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    let key = vec![0x00; 32];
    let file_path = "test_files/input.txt";
    
    println!("-------------------------------------------\nTempo para AES customizado:");
    aes::measure_custom_aes(file_path, &key);

    println!("-------------------------------------------\nTempo para OpenSSL AES:");
    openssl::measure_openssl_aes(file_path, &key);
    println!("-------------------------------------------");
}
