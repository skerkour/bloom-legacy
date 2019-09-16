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
* https://cybarrior.com/blog/2019/03/28/p4wnp1/
* https://docs.tcell.io/docs/data-collected
* https://news.ycombinator.com/item?id=19265377
* https://www.obdev.at/products/littlesnitch/index.html
* https://pastebin.com/0SNSvyjJ
* https://medium.com/tenable-techblog/owning-the-network-with-badusb-72daa45d1b00
* https://hackaday.com/2017/12/28/antenna-alignment-and-hunting-rogue-access-points-with-the-esp8266/
* https://0xpatrik.com/subdomain-takeover-candidates/
* https://github.com/rapid7/metasploit-framework/blob/master/modules/auxiliary/crawler/msfcrawler.rb
* https://wiki.mozilla.org/Security/Projects/Minion
* https://labs.detectify.com/2019/02/28/thinking-outside-of-the-password-manager-box/
* https://www.packtpub.com/networking-and-servers/mastering-linux-security-and-hardening
* https://coreruleset.org/
* http://appsensor.org/documentation.html
* https://labs.signalsciences.com/go-development-tools-for-testing-and-hot-reloading
* https://docs.signalsciences.net/install-guides/python-module/
* https://www.i-programmer.info/news/83-mobliephone/12440-eu-bug-bounty-software-security-as-a-civil-right.html
* https://medium.com/@karpov2007/pvs-studio-and-bug-bounties-on-free-and-open-source-software-538f42ee2701
* https://opaque.link/post/dropgang/
* https://www.contextis.com/en/blog/basic-electron-framework-exploitation
* https://news.ycombinator.com/item?id=20772416
* https://www.glasswire.com/
* https://www.youtube.com/watch?v=V29TkJOHlH4
* https://news.ycombinator.com/item?id=16743055
* https://www.opendns.com/phishing-quiz/
* https://www.sonicwall.com/phishing-iq-test-landing/
* https://www.owasp.org/index.php/Logging_Cheat_Sheet#Purpose
* https://deepscan.io/home/
* https://apipulse.tinfoilsecurity.com/
* https://duo.com/docs
* https://depfu.com/
* https://demo.goharbor.io/harbor/sign-in
* https://alternativeto.net/software/detectify/
* https://intruder.io/
* http://patrolserver.com/
* https://probely.com/
* https://duo.com/blog/part-1-usability-is-security
* https://medium.com/@jsoverson/the-lifespan-of-a-data-breach-fac810841b10
* https://www.knownsec.com/#/
* https://github.com/knownsec/wam
* https://github.com/topics/security-hardening
* https://github.com/topics/security-automation
* https://gofi.sh/index.html#intro
* https://gist.github.com/jivoi/724e4b4b22501b77ef133edc63eba7b4
* https://www.aquasec.com/
* https://github.com/lizheng3401?tab=repositories
* http://workinukraine.space/
* https://medium.com/@tom_35522/securing-your-practice-a1882261b571


## Static analysis

* https://www.sonarqube.org/features/quality-gate/
* https://www.sonarsource.com/products/codeanalyzers/sonarjs.html
* https://docs.sonarqube.org/latest/analysis/scan/sonarscanner/
* https://www.sonarlint.org/
* https://rules.sonarsource.com/go/RSPEC-1186
* https://www.codacy.com/product
* https://www.codefactor.io/features
* https://www.jetbrains.com/upsource/features/automatedcodereview.html
* https://docs.sonarqube.org/latest/setup/install-server/
* https://docs.sonarqube.org/latest/user-guide/concepts/
* https://github.com/SonarSource/sonar-scanner-api/blob/master/api/src/main/java/org/sonarsource/scanner/api/EmbeddedScanner.java
* https://github.com/SonarSource/sonar-go
* https://github.com/uartois/sonar-golang
* https://github.com/SonarSource/sonarts
* https://docs.sonarqube.org/latest/analysis/languages/javascript
* https://docs.coala.io/en/latest/Users/Tutorial.html
* https://github.com/ajinabraham/NodeJsScan
* https://help.sider.review/introduction#analysis-tools
* https://golangci.com/
* https://intercom.help/hound/en/collections/33942-configuration
* https://docs.codeclimate.com/docs/git-legal
* https://scrutinizer-ci.com/continuous-analysis
* https://github.com/alecthomas/gometalinter
* https://github.com/golangci/awesome-go-linters
* https://github.com/golangci
* https://github.com/markstory/lint-review
* https://livablesoftware.com/best-bots-software-development/
* http://groups.csail.mit.edu/pac/patchgen/
* https://github.com/probot/linter
* https://github.com/marketplace/better-code-hub
* https://imgbot.net/
* https://docs.coveralls.io/go
* https://github.com/topics/static-analysis
* https://en.wikipedia.org/wiki/Software_metric
* https://houndci.com/
* https://rust-lang.github.io/rust-clippy/master/
* https://styleci.io/


# Honypot

* https://jsonsecurity.blogspot.com/2017/01/dir-610-exploit-attack-seen-on-honeypot.html
* https://labs.mwrinfosecurity.com/blog/high-interaction-honeypots-with-sysdig-and-falco
* https://github.com/cowrie/cowrie
* https://www.robertputt.co.uk/basic-malware-analysis-with-cuckoo-sandbox.html
* https://www.robertputt.co.uk/learn-from-your-attackers-ssh-honeypot.html
* https://blog.varonis.fr/pourquoi-un-honeypot-nest-pas-une-solution-de-securite-complete/
* https://www.supinfo.com/articles/single/772-configuration-utilisation-honeypot-test-failles-avec-metasploit


## Cryptography

* https://blog.trailofbits.com/2019/09/11/crypto-2019-takeaways/
* http://loup-vaillant.fr/tutorials/chacha20-design
* https://www.cryptologie.net
* https://www.imperialviolet.org/posts-index.html
* https://support.1password.com/1password-security/
* https://support.1password.com/secret-key-security/
* http://loup-vaillant.fr/articles/implemented-my-own-crypto
* https://latacora.singles/2018/04/03/cryptographic-right-answers.html
* https://medium.com/vitelabs/the-selection-of-security-algorithm-of-vite-28180fd986c8
* https://bettercrypto.org
* https://www.reddit.com/r/crypto/comments/d4itki/is_eddsa_with_blake2b_as_hashing_function_secure/
* https://www.lawfareblog.com/snowden-revelations
