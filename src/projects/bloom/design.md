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
* https://blog.bitsrc.io/building-a-consistent-ui-design-system-4481fb37470f
* https://gallery.manypixels.co/
* https://undraw.co/
* https://www.drawkit.io/illustrations/shipping-package-colour
* https://hackertarget.com/cowrie-honeypot-analysis-24hrs/
* https://sysdig.com/blog/detecting-cryptojacking/
* https://laskowski-tech.com/2017/12/19/setting-up-a-honeypot-using-opencanary/
* https://www.deepshankaryadav.net/low-cost-honeypots-as-enterprise-defense-mechanism/
* https://news.ycombinator.com/item?id=15288720
* http://thecodist.com/article/every-server-is-a-kind-of-honeypot
* https://doublepulsar.com/eternalpot-lessons-from-building-a-global-nation-state-smb-exploit-honeypot-infrastructure-3f2a0b064ffe
* https://hackertarget.com/cowrie-honeypot-analysis-24hrs
* https://medium.com/@sudojune/deploying-a-honeypot-on-aws-5bb414753f32
* https://fr.wikipedia.org/wiki/Honeypot
* https://en.wikipedia.org/wiki/Honeypot_(computing)
* https://www.leblogduhacker.fr/les-honeypots-des-pieges-a-pirates/



See https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/


Vue frameworks:

* https://github.com/vuejs/awesome-vue#frameworks
* https://element.eleme.io/#/en-US/component/timeline


-------------------------

* https://github.com/KELiON/cerebro
* https://github.com/Physiix/topsi-project-manager
* https://github.com/atom-archive/xray
* https://www.behance.net/gallery/79789491/Perfect-Apps-WEB-Workspace

Desktop: Plugin system

## Gui

* maintenir l'etat dans JS et on passe que ce qui est necessaire a Rust (plus facile mais risque de prendre pas mal de RAM).

* Etat dans Rust, mais ocmment on maintient un etat global et des sosu etats (pour chaque composant/BLoC?)


### State

* Tout l'etat dans Rust dans une seule struct avec des sous structs, mais c'est galere, comment on libere la memoire des
fonctions non utilisees ?
  * https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton

* Seulement IO, c'est deja un bon debut mais du coup c'est dommage d'avoir acces a tout node, et tout chrome

* Rien, pas possible, dans tous les cas il faut un daemon en Rust pour les fichiers et pour tout ce qui est cryptography,
et ca serait pas tres rapide, et prendrait la memoire qu'on connait....


### Comment on parle avec Rust?

Sync/Async ?

Dispatch ?
oui, enfin un truc qui peut etre facilement echangés avec jsonrpc

notifications ?

global events/stream ?


question: ls "rpc" stub entre Rust est JS on le met de quel cote? Rust ou JS?

Comment on organise rust pour qu'il puisse emettre des events?

threads?


-----------------

# Apps


## pages

* Chats: display personnal chats
* Discover (Explore): Explore communities, brands, services...
* contacts: Manage contacts....
* Communities: Display list of communities we are part of
* Me (Profile): update profile, account and settings...

combinations:

* chats, contacts, explore, profile
* chats, communities, contacts, profile


le genie de weChat: Chaque interraction social passe a travers l'app


soit bottom menu,
soit a la discord

la question: Est-ce qu'on veut que les workspace soient mis hyper en avant ?

je ne pense pas

kais alors comment on affiche la liste des channels dans un workspace si il n'y a pas de sidemenu?



