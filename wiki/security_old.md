* https://github.com/auditdrivencrypto/private-box
* https://matrix.org/docs/spec/client_server/r0.6.0#syncing
* https://github.com/TokTok/c-toxcore/issues/426
* https://crypto.stackexchange.com/questions/68925/how-to-understand-authenticated-key-exchange
* https://crypto.stackexchange.com/questions/53670/is-libsodiums-box-construct-secure-without-additional-manual-padding-of-the-p
* https://crypto.stackexchange.com/questions/33904/authenticated-encryption-with-signatures
* https://crypto.stackexchange.com/questions/52912/how-safe-are-libsodium-crypto-boxes
* https://crypto.stackexchange.com/questions/60609/what-is-the-difference-between-a-sealed-box-and-a-normal-box-in-libsodium
* https://www.reddit.com/r/crypto/comments/fx7sx0/ed25519_or_curve25519_for_long_term_identity/
* https://ristretto.group/ristretto.html
* https://moderncrypto.org/mail-archive/curves/2014/000293.html
* https://crypto.stackexchange.com/questions/47147/ed25519-is-a-signature-or-just-elliptic-curve
* https://crypto.stackexchange.com/questions/27866/why-curve25519-for-encryption-but-ed25519-for-signatures
* https://ianix.com/pub/curve25519-deployment.html
* https://ianix.com/pub/chacha-deployment.html
* https://ianix.com/pub/ed25519-deployment.html
* https://crypto.stackexchange.com/questions/72148/signal-protocol-how-is-signed-prekey-created
* https://curves.moderncrypto.narkive.com/BMafwQ5i/xeddsa-specification
* https://crypto.stackexchange.com/questions/48212/securely-re-using-a-shared-key-with-nacl
* https://crypto.stackexchange.com/questions/16276/perfect-forward-secrecy-with-nacl
* https://crypto.stackexchange.com/questions/22850/why-does-nacl-have-different-keys-for-signing-and-encryption
* https://crypto.stackexchange.com/questions/27866/why-curve25519-for-encryption-but-ed25519-for-signatures
* https://github.com/FiloSottile/age
* https://github.com/FiloSottile/age/blob/f0f8092d60bb96737fa096c29ec6d8adb5810390/internal/age/ssh.go#L174
* https://godoc.org/golang.org/x/crypto/nacl/secretbox
* https://blog.filippo.io/using-ed25519-keys-for-encryption/
* https://blog.filippo.io/we-need-to-talk-about-session-tickets/
* https://security.stackexchange.com/questions/50878/ecdsa-vs-ecdh-vs-ed25519-vs-curve25519
* https://crypto.stackexchange.com/questions/62879/verifying-eddsa-signatures-using-xeddsa-verify-function/62881#62881
* https://crypto.stackexchange.com/questions/27866/why-curve25519-for-encryption-but-ed25519-for-signatures
* https://crypto.stackexchange.com/questions/68121/curve25519-over-ed25519-for-key-exchange-why
* https://jmap.io/spec-core.html
* https://github.com/secure-io/sio-go
* https://branca.io/
* https://godoc.org/golang.org/x/crypto/nacl/secretbox
* https://blog.alltherunning.com/2018/02/12/pass-keybase-multi-gpg.html
* https://blog.filippo.io/mkcert-valid-https-certificates-for-localhost/
* https://godoc.org/golang.org/x/crypto/nacl/box
* https://crypto.stackexchange.com/questions/48475/why-are-ephemeral-session-temporary-keys-useful

go:
* https://gist.github.com/jmoiron/6979540
* https://gist.github.com/alex-ant/aeaaf497055590dacba760af24839b8d


addresses format:
* https://en.bitcoin.it/wiki/Bech32
* https://en.bitcoin.it/wiki/Bech32_adoption
* https://en.bitcoin.it/wiki/BIP_0173
* https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki
* https://bitcoin.stackexchange.com/questions/92251/convert-bech32-bitcoin-address-to-legacy


