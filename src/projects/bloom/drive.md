# Drive

1. [Overviw](#overviw)
2. [Scenarios](#scenarios)

-------------------

## Overview

The drive service is a user friendly cloud storage solution.

An user should be able to upload files, create folders to organise his files, download files, rename files or folders


## Scenarios

### 1
Sylvain wants to send a file from his computer to his phone. he uplaods the file from his computer and can download it later from his phone.


### 2
Marina wants to backup her holidays pictures. She create a folder, She Select all her photos on her computer and upload it to her drive.

### 3
After having uploaded some files, zinedine wants to search amongs them. He uses the search bar.

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


-is it possible to have files with same name in the same folder ? no

## Models
`file`

`drive_profile`
- space -> used space
- home -> home directory (file)

### initial folders
- Downloads
- Documents
- Music
- Pictures
- Books
- Videos
- Projects



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



---------------


# Drive

fuse ?

+:
+ facile d'utilisation: permet de lister tous les fichiers, et ensuite on cache que ceux qui sont necessaires


-:
* necessite une extension kernel sur
* gestion du cache doit se faire intelligement, ou alors on permet a l'utilisateur de choisir ce qu'il veut sync, et les autres ont les cache au besoin

notify ?

+:
* pas besoin de kernel extension -> ++++ safe
* plus previsible pour savoir la place que ca va prendre, et donc ce qui sera accessible hors ligne
-:
* un peut plsu dur de choisir quels fichiers on sync



le mieux est peut etre de faire le plus safe, avec la possibilite d'activer une option "SmartDrive"

directement dans l'app:


https://developer.apple.com/library/archive/documentation/General/Conceptual/ExtensibilityPG/Finder.html
https://github.com/osxfuse/osxfuse/wiki/Mount-options
https://github.com/osxfuse/osxfuse/wiki/FAQ
https://news.ycombinator.com/item?id=11037257
https://keybase.io/docs/crypto/kbfs

* DropBox (seems to use notify, https://www.dropboxforum.com/t5/Files-folders/Dropbox-does-not-automatically-sync-FUSE-directory/td-p/280902, https://help.dropbox.com/installs-integrations/sync-uploads/smart-sync)
* OneDrive
* GoogleDrive
* SyncThings (use periodic scans, with optional notify)
* rclone, FUSE avec mount (https://rclone.org/commands/rclone_mount/) et inotify
* gocryptfs / cryptomator (https://nuetzlich.net/gocryptfs/comparison/)
* KeyBase: FUSE (https://github.com/keybase/client/tree/master/go/kbfs)
* Tresorit: utulise selective sync (https://support.tresorit.com/hc/en-us/articles/216114247-Using-selective-sync) et FUSE (https://support.tresorit.com/hc/en-us/articles/219312887-Set-up-Tresorit-Drive)
* https://librevault.com/
* https://github.com/haiwen/seafile: les 2, inotify, et FUSE avec DRIVE (https://help.seafile.com/en/sharing_collaboration/sharing_files_and_folders.html)
* rsync (https://en.wikipedia.org/wiki/Rsync, https://rsync.samba.org/, https://www.quora.com/How-does-the-rsync-algorithm-work) and zsync (http://zsync.moria.org.uk/)
* OwnCloud
* NextCloud


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



## Resources

* https://stackoverflow.com/questions/35711897/dropbox-fs-inotify-error
* https://stackoverflow.com/questions/1960799/using-git-and-dropbox-together-effectively?rq=1
* https://www.dropboxforum.com/t5/Dropbox/When-will-Smart-Sync-work-on-linux/idi-p/262171
* https://rclone.org/commands/rclone_mount/#file-caching
* https://www.smartsync.com/
* https://help.dropbox.com/installs-integrations/sync-uploads/smart-sync
* https://www.dropboxforum.com/t5/Files-folders/Can-you-explain-the-difference-between-Smart-Sync-and-Selective/td-p/319218
* https://news.ycombinator.com/item?id=16743055
* https://support.tresorit.com/hc/en-us/articles/216114247-Using-selective-sync
* https://support.tresorit.com/hc/en-us/articles/219312887-Set-up-Tresorit-Drive
* https://support.tresorit.com/hc/en-us/articles/216114367-How-is-my-password-managed-in-Tresorit-
* https://support.tresorit.com/hc/en-us/articles/360002172334-Installing-and-Updating-FUSE-on-Mac#
* https://support.tresorit.com/hc/en-us/articles/216114397-Third-party-services
* https://support.tresorit.com/hc/en-us/articles/360000936353-Glossary
* https://wiki.archlinux.org/index.php/Dropbox
* https://help.nextcloud.com/t/solved-multiple-folder-sync-across-drives/13639
* https://help.nextcloud.com/t/how-to-sync-a-folder-on-a-server/12901
* https://help.nextcloud.com/t/to-be-clear-no-way-to-have-2way-sync-with-android-client/35027
* https://gladinet.com/CloudEnterprise/SelfHostedDropbox.aspx
* https://docs.nextcloud.com/desktop/2.5/installing.html
* https://docs.nextcloud.com/desktop/2.3/navigating.html
* https://github.com/syncany/syncany
* https://github.com/pydio
* https://github.com/pydio/cells
* https://github.com/pydio/cells-client
* https://www.dropbox.com/developers-v1/datastore/docs/http
* https://www.dropbox.com/developers/documentation/http/documentation
* https://en.wikipedia.org/wiki/Rsync#Variations
* http://librsync.sourceforge.net/
* https://en.wikipedia.org/wiki/Solid_compression
* https://serverfault.com/questions/52861/how-does-dropbox-version-upload-large-files
* https://blogs.dropbox.com/tech/2016/05/inside-the-magic-pocket/
* https://www.pcloud.com/download-free-online-cloud-file-storage.html
* https://news.ycombinator.com/item?id=20163389
* https://news.ycombinator.com/item?id=12619722
* https://news.ycombinator.com/item?id=12463338&p=2
* https://news.ycombinator.com/item?id=11570888
* https://librevault.com/
* https://www.seafile.com/en/features/
* https://owncloud.org/news/welcome-delta-sync-for-owncloud/
* https://github.com/owncloud/client/wiki/DeltaSync
* https://neon-bindings.com/docs/functions
* https://nihaocloud.zendesk.com/hc/en-us/articles/115003155753-How-to-Setup-and-use-Seafile-Drive-Client-on-Mac-Windows
* https://www.networkworld.com/article/3142390/down-the-rabbit-hole-part-6-secure-and-private-online-file-storage.html
* https://www.linux.com/blog/learn/2019/2/5-linux-gui-cloud-backup-tools
* https://filebrowser.github.io/

Pouvoir convertir des fichiers (audio, video)


3 etats:
- local mais pas cloud
- cloud mais pas local
- synchronized
