# crypto42

[![pipeline status](https://gitlab.com/bloom42/libs/crypto42/badges/dev/pipeline.svg)](https://gitlab.com/bloom42/libs/crypto42/commits/dev)


`crypto42` is a cross-platform, secure, easy to use, and hard to misuse cryptographic library in Rust.
It uses [libsodium](https://github.com/jedisct1/libsodium) as backend.

Repository: https://gitlab.com/bloom42/libs/crypto42

The goal of `crypto42` is to keep it's API surface as minial as possible and to implement the less ciphers
as possible.

Only safe to use/implement ciphers are in the library.


## Primitives

- Authenticated Encryption with Associated Data (primitive: AEAD)
- Streaming Authenticated Encryption with Associated Data (primitive:
Streaming AEAD)
- One-way hash function (primitives: Hash).
- Key Derivation Functions (primitive: KDF)
<!-- - *deterministic* authenticated encryption with associated data (primitive: -->
<!-- Deterministic Aead) -->
<!-- - message authentication codes (primitive: MAC), -->
<!-- - Digital signatures (primitives: PublicKeySign and PublicKeyVerify) -->
<!-- - hybrid encryption (primitives: HybridEncrypt and HybridDecrypt). -->

| Primitive          | Algorithms                            |
| ------------------ | ----------------------------------------------- |
| AEAD               | XCHACHA20_POLY1305 |
| Streaming AEAD     | XCHACHA20_POLY1305 |
| Hash               | BLAKE2B, SHA2_256, SHA2_512, (SHA3_256, SHA3_512) |
| Key Derivation Functions | SCRYPT, ARGON2I |

<!-- | Digital Signatures | ED25519, EDDSA_BLAKE2B_ED25519 | -->
<!-- | Hybrid Encryption  | ECIES with AEAD and HKDF                        | -->
<!-- | Streaming AEAD     | AES-GCM-HKDF-STREAMING, AES-CTR-HMAC-STREAMING  | -->
<!-- | MAC                | HMAC-SHA2                                       | -->
<!-- | Deterministic AEAD | AES-SIV | -->


## Hash

```rust
hash::new()
fn update() | fn input() | fn write()
fn finalize() | fn digest((encoding)) | result() | final()

hash::hash(data);
```

## Streaming AEAD

```rust
fn init(_push)(key)
fn push(plaintext) | write(data)

fn init(_pull_)(key)
fn pull(ciphertext) | read()

```


## Libsodium

`crypto42` uses `libsodium` as backend.

`libsodium` documentation: https://libsodium.gitbook.io
