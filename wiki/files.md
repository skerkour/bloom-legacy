## Landscape

### Filesystem

* https://github.com/cloudflare/utahfs/blob/master/docs/functional-specification.md
* https://book.keybase.io/docs/files (https://book.keybase.io/docs/crypto/kbfs)


## Challenges

quelle metadata doivent rester en clair?
* filesize


comment faire du file deduplication?


Une nonce / key par block ou on derive en stream?



comment chiffrer des fichier plus grands que la memoire des devices?


comment on gere si un upload is interoompu en plein milieu ?


## Spec (work in progress)

```
File {
  ID
  Parent *fileID
  Nonce
  EncryptedKey
  Size
  EncryptedMetadata (name, mimetype,)
}

Blob {
  FileID
  Size
}
```