Combien on veut d'app dans l'ecosysteme ?
- chat
- productivite
- administratif
- office suite
- media (Music, video, podcasts?)
- service (https://www.v3cube.com/gojek-clone/ :taxi, Car Rental, Common Deliveries, )
- shopping
- Home (domotic)
- services financiers
- creatifs (sketch, photoshop, Imovie...)
- code editors



si on devait reduire:
- chat
- shopping
- media
- bloom (tout le reste)
- office suite
- games


avoir un feed global (a la twitter), et un feed local (pour permettre aux commercants, office de tourisme...)
de publier des annonces

ou mettre le wallet ?

Apple:

books
podcasts
findMyFriend
ITunesU
IMovie
Pages
Apple Books
Apple podcasts
Keynote
Numbers
Xcode
Classroom
QuickTime



wechat:
https://hackernoon.com/heres-a-tour-of-wechat-the-greatest-mobile-app-in-the-world-5be62536ff4d
* DIY postcard
* Book an appointment at a local hospital
* Weather forecast
* See results of your college entrance exam
* Pay your speeding and parking tickets!
* Get documents notarized
* Inquire about illegal vehicles
* See all sorts of transport schedules
* Look at heat maps of your city to avoid traffic!
* Social security inquiries.

COMMENT METTRE EN RELATION LES MARQUES / CREATEURS AVEC LE PUBLIC?


1. Compare Prices
2. Quick Pay
3. Mobile Account Top Up
4. Pay Bills
5. Web WeChat
6.  Edit Aliases
7. Group Tags
8. Voiceprint
9. Track Deliveries
10. Order Food, Reserve Tables and Interact with Your Favorite Brands
11. Send a Postcard
12. Step Tracker

There are three forms of official account- Subscription accounts, Service accounts and enterprise accounts.

Mobile payments for both offline and online goods




3 options se demarquent:

- Bottom
Home (feed), chats, teams (groups), me
et on met toutes les apps dans me

- Bottom
Chats, teams, discover, me

- drawer:
comme deja pense


boutton (+):
group chat
create contacts
Scan QR code
receive moeny...
create team

team, group, organization, workspace, community, crew, band



ou mettre les settings/account management dans desktop ?


Wechat cest pas un chat, cest un outil pour  connecter lea gens


Il gere bien les petits cercles, et les relations avec les.business, mais pas les communautés

Comment gerer les projets individuellements ?


ajouter possibilite de mettre les icones des app individuelles sur le home


Avatar url:

On les updates tous, on remet celui par défaut
Ensuite le nouveau de nécessitera pas de stocker la full url dans la DB, mais juste le path sans l’host, comme ca on peut changer l’host
Delete ensuite sur ans
Delete avatar on avatar update and account::delete


https://github.com/keybase/client/tree/master/go


* https://www.robinwieruch.de/the-soundcloud-client-in-react-redux/
* https://www.rust-lang.org/what/wasm

# Station & co workspaces

* https://www.youtube.com/watch?v=CZR2qs91nIQ
* https://www.youtube.com/watch?v=vee8jkCblPA
* https://www.youtube.com/watch?v=Z5vCWw3993M
* https://www.youtube.com/watch?v=o2RRDS2tIoY
* http://www.igoogleportal.com/
* https://meetfranz.com/
* https://www.producthunt.com/posts/station-3
* https://techcrunch.com/2017/10/26/station-combines-all-your-messy-web-apps-into-a-single-app/
* https://getstation.com/
* https://www.theverge.com/2018/12/27/18148937/best-mac-apps-2018-apple-macos-adobe-alfred
* https://medium.com/getstation/your-way-of-working-belongs-to-the-stone-age-9ff64782f40
* https://www.ausy.com/en/technical-news/mobility-one-or-multiple-apps-your-business
* https://slite.com/
* https://www.designernews.co/stories/31375-strategy-question-one-app-to-rule-them-all-or-multiple-apps-for-different-purpose
* https://forums.meteor.com/t/monolith-vs-multiple-apps-an-architecture-question/9342
* https://softwareengineering.stackexchange.com/questions/204552/why-dont-companies-ship-multiple-modules-within-a-mobile-app-are-there-concern
* https://appdevelopermagazine.com/is-this-the-age-of-the-single-purpose-app/
* https://www.enterprise-cio.com/news/2017/nov/17/are-we-approaching-age-single-purpose-app/
* https://portableapps.com/apps
* https://www.behance.net/gallery/79789491/Perfect-Apps-WEB-Workspace
* https://rambox.pro/#home
* https://wavebox.io/


* 2 sidebar, 1 pour les apps, 1 pour l'app en cours, avec une AppBar, icon de compte dans sidebar

comment on change de groupe ?
en cliquant sur notre profile, s'affiche la liste des groupes


comment on change de compte ?

combien de click pour aller d'une app d'un groupe, vers une autre d'une autre groupe ?
1 click sur notre profile, 1 click sur le groupe, 1 click sur l'app

* 2 sidebars, 1 pour les groupes, 1 pour l'app en cours, avec une Appbar, icon de compte dans AppBar

comment on change de compte ?


combien de click pour aller d'une app d'un groupe, vers une autre d'une autre groupe ?
1 click sur le groupe, 1 click sur la liste des apps, 1 click sur l'app

* 1 sidebar, ave cun bouton pour afficher les apps, icon de compte dans AppBar

Mais comment on change de groupe ?
en cliquant sur notre profile, s'affiche la liste des groupes

1 click sur notre profile, 1 click sur le groupe, 1 click sur l'app


le plus important:

* Changer d'app ? -> productivity (station, rambox)

* Changer de groupe ? -> social (slack, discord)
