# Trabalho 1

Nome: Daniella Martins Vasconcellos

## Enunciado

Baseado na implementação do AES:

- Trocar a Caixa-S do AES por alguma outra cifra de substituição (exceto Cifra de Cesar)
- Calcular o tempo total para cifrar e decifrar arquivos de tamanhos diversos (serão disponibilizados) 
- Calcular o custo para cifrar e decifrar de cada fase do algoritmo.
- Comparar o tempo da implementação de voces com alguma implementação da OpenSSL

## Explicação

Dentro da pasta [src](https://github.com/danmvas/CRIPTOGRAFIA-CI1017/trab1/src) há três arquivos: [aes.rs](https://github.com/danmvas/CRIPTOGRAFIA-CI1017/trab1/src/aes.rs), [main.rs](https://github.com/danmvas/CRIPTOGRAFIA-CI1017/trab1/src/main.rs), [openssl.rs](https://github.com/danmvas/CRIPTOGRAFIA-CI1017/trab1/src/openssl.rs).

Já na pasta [test_files](https://github.com/danmvas/CRIPTOGRAFIA-CI1017/trab1/test_files), há cinco arquivos: decrypted_custom.txt, decrypted_openssl.txt, encrypted_custom.txt, encrypted_openssl e input.txt.

- decrypted_custom.txt e decrypted_openssl: inicialmente vazio, depois da execução do código irá ter o texto decifrado escrito.
- encrypted_custom.txt e encrypted_openssl: contém os arquivos pós criptografia.
- input.txt: arquivo de texto a ser encriptografado.

### main.rs

Arquivo onde irá chamar as execuções e comparação do AES customizado e o AES do OpenSSL.

### openssl.rs

Arquivo onde executa o AES pela implementação do OpenSSL (crate *openssl*). Chama as funções e calcula os tempos de execução.

### aes.rs

Arquivo do AES customizado. A Caixa-S foi trocada por uma substituição baseada em XOR (cifra de Vernam simplificada). O XOR transforma o valor do byte no estado, utilizando uma chave fixa como modificador adicional e uma chave dinâmica como segurança a mais. Torna-se uma transformação linear em comparação à Caixa-S padrão, também sendo menos segura. 

O arquivo aes_phases.rs irá printar uma versão mais detalhadada dos tempos de cada fase do algoritmo.

## Execução

Dentro do repositório, fazer o comando:

``cargo run``

O output esperado é:

```
-------------------------------------------
Tempo para AES customizado:
Tempo de encriptação AES: [ tempo ]
Tempo de decriptação AES: [ tempo ]
-------------------------------------------
Tempo para OpenSSL AES:
OpenSSL tempo encriptografia: [ tempo ]
OpenSSL decriptografia: [ tempo ]
-------------------------------------------
```