local db:
* https://github.com/zero-os/0-stor
* https://github.com/zippoxer/bow
* https://github.com/timshannon/badgerhold
* https://github.com/asdine/storm
* https://github.com/asdine/genji
* https://github.com/dgraph-io/badger#benchmarks



drive:
* https://www.tarsnap.com/design.html
* https://news.ycombinator.com/item?id=22451690


* https://github.com/gopasspw/gopass
* https://www.openbsd.org/papers/bsdcan-signify.html
----------------------------------------------------------------------------------------------------


authentification: signer la auth_key avec la cle publique de l'utilisateur



replay attacks

https://en.wikipedia.org/wiki/Replay_attack#Prevention_and_countermeasures
https://www.kaspersky.com/resource-center/definitions/replay-attack
https://www.binance.vision/security/what-is-a-replay-attack



See also:
https://outercorner.com/secrets/security/#mainkey_file
https://duo.com/labs/tech-notes/noise-protocol-framework-intro
https://carmagnole.ovh/articles/ssh-gpg-elliptique-ed25519/
https://docs.inkdrop.app/security
https://restic.readthedocs.io/en/latest/100_references.html#design
https://kinsta.com/blog/tls-1-3/#
https://blog.cloudflare.com/why-tls-1-3-isnt-in-browsers-yet/
https://blog.cloudflare.com/rfc-8446-aka-tls-1-3/
https://signal.org/docs/specifications/x3dh/
https://medium.com/asecuritysite-when-bob-met-alice/forward-secrecy-and-ephemeral-keys-guarding-against-data-breaches-in-the-future-b709295c6e5a
https://tls.mbed.org/kb/cryptography/ephemeral-diffie-hellman
https://blog.jabberhead.tk/2019/03/10/a-look-at-matrix-orgs-olm-megolm-encryption-protocol/
https://gitlab.matrix.org/matrix-org/olm/-/blob/master/docs/megolm.md
https://crypto.stackexchange.com/questions/61950/how-does-future-secrecy-fail-for-signal-group-messaging
https://crypto.stackexchange.com/questions/68795/what-attacks-does-future-secrecy-protect-against
https://crypto.stackexchange.com/questions/80138/using-encryption-for-authenticating-messages-instead-of-signing
http://www.noiseprotocol.org/noise.html
https://signal.org/blog/asynchronous-security/
https://security.stackexchange.com/questions/179273/what-is-the-purpose-of-associated-authenticated-data-in-aead
https://www.cloudinsidr.com/content/tls-1-3-and-tls-1-2-cipher-suites-demystified-how-to-pick-your-ciphers-wisely/
https://www.thesslstore.com/blog/cipher-suites-algorithms-security-settings/
https://crypto.stackexchange.com/questions/5458/should-we-sign-then-encrypt-or-encrypt-then-sign
https://github.com/awnumar/memguard
https://crypto.stackexchange.com/questions/68121/curve25519-over-ed25519-for-key-exchange-why
https://github.com/miguelsandro/curve25519-go
https://github.com/golang/go/issues/32670
https://github.com/ProtonMail/gopenpgp
https://github.com/ProtonMail/gopenpgp/blob/master/crypto/signature.go
https://github.com/ProtonMail/gopenpgp/tree/master/crypto
https://github.com/ProtonMail/gopenpgp/tree/master/armor
https://godoc.org/gopkg.in/ProtonMail/gopenpgp.v2/crypto
https://users.ece.cmu.edu/~adrian/630-f04/PGP-intro.html
https://security.stackexchange.com/questions/50878/ecdsa-vs-ecdh-vs-ed25519-vs-curve25519
https://www.cloudinsidr.com/content/tls-1-3-and-tls-1-2-cipher-suites-demystified-how-to-pick-your-ciphers-wisely/
https://blog.cryptographyengineering.com/2011/12/04/matt-green-smackdown-watch-are-aead/
https://adecentralizedworld.com/2020/03/studytime-a-decentralized-mooc-platform/
* https://signal.org/blog/signal-private-group-system/


