use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn read_public_key(file_path: &str) -> (u64, u64) {
    let file = File::open(file_path).expect("Erro ao abrir o arquivo da chave pública.");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let e = lines
        .next()
        .expect("Erro ao ler a chave pública.")
        .expect("Erro ao processar a linha.")
        .parse::<u64>()
        .expect("Erro ao converter e para u64.");
    let n = lines
        .next()
        .expect("Erro ao ler a chave pública.")
        .expect("Erro ao processar a linha.")
        .parse::<u64>()
        .expect("Erro ao converter n para u64.");

    (e, n)
}

pub fn read_encrypted_file(file_path: &str) -> Vec<u64> {
    let file = File::open(file_path).expect("Erro ao abrir o arquivo criptografado.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.expect("Erro ao ler uma linha.")
                .parse::<u64>()
                .expect("Erro ao converter linha para u64.")
        })
        .collect()
}

pub fn write_decrypted_file(file_path: &str, data: Vec<u8>) {
    let mut file = File::create(file_path).expect("Erro ao criar o arquivo descriptografado.");
    file.write_all(&data).expect("Erro ao escrever no arquivo.");
}
