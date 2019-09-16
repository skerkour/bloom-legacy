# crypto42-rs overview

`crypto42-rs` is a cross-platform, secure, easy to use, and hard to misuse Rust cryptographic library.
It uses [libsodium](https://github.com/jedisct1/libsodium) as backend.

## Primitives

- Authenticated Encryption with Associated Data (primitive: AEAD)
- *streaming* authenticated encryption with associated data (primitive:
Streaming AEAD)
<!-- - *deterministic* authenticated encryption with associated data (primitive: -->
<!-- Deterministic Aead) -->
<!-- - message authentication codes (primitive: MAC), -->
- Digital signatures (primitives: PublicKeySign and PublicKeyVerify)
<!-- - hybrid encryption (primitives: HybridEncrypt and HybridDecrypt). -->
- One-way hash function (primitives: Hash).
- Key Derivation Functions (primitive: KDF)


| Primitive          | Implementations                            |
| ------------------ | ----------------------------------------------- |
| AEAD               | CHACHA20_POLY1305, CHACHA20_POLY1305_IETF, XCHACHA20_POLY1305_IETF, AES_256_GCM |
| Streaming AEAD     | XCHACHA20_POLY1305 |
<!-- | Digital Signatures | ED25519, EDDSA_BLAKE2B_ED25519 | -->
| Hash               | BLAKE2B, SHA2_256, SHA2_512, (SHA3_256, SHA3_512) |
| Key Derivation Functions | SCRYPT, ARGON2I |
<!-- | Hybrid Encryption  | ECIES with AEAD and HKDF                        | -->
<!-- | Streaming AEAD     | AES-GCM-HKDF-STREAMING, AES-CTR-HMAC-STREAMING  | -->
<!-- | MAC                | HMAC-SHA2                                       | -->
<!-- | Deterministic AEAD | AES-SIV | -->


## AEAD

```rust
fn encrypt() | fn seal()
fn decrypt() | fn open()

```

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


## Resources

* https://geti2p.net/spec/proposals/148-eddsa-blake2b-ed25519
* https://github.com/libra/libra/tree/master/crypto/crypto
* https://github.com/google/tink/blob/master/docs/PRIMITIVES.md
* https://github.com/google/tink/blob/master/docs/JAVA-HOWTO.md