drive
https://github.com/odeke-em/drive
https://github.com/hbons/SparkleShare
https://github.com/gaubert/gmvault
https://github.com/Librevault/librevault
https://github.com/serkanyersen/sync
https://github.com/stefankueng/CryptSync
https://github.com/ForstaLabs/libsignal-node
https://github.com/cryptomator/cryptomator
https://github.com/PrivateBin/PrivateBin
https://github.com/keybase/kbfs
https://github.com/topics/synchronization
https://github.com/syncthing/syncthing
https://github.com/syncthing/syncthing-android
https://github.com/librsync/librsync
https://github.com/Redundancy/go-sync
https://datprotocol.github.io/how-dat-works/
* https://restic.readthedocs.io/en/v0.7.3/references.html
* https://blog.filippo.io/restic-cryptography/
* https://docs.syncthing.net/specs/relay-v1.html
* https://docs.syncthing.net/specs/bep-v1.html
* https://github.com/c4milo/gsync
* https://github.com/Redundancy/go-sync
* https://dropbox.tech/infrastructure/rewriting-the-heart-of-our-sync-engine
* https://rclone.org/overview/
* https://rclone.org/faq/

notes:
https://blog.inkdrop.info/introducing-inkdrop-4-9d0c63de16d2
https://news.ycombinator.com/item?id=20103589
https://github.com/dnote/dnote
https://www.getdnote.com/
https://www.getdnote.com/blog/back-to-basic-command-line-notebook/
https://www.getdnote.com/blog/back-to-knowledge-base/
https://www.getdnote.com/blog/how-i-built-personal-knowledge-base-for-myself/
https://github.com/cabalamat/catwiki
https://blog.inkdrop.info/how-i-built-a-markdown-editor-earning-1300-mo-profit-inkdrop-ddf6ad702c42


----------------------------------------------------------------------------------------------------




## Chat

### OTR
### OMEMO

### Signal
https://signal.org/docs/
https://medium.com/@justinomora/demystifying-the-signal-protocol-for-end-to-end-encryption-e2ee-ad6a567e6cb4
https://www.reddit.com/r/signal/comments/a2ogk2/this_is_how_signal_protocol_encrypts_group/
https://signal.org/blog/private-groups/

### OLM/Megolm
https://gitlab.matrix.org/matrix-org/olm/blob/master/docs/megolm.md
https://blog.jabberhead.tk/2019/03/10/a-look-at-matrix-orgs-olm-megolm-encryption-protocol/
https://news.ycombinator.com/item?id=12880769

https://github.com/uhoreg/matrix-doc/blob/cross-signing2/proposals/1756-cross-signing.md
https://github.com/uhoreg/matrix-doc/blob/e2e_verification/proposals/1717-key_verification.md
https://matrix.org/docs/guides/end-to-end-encryption-implementation-guide

https://github.com/uhoreg/matrix-doc/blob/qr_key_verification/proposals/1543-qr_code_key_verification.md

### MLS

According to
https://blog.trailofbits.com/2019/08/06/better-encrypted-group-chat/
and https://blog.jabberhead.tk/2019/03/10/a-look-at-matrix-orgs-olm-megolm-encryption-protocol/
the future of secure messaging is MLS:

By using trees, it allows better scalability of group conversations

* https://github.com/mlswg
* https://messaginglayersecurity.rocks/
* https://www.blackhat.com/us-19/briefings/schedule/index.html#messaging-layer-security-towards-a-new-era-of-secure-group-messaging-16230
* https://datatracker.ietf.org/wg/mls/about/
* https://tools.ietf.org/html/draft-ietf-mls-protocol-07
* https://github.com/wireapp/melissa


### WhatsApp

* https://www.whatsapp.com/security/WhatsApp-Security-Whitepaper.pdf


## Other resources

