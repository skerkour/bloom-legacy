# BitFlow

1. [Overview](#overview)
2. [Scenarios](#scenarios)
3. [Non goals](#non-goals)
4. [Models](#models)
5. [Views](#views)
6. [Open issues](#open-issues)

-------------------


## Overview

`BitFlow` est un gestionnaire de telechargement dans le cloud.

il doit permettre de pouvoir telecharger des fichiers depuis plusieurs sources (torrents, http, youtube, FTP...)
directement dans le Drive d'un utilisateur.



possibiltie de filtrer la liste des torrents :
- all
- actives
- queded
- finished

flow:
- on clique sur plus
- un fenetre avec un dialogue pour entrer une URL s'ouvre
- on entre le magnet, le lien vers le fichier, ou le lien vers le service (vimeo, youtube....)
- detection du service
- vue (tjr dans le dialog) avec les fichiers a ltelecharger et eventuellement des parametres.
- le dialog se ferme et le le telechargement est ajoute a la liste


service bitflow recoit la requete
telecharge, envoie sur s3, et envoie sur sqs les fichiers envoyes sur s3


Business model:
- gratuit: nombre de telechargement paralleles limite, max download size
- abonnement: auguementer les nombre de telechargements paralleles,

## Scenarios


### 1

Sylvain wants to download a torrent directly to his drive and then stream it to his TV.



## Non goals
- upload (from drive to s3 or other)
- seedbox (ratio never > 1:1)
- torrent search engine


## Models

### Download
- state (queued, downloading, completed)
- type: HTTP/TORRENT
- name
- progress (0-100)
- error

#### events
- queued (Created)
- started
- failed
- completed
- progress updated
- removed


### Profile
- settings -> downlaod directory

#### events
- created



## Views


### Downloads list (Dashboard ?)
une list des downloads en cours avec la possibilite de stopper, et supprimer


### History


### Settings
Choose download location

### Download information (file, )

dialog


## Open issues
- pause/ resume / stop
- uploaded un fichier torrent


## Ideas
alternate names:
- Beam

Torrent
Scp
Ftp
Http
Youtube
from S3

Bouton fab + avec
- Torrent
- HTTP
- Youtube
- other (ouvre une modale)
