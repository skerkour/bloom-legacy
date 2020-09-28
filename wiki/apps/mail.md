email replacement

stream_id

bio ua lie ude sognature, a venir recup sur le profile de l'utilsateur quand necessaire





(probleme du coup on leake des metadata, idealementm que to & data)

- stream_id 
- type
- from
- to
- data / payload / message
- timestamp
- signature (crypto)


----------------------------------------------------------------------------------------------------


What is an Event?

    An event encapsulates a change in state (what has happened)
    It should be lightweight and carry only enough information required about the change in state
    An event is distributed to notify any interested parties
    Events are distributed through channels such as streaming and messaging

An example, when a consumer borrows a DVD, the DVD state changes from “for rent” to “rented”.
What is a Message?

    A message encapsulates the intention / action (what has to happen)
    It is not lightweight containing a payload of all data required for processing
    A message can be formatted and adhere to a contract to be suitable for the interested parties
    A message is distributed through channels such as messaging


    https://robertleggett.blog/2020/03/02/choosing-event-streaming-or-messaging-for-your-architecture/

----------------------------------------------------------------------------------------------------

https://tonsky.me/blog/streams/
https://delta.chat/
https://medium.com/@collinmathilde/why-its-so-hard-to-innovate-in-the-e-mail-space-9874e08e3426
https://www.intercom.com/blog/the-future-of-email-products/
https://friendlybit.com/security/how-to-make-e-mail-encrypted-for-everyone/
https://zapier.com/blog/best-email-app-for-iphone-ipad/
https://zapier.com/blog/best-email-app/

conversation = stream = topic = thread


il faut que que l'on puisse rajouter un stream dans la partie de gauche de son client email:
- un dossier regroupe une ou plusieurs conversations
par exemple, ca peut etre toutes les newsletters,
ou un 'groupe' topicbox


Folders ou labels?

chats:
pour les conversations 1 to 1 ou de groupe


Inbox:
latest messages qui ne sont pas dans un dossier

Snoozed:
pour setup des reminders sur des messages specifiques

Done:
pour les conversations dont on s'est occupé. equivalent de **archive**


Draft:
on retrouve les drafts


Scheduled:
message qui sont schedulé pour un envoie futur


Sent:
liste tous les derniers messages que l'on a envoye dans leurs conversations respectives

Spam:
conversations marquees comme spam


Trash:
conversation marquees pour etre supprimee. supprimees 48h apres


Folders...
User defined folder avec certain folder par defaut



----------------------------------------------------------------------------------------------------

Working with email at per-message VS working at email at per-stream

fournir un bridge comme proton mail:
Ainsi les gens peuvent continuer d'utiliser un client email non compatiblet et parler avec un serveur compatible


comment on s'inscrit ?
- soit on met son email sur le site, comme actuellement -> contexte
- soit depuis notre client, en envoyant un message ua service -> plus de controle



push or pull ?
This sounds extremely exciting. But there's one question: how do I tell someone to subscribe to a
stream? Do I need to email them a link to my stream? The killer feature of email is that it allows anyone to contact anyone else.

account creation


comment on fait pour suivre qqun (comme twitter) et qu'il nous envoie des chat aussi:
- on suit une adress genre sylvain+social@kerkour.fr


RSS / dsiqus:
to = site
stream = post
on souscrit a un site, et on recoit les nouveaux articles par email.
possibilité de commenter en commentant sous le post



chat:
to = user
stream = conversation


email:
to = user
stream = conversation



mailing list/topicsbox:
to = group/list
stream = topic

workflow:
l'utilisateur peut
- envoyer un email directement sans etre connecté.
  dans ce cas, il est directement souscrit a tous les emails du stream
- entrer so nemail pour creer un compte / se connecter et ainsi interargir directement depuis l'interface web
  dans ce cas, par default il est pas souscrit aux messages des streams

le reglage des emails recu peut se faire soit au niveau du to, soit au niveau du stream



zulip:
to = ??
stream = topic
?? = streams

zulip ce qui change par rapport a une mailing list, c'est qu'on peut avoir plusieurs to par groupe


slack:
to = worksapce
stream = channel


newsletter:
to = lists
stream = issues + comments



Forum:
to = Forum
stream = topic


Micrblogging (twitter, mastodon):
to = Users + world
stream = tweet

pour twetter on envoie un message a world(public?)@instance.com

il apparait dans l'interface web,
ensuite les gens peuvent repondre soit sur l'interface web, soit en envoyant un message



contacts:
to = User
stream = contact




notes:
to = User
stream = note




Calendar:
to = User
stream = note




Tickets/Gitlab issue/zendesk:
to = Project
stream = issue & comments



Youtube:
to = User / World
stream = Video + comments




Facebook:
to = Users
stream = post + comments




instagram:
to = Users
stream = post + comments



reddit:
to = Users
stream = post + comments


blog:
to = users
stream = post + commetns