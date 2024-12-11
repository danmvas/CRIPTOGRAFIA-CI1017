use std::time::Instant;
use std::fs;
use std::sync::Arc;
use std::thread;

const BLOCK_SIZE: usize = 16;
const NUM_ROUNDS: usize = 14;

fn add_padding(data: &[u8]) -> Vec<u8> {
    let padding_size = BLOCK_SIZE - (data.len() % BLOCK_SIZE);
    let mut padded_data = vec![padding_size as u8; data.len() + padding_size];
    padded_data[..data.len()].copy_from_slice(data);
    padded_data
}

fn remove_padding(data: &[u8]) -> &[u8] {
    let padding_size = *data.last().unwrap() as usize;
    &data[..data.len() - padding_size]
}

fn custom_aes(data: &[u8], key: &[u8]) -> Vec<u8> {
    let sub_key = 0b10101010;
    let padded_data = Arc::new(add_padding(data));
    let mut output = vec![0u8; padded_data.len()];

    let blocks: Vec<_> = (0..padded_data.len() / BLOCK_SIZE).collect();
    let num_threads = std::cmp::min(blocks.len(), 8); 
    let chunk_size = (blocks.len() + num_threads - 1) / num_threads;

    let mut handles = Vec::with_capacity(num_threads);

    for chunk in blocks.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let key = key.to_vec();
        let padded_data = Arc::clone(&padded_data);

        handles.push(thread::spawn(move || {
            let mut result = Vec::with_capacity(chunk.len() * BLOCK_SIZE);
            let mut state = vec![0u8; BLOCK_SIZE];

            for block_index in chunk {
                state[..].copy_from_slice(&padded_data[block_index * BLOCK_SIZE..(block_index + 1) * BLOCK_SIZE]);

                for _ in 0..NUM_ROUNDS {
                    for i in 0..BLOCK_SIZE {
                        state[i] ^= sub_key ^ key[i];
                    }

                    state[4..8].rotate_left(1);
                    state[8..12].rotate_left(2);
                    state[12..16].rotate_left(3);

                    let mut column_xor = [0u8; 4];
                    for i in 0..4 {
                        column_xor[i] = state[i * 4] ^ state[i * 4 + 1] ^ state[i * 4 + 2] ^ state[i * 4 + 3];
                    }
                    for i in 0..4 {
                        let col_start = i * 4;
                        state[col_start..col_start + 4]
                            .iter_mut()
                            .for_each(|b| *b ^= column_xor[i]);
                    }
                }

                result.extend_from_slice(&state);
            }

            result
        }));
    }

    let mut offset = 0;
    for handle in handles {
        let chunk_result = handle.join().expect("Thread falhou");
        output[offset..offset + chunk_result.len()].copy_from_slice(&chunk_result);
        offset += chunk_result.len();
    }

    output
}

fn custom_aes_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let sub_key = 0b10101010;
    let padded_data = Arc::new(data.to_vec());
    let mut output = vec![0u8; data.len()];

    let blocks: Vec<_> = (0..data.len() / BLOCK_SIZE).collect();
    let num_threads = std::cmp::min(blocks.len(), 8);
    let chunk_size = (blocks.len() + num_threads - 1) / num_threads;

    let mut handles = Vec::with_capacity(num_threads);

    for chunk in blocks.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let key = key.to_vec();
        let padded_data = Arc::clone(&padded_data);

        handles.push(thread::spawn(move || {
            let mut result = Vec::with_capacity(chunk.len() * BLOCK_SIZE);
            let mut state = vec![0u8; BLOCK_SIZE];

            for block_index in chunk {
                state[..].copy_from_slice(&padded_data[block_index * BLOCK_SIZE..(block_index + 1) * BLOCK_SIZE]);

                for _ in 0..NUM_ROUNDS {
                    let mut column_xor = [0u8; 4];
                    for i in 0..4 {
                        column_xor[i] = state[i * 4] ^ state[i * 4 + 1] ^ state[i * 4 + 2] ^ state[i * 4 + 3];
                    }
                    for i in 0..4 {
                        let col_start = i * 4;
                        state[col_start..col_start + 4]
                            .iter_mut()
                            .for_each(|b| *b ^= column_xor[i]);
                    }

                    state[4..8].rotate_right(1);
                    state[8..12].rotate_right(2);
                    state[12..16].rotate_right(3);

                    // substituição XOR Simples
                    for i in 0..BLOCK_SIZE {
                        state[i] ^= sub_key ^ key[i];
                    }
                }

                result.extend_from_slice(&state);
            }

            result
        }));
    }

    let mut offset = 0;
    for handle in handles {
        let chunk_result = handle.join().expect("Thread falhou");
        output[offset..offset + chunk_result.len()].copy_from_slice(&chunk_result);
        offset += chunk_result.len();
    }

    output
}

pub fn measure_custom_aes(file_path: &str, key: &[u8]) {
    let data = fs::read(file_path).expect("Não foi possível ler o arquivo input.txt");

    let start_encrypt = Instant::now();
    let encrypted_data = custom_aes(&data, key);
    let encrypt_duration = start_encrypt.elapsed();
    println!("Tempo de encriptação AES: {:?}", encrypt_duration);

    fs::write("test_files/encrypted_custom.txt", &encrypted_data)
        .expect("Não foi possível escrever no arquivo encrypted_custom");

    let start_decrypt = Instant::now();
    let decrypted_data = custom_aes_decrypt(&encrypted_data, key);
    let decrypt_duration = start_decrypt.elapsed();
    println!("Tempo de decriptação AES: {:?}", decrypt_duration);

    fs::write("test_files/decrypted_custom.txt", &decrypted_data)
        .expect("Não foi possível escrever no arquivo decrypted_custom");

    assert_eq!(data, remove_padding(&decrypted_data));
}