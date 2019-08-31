# Threat Model

See https://restic.readthedocs.io/en/latest/100_references.html#threat-model

## Authentication

Authentication MUST BE zer-knowledge. See [Auth](auth) for more details.

## End-to-end encryption

### Tresorit

https://tresorit.com/security/encryption

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
## On device encryption

Data stored on device are encrypted. Encyption keys are stored in the device secure KeyStore ([KeyStore](https://developer.android.com/training/articles/keystore) on Android, and [Keychain](https://developer.apple.com/documentation/security/keychain_services) on IOS), for whatever it's worth.


## Encryption in transit

Connection between clients and server and between servers and servers is encrypted using [INSERT PROTOCOL AND VERSION].
Please note that Bloom's threat model accept that this encryption in transit is insecure.


* https://github.com/nowsecure
* https://www.packtpub.com/eu/security/improving-your-penetration-testing-skills
* https://github.com/osquery/osquery/blob/master/README.md
* https://news.ycombinator.com/item?id=19681270


# Crypto

* http://engineering.mindlinksoft.com/end-to-end-encryption-with-the-signal-protocol/
* https://medium.com/@justinomora/demystifying-the-signal-protocol-for-end-to-end-encryption-e2ee-ad6a567e6cb4
* https://hackernoon.com/e2ee-app-backends-286cc94b8a7
* https://medium.com/codeclan/what-are-encryption-keys-and-how-do-they-work-cc48c3053bd6
* https://developers.connectycube.com/guides/end-to-end-encryption-otr
* https://blog.jabberhead.tk/2019/03/10/a-look-at-matrix-orgs-olm-megolm-encryption-protocol/
* https://news.ycombinator.com/item?id=18220996
* https://www.securemessagingapps.com/ratings/
* https://securechatguide.org/featuresmatrix.html
* https://dx.eng.uiowa.edu/dave/encodhertext.php
* https://www.keycdn.com/blog/perfect-forward-secrecy
* https://cacm.acm.org/magazines/2019/1/233523-imperfect-forward-secrecy/abstract
* https://signal.org/blog/asynchronous-security/
* https://signal.org/blog/advanced-ratcheting/
* https://users.ece.cmu.edu/~adrian/projects/sec/node6.html
* https://www.quora.com/What-is-exactly-backward-secrecy-property-in-cryptography-attribute-based-encryption
* https://matrix.org/blog/2016/11/21/matrixs-olm-end-to-end-encryption-security-assessment-released-and-implemented-cross-platform-on-riot-at-last
* https://security.stackexchange.com/questions/162773/are-matrix-messages-encrypted-using-perfect-forward-secrecy/172544
* https://security.stackexchange.com/questions/48644/forward-secrecy-for-kids
* https://matrix.org/blog/2015/12/25/the-matrix-holiday-special
* https://matrix.org/docs/guides/application-services
* https://en.wikipedia.org/wiki/Matrix_(protocol)
* https://matrix.org/docs/guides/moderation
* https://www.aviraldg.com/p/matrix-vector/
* https://crypto.stackexchange.com/questions/39761/definitions-of-secrecy
* https://whatis.techtarget.com/definition/perfect-forward-secrecy
* https://medium.com/mercuryprotocol/introducing-signal-protocol-to-dust-19b66c9331be
* https://medium.com/coinmonks/build-a-insurance-application-with-hyperledger-composer-and-react-js-part-1-3ebe7ad54986
* https://alexgaynor.net/2017/apr/26/forward-secrecy-is-the-most-important-thing/
* https://alexgaynor.net/2015/nov/28/5-critical-security-projects/
* https://alexgaynor.net/2019/jan/06/security-wish-list-2019/
* https://alexgaynor.net/2016/mar/14/anatomy-of-a-crypto-vulnerability/
* https://alexgaynor.net/2017/feb/26/sha1-and-richard-feynman/
* https://moxie.org/blog/we-should-all-have-something-to-hide/
* https://www.wired.com/2016/07/meet-moxie-marlinspike-anarchist-bringing-encryption-us/
* https://www.wired.com/2016/11/what-is-perfect-forward-secrecy/
* https://en.wikipedia.org/wiki/Forward_secrecy
* https://matrix.org/blog/2018/05/08/gdpr-compliance-in-matrix
* https://en.wikipedia.org/wiki/Double_Ratchet_Algorithm
* https://github.com/xwiki-labs/cryptpad/blob/master/docs/ARCHITECTURE.md
* https://security.stackexchange.com/questions/48644/forward-secrecy-for-kids
* https://en.wikipedia.org/wiki/Off-the-Record_Messaging
* https://conversations.im/omemo/
* https://news.ycombinator.com/item?id=12880769
* https://news.ycombinator.com/item?id=13239476
* https://www.uhoreg.ca/blog/20170910-2110
* https://www.snoyman.com/blog/2018/05/guide-to-matrix-riot
* https://brendan.abolivier.bzh/enter-the-matrix/
* https://xmpp.org/extensions/xep-0384.html
* https://blog.securegroup.com/omemo-end-to-end-encryption-for-chat-explained
* https://chatsecure.org/blog/chatsecure-v4-released/
* https://gitlab.matrix.org/matrix-org/olm/blob/master/docs/megolm.md
* https://matrix.org/docs/spec/
* https://infosec-handbook.eu/blog/limits-e2ee/
* https://github.com/matrix-org/matrix-doc/blob/hs/hash-identity/proposals/2134-identity-hash-lookup.md
* https://docs.google.com/document/d/1ni4LnC_vafX4h4K4sYNpmccS7QeHEFpAcYcbLS-J21Q/edit#heading=h.8qfidi5t91ba
* https://hacks.mozilla.org/2018/11/firefox-sync-privacy/
* https://mobile-security.gitbook.io/mobile-security-testing-guide/android-testing-guide/0x05e-testing-cryptography
* https://doridori.github.io/Android-Security-Beware-of-the-default-IV/?source=post_page---------------------------#sthash.jymTOnIq.ZpVHf4Be.dpbs
* https://blog.1password.com/guess-why-were-moving-to-256-bit-aes-keys/
* https://doridori.github.io/Android-Security-Beware-of-the-default-IV/#sthash.oCRshviA.fchGnCuF.dpbs
* https://medium.com/@tiensinodev/basic-android-encryption-dos-and-don-ts-7bc2cd3335ff
* https://doridori.github.io/android-security-the-forgetful-keystore/#sthash.QR9VjnpI.IpG0AHYu.dpbs
* https://github.com/drydart/flutter_sqlcipher
* https://www.unix-ninja.com/p/attacking_google_authenticator
* https://developer.android.com/reference/androidx/security/crypto/EncryptedSharedPreferences.html
* https://crypto.stackexchange.com/questions/18420/aes-gcm-disadvantage
* https://crypto.stackexchange.com/questions/26783/ciphertext-and-tag-size-and-iv-transmission-with-aes-in-gcm-mode
* https://libsodium.gitbook.io/doc/secret-key_cryptography/aead/aes-256-gcm
* https://proandroiddev.com/security-best-practices-symmetric-encryption-with-aes-in-java-and-android-part-2-b3b80e99ad36
* https://crypto.stackexchange.com/questions/10775/practical-disadvantages-of-gcm-mode-encryption/10808#10808
* https://crypto.stackexchange.com/questions/68032/deterministic-encryption-using-aes
* https://crypto.stackexchange.com/questions/17999/aes256-gcm-can-someone-explain-how-to-use-it-securely-ruby
* https://github.com/d0nutptr/Android-Security-Examples
* https://github.com/google/tink/blob/master/docs/PRIMITIVES.md
* https://www.raywenderlich.com/778533-encryption-tutorial-for-android-getting-started#
* https://news.ycombinator.com/item?id=16285135
* https://research.checkpoint.com/cryptographic-attacks-a-guide-for-the-perplexed/
