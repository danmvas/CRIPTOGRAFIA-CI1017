use openssl::symm::{Cipher, Crypter, Mode};
use std::time::Instant;
use std::fs;

pub fn measure_openssl_aes(file_path: &str, key: &[u8]) {
    let data = fs::read(file_path).expect("Não foi possível ler o arquivo");

    let cipher = Cipher::aes_256_cbc();
    let iv = vec![0; cipher.iv_len().unwrap_or(16)];
    
    let start_encrypt = Instant::now();
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, Some(&iv)).unwrap();
    let mut encrypted_data = vec![0; data.len() + cipher.block_size()];
    let mut count = crypter.update(&data, &mut encrypted_data).unwrap();
    count += crypter.finalize(&mut encrypted_data[count..]).unwrap();
    encrypted_data.truncate(count);
    let encrypt_duration = start_encrypt.elapsed();
    println!("OpenSSL tempo encriptografia: {:?}", encrypt_duration);
    
    fs::write("test_files/encrypted_openssl.txt", &encrypted_data)
        .expect("Não foi possível escrever no arquivo encrypted_openssl");
    
    let start_decrypt = Instant::now();
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, Some(&iv)).unwrap();
    let mut decrypted_data = vec![0; encrypted_data.len() + cipher.block_size()];
    let mut count = crypter.update(&encrypted_data, &mut decrypted_data).unwrap();
    count += crypter.finalize(&mut decrypted_data[count..]).unwrap();
    decrypted_data.truncate(count);

    let decrypt_duration = start_decrypt.elapsed();
    println!("OpenSSL decriptografia: {:?}", decrypt_duration);
    
    fs::write("test_files/decrypted_openssl.txt", &decrypted_data).expect("Não foi possível escrever no arquivo decrypted_openssl");

    assert_eq!(data, decrypted_data);
}
