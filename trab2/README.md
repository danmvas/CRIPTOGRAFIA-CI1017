# Trabalho 2

Nome: Daniella Martins Vasconcellos

## Enunciado

Dado um texto cifrado com o RSA e sabendo que p e q são números primos menores que 1024 (10 bits), faça um programa que encontre a chave privada d sendo que a chave pública {e , n} é conhecida.

## Explicação

Dentro da pasta [src](https://github.com/danmvas/CRIPTOGRAFIA-CI1017/trab2/src) há quatro arquivos: [io_utils.rs](https://github.com/danmvas/CRIPTOGRAFIA-CI1017/trab2/src/io_utils.rs), [main.rs](https://github.com/danmvas/CRIPTOGRAFIA-CI1017/trab2/src/main.rs), [primes.rs](https://github.com/danmvas/CRIPTOGRAFIA-CI1017/trab2/src/primes.rs) e [rsa_utils.rs](https://github.com/danmvas/CRIPTOGRAFIA-CI1017/trab2/src/rsa_utils.rs).

Já na pasta [test_files](https://github.com/danmvas/CRIPTOGRAFIA-CI1017/trab2/test_files), há três arquivos: decrypted.txt, encrypted.txt e public_key.txt.

- decrypted.txt: inicialmente vazio, depois da execução do código irá ter o texto decifrado escrito.
- encrypted.txt: contém os valores inteiros criptografados.
- public_key.txt: contém dois inteiros em duas linhas diferentes. O primeiro é o valor de "e", o segundo é o "n".

### io_utils.rs

Arquivo de leitura e processamento das informações dos arquivos texto.

### main.rs

Arquivo no qual irão ser chamadas as funções principais dos arquivos. Nela é calculada a chave privada a partir da chave pública e faz a decriptação do texto.

### primes.rs

Arquivo no qual são encontrados os primos a partir de n. Também faz a aplicação do algoritmo de Miller-Rabin para a verificação da primalidade dos números encontrados.

### rsa_utils.rs

Arquivo onde é calculado a chave privada através do inverso modular, junto da decriptografia do arquivo fornecido.

## Execução

Dentro do repositório, fazer o comando:

``cargo run``

O output esperado é:

```
-------------------------------------------
Primos encontrados: p = [valor], q = [valor]
Chave privada d encontrada: [valor]
Tempo para cálculo da chave privada: [tempo]
Tempo para decriptação: [tempo]
Tempo total de execução: [tempo]
-------------------------------------------
```

O arquivo `decrypted.txt` que inicialmente está vazio deve aparecer com a frase agora formada.