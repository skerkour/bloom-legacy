# Files

## Overview

The drive service is a user friendly cloud storage/backup/file synching solution.

<!-- An user should be able to upload files, create folders to organise his files, download files, rename files or folders -->

## Scenarios

### 1
Sylvain wants to send a file from his computer to his phone. he uplaods the file from his computer and can download it later from his phone.


### 2
Marina wants to backup her holidays pictures. She create a folder, She Select all her photos on her computer and upload it to her drive.

### 3
At first I thought "Well, Dropbox storage is 5x as expensive as hard drive space, so what's the point?". What I didn't see was the team use case: It's actually brilliant to have access to every document of all of my colleagues at all times, without saving all that stuff locally, or taking forever sync. This is really amazing.


## Goal

## Non goals

- file convertion
- copies
- apps binding
- drag and drop
- downlaod multiple files
- file edition or reading
- file sharing
- billing
- starred


- is it possible to have files with same name in the same folder ? no


## Competitive landscape

* DropBox (seems to use notify, https://www.dropboxforum.com/t5/Files-folders/Dropbox-does-not-automatically-sync-FUSE-directory/td-p/280902, https://help.dropbox.com/installs-integrations/sync-uploads/smart-sync)
* OneDrive
* GoogleDrive
* SyncThings (use periodic scans, with optional notify)
* rclone, FUSE avec mount (https://rclone.org/commands/rclone_mount/) et inotify
* gocryptfs / cryptomator (https://nuetzlich.net/gocryptfs/comparison/)
* Tresorit: utulise selective sync (https://support.tresorit.com/hc/en-us/articles/216114247-Using-selective-sync) et FUSE (https://support.tresorit.com/hc/en-us/articles/219312887-Set-up-Tresorit-Drive)
* https://librevault.com/
* https://github.com/haiwen/seafile: les 2, inotify, et FUSE avec DRIVE (https://help.seafile.com/en/sharing_collaboration/sharing_files_and_folders.html)
* rsync (https://en.wikipedia.org/wiki/Rsync, https://rsync.samba.org/, https://www.quora.com/How-does-the-rsync-algorithm-work) and zsync (http://zsync.moria.org.uk/)
* OwnCloud
* NextCloud

<!-- ## Analyst landscape
## Top Customer Success/Sales issue(s)
## Top user issue(s)
## Top internal customer issue(s)
## Top Vision Item(s) -->

## Sync

Files synchronization is in reality 2 challenges:
1. detect files changes
2. upload/download these changes


### Changes detection

There is 2 ways to detect files changes

1. Using a Filesystem in USErspace (FUSE)
2. Using (i)notify

Using FUSE requires kernel extensions on non Linux platforms and thus is not a good option regarding security
when devlopment resources are limited. This is

#### Notify

Notify is the first method used by Bloom drive as it's the safest.

##### +

* does not requires a kernel extension
* easier to predict the local place occupied
* easier to predict what will be available offline (only the check folders).

##### -

* requires a little bit more effort as we need to check which files we want to sync


##### Resources

* https://stackoverflow.com/questions/35711897/dropbox-fs-inotify-error
* https://news.ycombinator.com/item?id=18750797 (My one-liner Linux Dropbox client)
* http://lpan.io/one-liner-dropbox-client/
* https://help.seafile.com/en/syncing_client/install_sync.html
* https://support.tresorit.com/hc/en-us/articles/216114247-Using-selective-sync
* https://help.nextcloud.com/t/to-be-clear-no-way-to-have-2way-sync-with-android-client/35027/2


#### FUSE

FUSE is not implemented yet but may be in the future, under the name **Smart Sync**.

##### +

* easier to use: allow to list all files/folders and local cache only tose which are used

##### -

* requires a kernel extension on non linux platofrms
* local cache should be smart. (or we allow user to sync only what he selects)

##### Resources

* https://help.dropbox.com/installs-integrations/sync-uploads/smart-sync
* https://github.com/osxfuse/osxfuse/wiki/FAQ
* https://news.ycombinator.com/item?id=11037257 (Introducing the Keybase filesystem)
* https://keybase.io/docs/crypto/kbfs
* https://rclone.org/commands/rclone_mount/
* https://www.dropboxforum.com/t5/Dropbox/When-will-Smart-Sync-work-on-linux/idi-p/262171
* https://github.com/pydio
* https://news.ycombinator.com/item?id=11570888 (Dropbox Project Infinite)
* https://support.tresorit.com/hc/en-us/articles/219312887-Set-up-Tresorit-Drive
* https://nihaocloud.zendesk.com/hc/en-us/articles/115003155753-How-to-Setup-and-use-Seafile-Drive-Client-on-Mac-Windows
* https://blogs.seafile.com/2016/09/02/announcing-seafile-drive-client-a-new-way-to-map-seafile-storage-as-virtual-drive/
* https://help.seafile.com/en/drive_client/using_drive_client.html


### Delta sync

compress, encrypt, upload

#### Blocks

Files are divided into blocks (or *chunks*).
The blocks are currently fixed size (1MB? 512kB?, dropbox uses 4MB) except the last one in the file which may be smaller.
Each file is sliced into a number of these blocks, and the `blake2b` hash of each block is computed
This results in a block list containing the offset, size and hash of all blocks in the file.

Each block is compressed using (??) then encrypted using a per (block/file) key, using the `XChachaPoly1305` AEAD cipher.

To update a file, Bloom Drive compares the block list of the current version of the file to the block
list of the desired version of the file. It then tries to find a source for each block that differs.
This might be locally, if another file already has a block with the same hash, or it may be from
another device in local network, or from the server. In the first case the block is simply copied
on disk, in the second case it is requested over the network.

When a block is copied or received from another source, its `blake2b` hash is computed and compared
with the expected value. If it matches the block is written to a temporary copy of the file, otherwise
it is discarded and Bloom Drive tries to find another source for the block.

#### Temporary Files

Bloom Drive never writes directly to a destination file. Instead all changes are made to a temporary
copy which is then moved in place over the old version. If an error occurs during the copying or syncing,
such as a necessary block not being available, the temporary file is kept around for up to a day.
This is to avoid needlessly requesting data over the network.

The temporary files are named `.drive.original_filename.ext.tmp` or, on Windows, `~drive~original_filename.ext.tmp`
where `original_filename.ext` is the destination filename. The temporary file is normally hidden.
If the temporary file name would be too long due to the addition of the prefix and extra extension,
a hash of the original file name is used instead of the actual original file name.

#### Conflicts

#### File metadata

```json
{
  "path": "",
  "name": "",
  "hash_blake2b": "",
  "blocks": [
    {
      "id": "",
      "comrpession_algo": "",
      "offset": 0,
      "hash_blake2b": "",
      "size": 0
    }
  ]
}
```

#### Resources

* https://help.backblaze.com/hc/en-us/articles/217665318-How-does-Backblaze-upload-modified-files-
* https://github.com/owncloud/client/wiki/DeltaSync
* https://owncloud.org/news/welcome-delta-sync-for-owncloud/
* https://github.com/keybase/client/tree/master/go/kbfs
* https://serverfault.com/questions/52861/how-does-dropbox-version-upload-large-files
* https://github.com/owncloud/client/issues/179
* https://docs.syncthing.net/users/syncing.html

### File explorer integration

File synchronization is one thing, but you also need to provide a way for the user to see if files
are currently synching, or synched whene navigating in his filesystem.

Depending of the Operating system, there is the possibility to provide a 'file explorer extension'
which allow to add icons to folders/files when they are in a particular state (synched, synching...).

#### Resources

* https://nihaocloud.zendesk.com/hc/en-us/articles/115003155753-How-to-Setup-and-use-Seafile-Drive-Client-on-Mac-Windows
* https://developer.apple.com/library/archive/documentation/General/Conceptual/ExtensibilityPG/Finder.html
* https://docs.nextcloud.com/desktop/2.3/navigating.html



## Models
`file`

`drive_profile`
- space -> used space
- home -> home directory (file)



## Views

### Sidebar
- + new button (or fab on mobile) -> Folder | File upload, Folder uplaod
- trash

### secondary toolbar
- delete
- move to
- Download

### new
- Folder
separator
- Files upload
- Folder upload

### Home
path: `/drive`

### Folder
path: `/drive/folders/{folder_id}`

### Trash
path: `/drive/trash`




## Resources

* https://www.networkworld.com/article/3142390/down-the-rabbit-hole-part-6-secure-and-private-online-file-storage.html
* https://www.linux.com/tutorials/5-linux-gui-cloud-backup-tools/
* https://filebrowser.xyz
* https://wiki.archlinux.org/index.php/Dropbox
* https://stackoverflow.com/questions/1960799/using-git-and-dropbox-together-effectively?rq=1
* https://news.ycombinator.com/item?id=12463338 (How Dropbox Hacks Your Mac)
* https://news.ycombinator.com/item?id=12619722 (Dropbox Hasn't Learned Their Lesson)
* https://news.ycombinator.com/item?id=18750797 (My one-liner Linux Dropbox client)
* https://raymii.org/s/articles/Set_up_your_own_truly_secure_encrypted_shared_storage_aka_Dropbox_clone.html#toc_1
* https://www.dropboxforum.com/t5/Files-folders/Can-you-explain-the-difference-between-Smart-Sync-and-Selective/td-p/319218
* https://gladinet.com/CloudEnterprise/SelfHostedDropbox.aspx
* https://github.com/syncany/syncany
* https://news.ycombinator.com/item?id=20163389 (The new Dropbox)
* https://librevault.com/
* https://www.pcloud.com/download-free-online-cloud-file-storage.html
* https://github.com/haiwen/seafile
* https://blogs.dropbox.com/tech/2016/05/inside-the-magic-pocket/


## Resources crypto

* https://security.stackexchange.com/questions/150837/keep-encrypted-files-in-sync-on-a-cloud-service-without-having-to-upload-an-ent
* https://www.boxcryptor.com/en/blog/post/google-backup-and-sync-and-boxcryptor/
* https://rclone.org/crypt/
* https://forum.seafile.com/t/how-strong-is-the-encryption/6275
* https://help.seafile.com/en/security_and_encryption/use_encrypted_libraries.html
* https://www.seafile.com/en/features/
* https://www.sync.com/your-privacy/
* https://www.boxcryptor.com/en/blog/post/end-to-end-encryption-e2ee-for-dropbox-google-drive-and-co/
* https://nextcloud.com/endtoend/
* https://github.com/nextcloud/end_to_end_encryption_rfc/blob/master/RFC.md
* https://support.1password.com/sync-options-security/
* https://docs.cryptomator.org/en/1.4/desktop/access-vault/
* https://help.resilio.com/hc/en-us/articles/207370466-Encrypted-folders
* https://www.resilio.com/blog/understanding-the-sync-encrypted-folder


Soit un.index geant, mais du coup on doit le paycher comme  les contacts etc....

Soit atborescence en.db, avec parent id

<!--

### initial folders
- Downloads
- Documents
- Music
- Pictures
- Books
- Videos
- Projects

## Drive

- quelque part ou se passent les actions cote serveur (upload s3, conversion...)
- download/move multiple files / folders (zip)
- supression des subfiles
- path au dessus de la liste des fichiers
- spotlight like search
- limiter l'acces aux dossiers /  application



barre de recherche

on peut selectionner plusierus fichiers, pour ensuite:
- les supprimer
- les deplacer
- en faire des copies (ajoute le suffixe "Copy of")
- telecharger
- convertion

renommer un fichier individuel

il faut pouvoir creer:
- upload de fichiers
- upload de dossier
- creer un dossier


starrer un fichier ou un dossier
Echange directe entre ordinateurs sur le reseau

v1:
sous domaines
v2:
tout sous le meme domaine ?

subdomain:
pour personnaliser les apps avec des domains personnalises pour les entreprises.


pourquoi on ne peut pas tout meme sous le meme sous domain des la v1 ?
- temps de chargement

pourquoi tout mettre dans la meme app des le debut ?
- pas de code / repetition de code
- plus besoin de cookies -> secu ++

sinon on peut commencer separe, mais avec les URLS d'une monoapp


partage:
- collaboraterus
ou public


Pouvoir convertir des fichiers (audio, video)


3 etats:
- local mais pas cloud
- cloud mais pas local
- synchronized -->
