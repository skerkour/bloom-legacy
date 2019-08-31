## Icons

in apps icons are provided as png images, 256x256, as square with 8px corner radius.

* [https://material.io/design/navigation/understanding-navigation.html](https://material.io/design/navigation/understanding-navigation.html#)

----------------

https://visme.co/blog/website-color-schemes/



les apps:

* sur la gauche, mais alors comment on fait pour la recherche ?

* ou dans la appbar


Comment on gere les groupes sur desktop ?

* quand on lance, on est sur son propre compte, en cliquant sur notre profile on peut change de groupe / compte

quand on change de groupe, les apps a gauche changent en accord

- [ ] weChat
- [ ] Slack
- [ ] Discord
- [ ] Messenger
- [ ] Line
- [ ] Telegram
- [ ] Viber
- [ ] Signal
- [ ] Wire
- [ ] Mattermost
- [ ] RocketChat


https://www.katriel.co.uk/

https://rocketbots.io/blog/ultimate-wechat-official-account-for-business-guide/
https://chinahelp4u.com/how-to-post-wechat-moments/

https://medium.com/@richard.chihong.ng/whatsapp-clone-with-flutter-in-a-week-3a65195fa080
https://www.behance.net/gallery/79789491/Perfect-Apps-WEB-Workspace


http://find-sec-bugs.github.io/bugs.htm
https://www.raywenderlich.com/778533-encryption-tutorial-for-android-getting-started
https://developer.android.com/topic/security/best-practices
https://developer.android.com/about/dashboards


# monolith
monolith vs single app:
Avantage du monolith: ne pas avoir a créer 25 000 comptes,

Avantage de multiple, quand on a un seul compte pour toutes les apps, aide a avoir un meilleur contexte

# Local first

Motivation: collaboration and ownership

Multi device

An important aspect of data ownership is that you can continue accessing the data for a long time in the future.

Il y a 2 types de fichiers:
* “normaux”, c’est juste une entree db avec le path, (et éventuellement la clé)
* ceux ouverts part une app de bloom, qui sont un “lien” vers le fichier dans l’app (comme un doc, une note?, une spreadsheet…)


Local first

Le serveur n’est plus la source de vérité, il sert de backup de synchronisation entre devices

Enjeux:

Synchronisation
Collaboration (conflits)


Fast	2. Multi-device	3. Offline	4. Collaboration	5. Longevity	6. Privacy	7. User control

CRDT


Similarly, cloud peers could be:
* an archival/backup location (especially for phones or other devices with limited storage);
* a bridge to traditional server APIs (such as weather forecasts or a stock tickers);
* a provider of burst computing resources (like rendering a video using a powerful GPU).


# Apps

- Bloom: chat, drive, notes, calendar, calculator, gallery, Music, Video, Maps
grand public, social, productivity
- Platform/genius: phaser, "gitlike"
for tech people
- Creative: for creatives (image retouche, dessin vectoriel)...
for creative people
- Office: words, spreadsheets, sldies, CRM
pour les executives


un editeur de code avec "gitlab" directement intégré, comme la possibilité de faire les MR directement depuis le code


Ajouter une colonne kernel_security_events pour logger les events importants!!

- music
- Photos
- video
- books
- podcast & audio books
- notes
- Docs (pdf, & Workd like)
- IDE
- Sheets
- Slides
- Desk => frontapp + crisp + zendesk
- Authenticator (Sync)


# next apps:
- Platform: redirect
- Platform: dependencies analysis
- Platform: static hosting: Pages
- Docs
- Platform: git
- Platform: queue
- Books
- platform: cron jobs
- platform: search
- platform: DNS
- Podcast & audio books
- platform: PaaS
- IDE
- platform: containers
- Analytics
- platform: hosted DB


Home: Dock: on peut mettre des raccourcis vers d'autres webapps


UX: Universal eXperience
M(i)X: mobile (interplanetary) experience



chromecast: il envoie seulement les commandes
on peut aussi miorror l'ecran



hascker bouton amazon pour envoyer un message automatique (bot, ex: a table, ou au secours)

franchise de jeux qui contient un jeu pour chaque mode:
- kart
- fps
- (mmo)rpg


# 3 types de données:
Celle qui appartiennent seulement a un user
Celles qui sont partagées entre plusieurs membres
Celles qui cont publiques

# service de subdomains

Domaines
sous domaines
Pour chaque domaine/sous domaines, si il resolve pour chaque type de query DNS et son historique

# Jeu de baston en 2d

Existing games:
- Dragon ball z
- Super smash bros brawl
- Skyhook
- Tower fall
- Tekken / mortal combat
Brawlhalla
Rivals of Aether
Super Smash Flash
Samurai Gunn
Nidhogg



# Si o devait inventer le futur de git:
Pas besoin de commit, c’est sync avec le server automatiquement
Realtime collaboration

Il n’y a que les merge requests qui necessitent de commenter -> on s’est rendu compte que chaque commit ne peut pas avoir un message correcte

On garde le systeme de branches et de merge request

- issues
- comments and code review

Le problème: les devs n’ont pas d’autre moyen de sauvegarder leur code que de commit

Probleme:
- comment lancer la cicd dans un tel système
- Good luck compiling at any given moment when 10 other people are changing files under you though!
- Security (si on ne ignore pas un .env




# Matrix: les rooms sont répliques sur les serveurs de chaque participant

Clients typically communicate with each other by emitting events in the context of a virtual "room". Room data is replicated across all of the homeservers whose users are participating in a given room. As such, no single homeserver has control or ownership over a given room

Pro: Le client ne se sync que sur son serveur a lui

* Every homeserver in a room keeps a content of the room’s history.
* If a homeserver in a room goes down for any reason, even if it’s the homeserver which has its name in the room’s ID, all of the other homeservers in the room can keep on talking with each other.



Cons: beaucoup de replication
Quand un utilisateur d’un petit serveur joint une room d’un gros serveur, il risque de se faire DOS



Alternative;
Pas répliquées,

Cons:
haque client doit se connecter a chaque serveur des room auxquelles il participe…



Ou alors: on envoie aux serveur fédérés que les messages nécessaires (les events qu’ils doivent envoyer aux clients), et non tout répliquer




Matrix:
C’est plus comme couchDB que xmpp: le point central est l’historique, et la replication de cet historique, non les messages



Pourquoi room et non channel ?
Pourquoi ne pas avoir utilise de verbes au passé pour décrire les évents comme c’est la norme ?



# e2ee

La question est la suivante: comment synchroniser des données entre les clients, sans les surcharger, avec une gestion des droit, et e2e encrypted
On veut aussi que l’historique ne prenne pas trop de place


- Git
- couchDb
- Crdt
- Blockchain


La gestion des droit se faire cote serveur, et la gestion des données cote client

Le type de l’event est en clair, mais pas ses données

Problem: new peers coming into a CRDT will have to download the entire operations tree in order to be in sync. solution: les client envoient des snapshot de la done de manure reguliere au serveur


2 commandes:
Sync, delete


On organize autour de commandes ou de resources ?


GITLAB PM
GitLab Issues with due-dates
Milestones with due-dates



Actor -> a un lifecycle (started, running, stopping stopped)
Est supervise (restart en cas de crash…)
Sont inclus dans un network/mesh

recoit certain type de messages -> réponse tout de suite ou en envoyant a l’actor qui call un autre message ?
Dans un système. Classique il n;y a pas de réponse, mais actix demande une réponse pour être plus dimple d’utilisation


Calls one way ?

Registry ? (Ou trouver les adresses des autres actors)

Actin-web: par default il utilise le nombre de cpu pour les workers


d’apres ce quelle comprends, un actor d’actix partage son thread avec les autre acteurs du système sauf les SynArbitrers qui ont leur propre thread


Phaser scanner actor:
Controle du nombre de workers

Le module est juste charge de valider + faire les mutations + dispatch

* https://carbon.now.sh/
* https://hyperpixel.io/
* https://news.ycombinator.com/item?id=20145206
* https://www.colorhexa.com/5fcdff
