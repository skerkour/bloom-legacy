## On device encryption

Data stored on device are encrypted. Encyption keys are stored in the device secure KeyStore ([KeyStore](https://developer.android.com/training/articles/keystore) on Android, and [Keychain](https://developer.apple.com/documentation/security/keychain_services) on IOS), for whatever it's worth.


## End-to-end encryption

## OTR
## OMEMO

## Signal
https://signal.org/docs/
https://medium.com/@justinomora/demystifying-the-signal-protocol-for-end-to-end-encryption-e2ee-ad6a567e6cb4
https://www.reddit.com/r/signal/comments/a2ogk2/this_is_how_signal_protocol_encrypts_group/
https://signal.org/blog/private-groups/

## OLM/Megolm
https://gitlab.matrix.org/matrix-org/olm/blob/master/docs/megolm.md
https://blog.jabberhead.tk/2019/03/10/a-look-at-matrix-orgs-olm-megolm-encryption-protocol/
https://news.ycombinator.com/item?id=12880769

https://github.com/uhoreg/matrix-doc/blob/cross-signing2/proposals/1756-cross-signing.md
https://github.com/uhoreg/matrix-doc/blob/e2e_verification/proposals/1717-key_verification.md
https://matrix.org/docs/guides/end-to-end-encryption-implementation-guide

https://github.com/uhoreg/matrix-doc/blob/qr_key_verification/proposals/1543-qr_code_key_verification.md

## MLS

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


## WhatsApp

https://www.whatsapp.com/security/WhatsApp-Security-Whitepaper.pdf


## Other resources

https://medium.com/@wireapp
https://wiki.mozilla.org/Identity/CryptoIdeas/01-PBKDF-scrypt#PBKDF_User.2FAttacker_Costs
https://security.stackexchange.com/questions/126768/which-protocols-exist-for-end-to-end-encrypted-group-chat
https://hal.inria.fr/hal-01426845/document
https://www.cryptologie.net/article/447/whatsapp-secure-messaging-transcript-consistency-and-trust-in-a-group-chat/


## Encryption in transit

Connection to server and between servers is encrypted using [INSERT PROTOCOL AND VERSION].
