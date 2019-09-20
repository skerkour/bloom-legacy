# Authentication

## Register

1. An

## Sign in

1. A 256 bits key `pw_key` is derived from `password` using the `argon2id` KDF and a random, per user `client_salt`
2. a 512 bits key `auth_key` is derived from `pwd_key` using `blake2b` from `crypto42::kdf`
3. `username` is sent with `auth_key` to server
4. server verify that `auth_key` match stored `hashed_auth_key` using `crypto42::kdf::argon2id::verify_password`
5. if ok, server generate a random UUIDv4 `session_id` and a 512 bits `session_token`
6. `session_token` is hashed using `crypto42::kdf::argon2id::hash_password` to product `session_token_hash`
7. both `session_id` and `session_token_hash` are stored in the Database
8. `base64(session_id+":"+session_token)` is sent back to client to be used as authitencation token.

![architecture](assets/bloom_auth_sign_in.jpg)


## Sign out


## Resources


### See also

### SRP

SRP, and SPAKE2 were discarded because too complexe (choice of primes...) and so too easy to screw up.

* https://docs.rs/srp/0.4.0/srp
* http://srp.stanford.edu/ndss.html
* https://protonmail.com/blog/encrypted_email_authentication/
* https://en.wikipedia.org/wiki/Secure_Remote_Password_protocol
* https://www.computest.nl/nl/knowledge-platform/blog/exploiting-two-buggy-srp-implementations/
* https://blog.cryptographyengineering.com/should-you-use-srp/
* https://blog.1password.com/developers-how-we-use-srp-and-you-can-too/
* https://medium.com/@intermediation/secure-remote-password-protocol-31ba8c2ab0b
* [ProtonMail whitepaper](/assets/resources/protonmail-whitepaper.pdf)
* https://docs.rs/spake2
## zero knowledge auth

* https://hacks.mozilla.org/2018/11/firefox-sync-privacy/
* https://crypto.stackexchange.com/questions/1145/how-much-would-it-cost-in-u-s-dollars-to-brute-force-a-256-bit-key-in-a-year/1160#1160
* https://blogs.dropbox.com/tech/2016/09/how-dropbox-securely-stores-your-passwords/
* https://medium.com/@harwoeck/password-and-credential-management-in-2018-56f43669d588
* https://crypto.stackexchange.com/questions/67261/does-gcm-or-ghash-only-provide-64-bit-security-against-forgeries/67367#67367
* http://srp.stanford.edu/
* https://www.cossacklabs.com/zero-knowledge-protocols-without-magic.html

### KDF
* https://medium.com/@mpreziuso/password-hashing-pbkdf2-scrypt-bcrypt-and-argon2-e25aaf41598e
* https://blog.filippo.io/the-scrypt-parameters/
