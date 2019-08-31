See https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/


Vue frameworks:

* https://github.com/vuejs/awesome-vue#frameworks
* https://element.eleme.io/#/en-US/component/timeline


-------------------------

* https://github.com/KELiON/cerebro
* https://github.com/Physiix/topsi-project-manager

Desktop: Plugin system

## Gui

* maintenir l'etat dans JS et on passe que ce qui est necessaire a Rust (plus facile mais risque de prendre pas mal de RAM).

* Etat dans Rust, mais ocmment on maintient un etat global et des sosu etats (pour chaque composant/BLoC?)


### State

* Tout l'etat dans Rust dans une seule struct avec des sous structs, mais c'est galere, comment on libere la memoire des
fonctions non utilisees ?

* Seulement IO, c'est deja un bon debut mais du coup c'est dommage d'avoir acces a tout node, et tout chrome

* Rien, pas possible, dans tous les cas il faut un daemon en Rust pour les fichiers et pour tout ce qui est cryptography,
et ca serait pas tres rapide, et prendrait la memoire qu'on connait....


### Comment on parle avec Rust?

Sync/Async ?

Dispatch ?
oui, enfin un truc qui peut etre facilement echangés avec jsonrpc

notifications ?

global events/stream ?


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

# Redux
reducers: prennent l’état et une commande (action) en paramètres, et modifient l’état (pure foncions)

