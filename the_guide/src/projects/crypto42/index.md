# crypto42

[![pipeline status](https://gitlab.com/bloom42/libs/crypto42/badges/dev/pipeline.svg)](https://gitlab.com/bloom42/libs/crypto42/commits/dev)

`crypto42` is cross-platform, secure, easy to use, and hard to misuse cryptographic library in Rust,
using [libsodium](https://github.com/jedisct1/libsodium) as backend.

This document presents the high level design of the library, you can find detailed code documentation here: https://bloom42.gitlab.io/libs/crypto42/crypto42

Repository: https://gitlab.com/bloom42/libs/crypto42

The goal of `crypto42` is to keep it's API surface as minial as possible and to implement the less ciphers
as possible.

Only safe to use/implement ciphers are available in `crypto42`.


## Primitives

- Authenticated Encryption with Associated Data (primitive: AEAD)
<!-- - Streaming Authenticated Encryption with Associated Data (primitive: -->
<!-- Streaming AEAD) -->
- One-way hash functions (primitive: HASH).
- Key Derivation Functions (primitive: KDF)
<!-- - *deterministic* authenticated encryption with associated data (primitive: -->
<!-- Deterministic Aead) -->
<!-- - message authentication codes (primitive: MAC), -->
- Public-key signatures (primitive: SIGN)
<!-- - hybrid encryption (primitives: HybridEncrypt and HybridDecrypt). -->

| Primitive          | Algorithms                            |
| ------------------ | ----------------------------------------------- |
| AEAD               | XCHACHA20_POLY1305 |
| Hash               | BLAKE2B, SHA2_512, SHA2_256, (SHA3_512, SHA3_256) |
| Key Derivation Functions | ARGON2ID, ARGON2I, SCRYPT |
| Public-key signatures | ED25519 |

<!-- | Streaming AEAD     | XCHACHA20_POLY1305 | -->
<!-- | Hybrid Encryption  | ECIES with AEAD and HKDF                        | -->
<!-- | MAC                | HMAC-SHA2                                       | -->
<!-- | Deterministic AEAD | AES-SIV | -->



## Libsodium

`crypto42` uses `libsodium` as backend.

`libsodium` documentation: https://libsodium.gitbook.io


## Fuzzing

In the library's folder, use `make fuzz` to list fuzzing targets and then `cargo fuzz run [target]`
to run a specific target.
