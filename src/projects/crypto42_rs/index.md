# crypto42-rs overview

`crypto42-rs` uses https://github.com/jedisct1/libsodium as backend.

## Primitives

- Authenticated Encryption with Associated Data (primitive: AEAD)
<!-- - *streaming* authenticated encryption with associated data (primitive: -->
<!-- Streaming Aead) -->
<!-- - *deterministic* authenticated encryption with associated data (primitive: -->
<!-- Deterministic Aead) -->
<!-- - message authentication codes (primitive: MAC), -->
- Digital signatures (primitives: PublicKeySign and PublicKeyVerify)
<!-- - hybrid encryption (primitives: HybridEncrypt and HybridDecrypt). -->
- One-way hash function (primitives: Hash).
- Key Derivation Functions (primitive: KDF)


| Primitive          | Implementations                            |
| ------------------ | ----------------------------------------------- |
| AEAD               | CHACHA20_POLY1305, CHACHA20_POLY1305_IETF, (AES_256_GCM) |
<!-- | Digital Signatures | ED25519, EDDSA_BLAKE2B_ED25519 | -->
| Hash               | BLAKE2B, (SHA2_256, SHA2_512, SHA3_256, SHA3_512) |
| Key Derivation Functions | SCRYPT, ARGON2I |
<!-- | Hybrid Encryption  | ECIES with AEAD and HKDF                        | -->
<!-- | Streaming AEAD     | AES-GCM-HKDF-STREAMING, AES-CTR-HMAC-STREAMING  | -->
<!-- | MAC                | HMAC-SHA2                                       | -->
<!-- | Deterministic AEAD | AES-SIV                                         | -->


## Resources

* https://geti2p.net/spec/proposals/148-eddsa-blake2b-ed25519
* https://medium.com/vitelabs/the-selection-of-security-algorithm-of-vite-28180fd986c8
* https://github.com/libra/libra/tree/master/crypto/crypto