* https://medium.com/@wireapp
* https://wiki.mozilla.org/Identity/CryptoIdeas/01-PBKDF-scrypt#PBKDF_User.2FAttacker_Costs
* https://security.stackexchange.com/questions/126768/which-protocols-exist-for-end-to-end-encrypted-group-chat
* https://hal.inria.fr/hal-01426845/document
* https://www.cryptologie.net/article/447/whatsapp-secure-messaging-transcript-consistency-and-trust-in-a-group-chat/
* https://ianix.com/pub/curve25519-deployment.html

### Other questions

Comment on gère le salt pour le kdf depuis le password ?

Le truc c’est que le salt doit être accessible pour tous les autres clients et idéalement non envoyé au serveur

* Utiliser l’username -> mais alors comment on fait pour le changer, et le problème c’est que dans le flow actuel, le username n’est pas encore choisi, et comment on fait le padding ?


* Utiliser un random genre cote client -> mais comment on le sync avec les autres clients ?
* Le signing in se fait en 2 step, d’abord le client demande le client salt, et ensuite il renvoie la auth_key


* Const salt -> quid de la securite ???




### See also

#### SRP

SRP, and SPAKE2 were discarded because too complexe (choice of primes...) and so too easy to screw up.

* https://docs.rs/srp/0.4.0/srp
* http://srp.stanford.edu/ndss.html
* https://protonmail.com/blog/encrypted_email_authentication/
* https://en.wikipedia.org/wiki/Secure_Remote_Password_protocol
* https://www.computest.nl/nl/knowledge-platform/blog/exploiting-two-buggy-srp-implementations/
* https://blog.cryptographyengineering.com/should-you-use-srp/
* https://blog.1password.com/developers-how-we-use-srp-and-you-can-too/
* https://medium.com/@intermediation/secure-remote-password-protocol-31ba8c2ab0b
* [ProtonMail whitepaper](https://protonmail.com/docs/business-whitepaper.pdf)
* https://docs.rs/spake2
* https://www.cossacklabs.com/zero-knowledge-protocols-without-magic.html



## Cryptography

* https://blog.trailofbits.com/2019/09/11/crypto-2019-takeaways/
* http://loup-vaillant.fr/tutorials/chacha20-design
* https://monocypher.org/manual
* https://www.cryptologie.net
* https://www.imperialviolet.org/posts-index.html
* https://support.1password.com/1password-security/
* https://support.1password.com/secret-key-security/
* http://loup-vaillant.fr/articles/implemented-my-own-crypto
* https://latacora.singles/2018/04/03/cryptographic-right-answers.html (https://news.ycombinator.com/item?id=16748400)
* https://medium.com/vitelabs/the-selection-of-security-algorithm-of-vite-28180fd986c8
* https://bettercrypto.org
* https://www.reddit.com/r/crypto/comments/d4itki/is_eddsa_with_blake2b_as_hashing_function_secure/
* https://www.lawfareblog.com/snowden-revelations
* https://github.com/wireapp/melissa
* https://github.com/trailofbits/molasses
* https://tools.ietf.org/html/draft-irtf-cfrg-xchacha-01
* https://tools.ietf.org/html/rfc8439
* https://geti2p.net/spec/proposals/148-eddsa-blake2b-ed25519
* https://github.com/libra/libra/tree/master/crypto/crypto
* https://github.com/google/tink/blob/master/docs/PRIMITIVES.md
* https://github.com/google/tink/blob/master/docs/JAVA-HOWTO.md
* https://news.ycombinator.com/item?id=13364424 (the design of ChaCha20)
* https://www.cossacklabs.com/blog/end-to-end-encryption-in-bear-app.html
* https://docs.rs/rust_sodium/0.10.2/rust_sodium/
* https://docs.rs/sodiumoxide/0.2.4/sodiumoxide/
* https://docs.rs/safe_crypto/0.8.0/safe_crypto/
* https://docs.nano.org/protocol-design/signing-hashing-and-key-derivation/
* https://crypto.stackexchange.com/questions/48136/blake2b-as-a-kdf
