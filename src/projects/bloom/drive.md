# Drive

## Overview

The drive service is a user friendly cloud storage/backup/file synching solution.

<!-- An user should be able to upload files, create folders to organise his files, download files, rename files or folders -->

## Scenarios

### 1
Sylvain wants to send a file from his computer to his phone. he uplaods the file from his computer and can download it later from his phone.


### 2
Marina wants to backup her holidays pictures. She create a folder, She Select all her photos on her computer and upload it to her drive.

### 3
After having uploaded some files, zinedine wants to search amongs them. He uses the search bar.

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

### Delta sync

compress, encrypt, upload


#### Resources

* https://help.backblaze.com/hc/en-us/articles/217665318-How-does-Backblaze-upload-modified-files-
* https://github.com/owncloud/client/wiki/DeltaSync
* https://owncloud.org/news/welcome-delta-sync-for-owncloud/
* https://github.com/keybase/client/tree/master/go/kbfs
* https://serverfault.com/questions/52861/how-does-dropbox-version-upload-large-files



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

-------------------



### initial folders
- Downloads
- Documents
- Music
- Pictures
- Books
- Videos
- Projects



### Resources

* [https://news.ycombinator.com/item?id=18750797](https://news.ycombinator.com/item?id=18750797)
* [http://lpan.io/one-liner-dropbox-client/](http://lpan.io/one-liner-dropbox-client/)
* [https://raymii.org/s/articles/Set_up_your_own_truly_secure_encrypted_shared_storage_aka_Dropbox_clone.html](https://raymii.org/s/articles/Set_up_your_own_truly_secure_encrypted_shared_storage_aka_Dropbox_clone.html)
* [https://docs.nextcloud.com/server/13/user_manual/files/desktop_mobile_sync.html](https://docs.nextcloud.com/server/13/user_manual/files/desktop_mobile_sync.html)
* [https://docs.syncthing.net/](https://docs.syncthing.net/)
* [https://github.com/haiwen/seafile](https://github.com/haiwen/seafile)
* [https://github.com/haiwen/seadrive-gui](https://github.com/haiwen/seadrive-gui)
* [https://github.com/haiwen/seafile-client](https://github.com/haiwen/seafile-client)
* [https://help.seafile.com/en/drive_client/using_drive_client.html](https://help.seafile.com/en/drive_client/using_drive_client.html)
* [https://blogs.seafile.com/2016/09/02/announcing-seafile-drive-client-a-new-way-to-map-seafile-storage-as-virtual-drive/](https://blogs.seafile.com/2016/09/02/announcing-seafile-drive-client-a-new-way-to-map-seafile-storage-as-virtual-drive/)
* [https://nihaocloud.zendesk.com/hc/en-us/articles/115003155753-How-to-Setup-and-use-Seafile-Drive-Client-on-Mac-Windows](https://nihaocloud.zendesk.com/hc/en-us/articles/115003155753-How-to-Setup-and-use-Seafile-Drive-Client-on-Mac-Windows)



<!-- ---------------




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
