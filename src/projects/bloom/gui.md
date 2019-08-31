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

https://github.com/keybase/client/tree/master/go

# Qt books

* https://www.packtpub.com/eu/application-development/end-end-gui-development-qt5
* https://www.packtpub.com/eu/application-development/hands-mobile-and-embedded-development-qt-5
* https://www.packtpub.com/eu/application-development/hands-embedded-programming-c17
* https://www.packtpub.com/eu/data/qt-5-and-opencv-4-computer-vision-projects


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


# Flutter & Mobile

* https://medium.com/@lucassaltoncardinali/keeping-state-with-the-bottom-navigation-bar-in-flutter-69e4168878e1
* https://bitbucket.org/arputer/flutter_with_go/src/master/
* https://flutter.dev/docs/cookbook
* https://medium.com/@kashifmin/flutter-setting-up-a-navigation-drawer-with-multiple-fragments-widgets-1914fda3c8a8
* https://github.com/fluttercommunity/flutter_contacts
* https://github.com/BaseflowIT/flutter-contacts-plugin
* https://pusher.com/tutorials/flutter-widgets
* https://api.flutter.dev/flutter/widgets/RouteObserver-class.html
* https://flutter.dev/docs/development/ui/navigation
* https://stackoverflow.com/questions/52568428/detect-pushed-routes-on-flutter
* https://stackoverflow.com/questions/46483949/how-to-get-current-route-path-in-flutter
* https://stackoverflow.com/questions/52048101/how-to-reset-the-base-route-in-my-flutter-app-that-is-pop-any-routes-and-repla
* https://github.com/flutter/plugins/blob/master/packages/firebase_analytics/lib/observer.dart#L19
* https://medium.com/jlouage/flutter-row-column-cheat-sheet-78c38d242041
* https://medium.com/coding-with-flutter/flutter-layouts-walkthrough-row-column-stack-expanded-padding-5ed64e8f9584
* https://flutter.dev/docs/development/ui/layout/box-constraints
* https://www.didierboelens.com/2018/08/reactive-programming---streams---bloc/
* https://medium.com/flutter-community/flutter-state-management-setstate-fn-is-the-easiest-and-the-most-powerful-44703c97f035
* https://flutterdoc.com/bottom-sheets-in-flutter-ec05c90453e7
* https://medium.com/flutter-community/using-sqlite-in-flutter-187c1a82e8b
* https://felangel.github.io/bloc/#/fluttertodostutorial
* https://medium.com/flutterpub/architecting-your-flutter-project-bd04e144a8f1
* https://medium.com/flutter-community/flutter-bloc-with-streams-6ed8d0a63bb8
* https://medium.com/flutter-community/why-use-rxdart-and-how-we-can-use-with-bloc-pattern-in-flutter-a64ca2c7c52d
* https://medium.com/flutterpub/effective-bloc-pattern-45c36d76d5fe
* https://blog.usejournal.com/flutter-advance-routing-and-navigator-df0f86f0974f
* https://blog.usejournal.com/flutter-advance-routing-and-navigator-971c1e97d3d2
* https://proandroiddev.com/flutter-multi-page-applications-with-routes-71d3d5f0cb26
* https://www.freecodecamp.org/news/how-to-handle-navigation-in-your-flutter-apps-ceaf2f411dcd/
* https://medium.com/flutter-community/clean-navigation-in-flutter-using-generated-routes-891bd6e000df
* https://medium.com/flutter-community/flutter-navigation-cheatsheet-a-guide-to-named-routing-dc642702b98c
* https://www.raywenderlich.com/4562634-flutter-navigation-getting-started
* https://stackoverflow.com/questions/49672706/flutter-navigation-pop-to-index-1/51562273
* https://flutterbyexample.com/flutter-widgets/
* https://flutter.dev/docs/get-started/flutter-for/declarative
* https://medium.com/flutter-community/flutter-todos-tutorial-with-flutter-bloc-d9dd833f9df3
* https://medium.com/flutter-community/flutter-bloc-pattern-for-dummies-like-me-c22d40f05a56
* https://www.freecodecamp.org/news/using-streams-blocs-and-sqlite-in-flutter-2e59e1f7cdce/
* https://medium.com/learnfazz/refactoring-with-bloc-pattern-meaningful-software-improvement-aaf1c45bfbb1
* https://medium.com/flutter-community/animate-your-flutter-with-bloc-db4a696af48e
* https://medium.com/@greg.perry/decode-streambuilder-e60948629d8e
* https://medium.com/flutter-community/flutter-state-management-has-never-been-easier-think-statelessly-then-add-reactivity-d30c75760da0
* https://medium.com/flutterpub/architect-your-flutter-project-using-bloc-pattern-part-2-d8dd1eca9ba5
* https://hackernoon.com/flutter-redux-how-to-make-shopping-list-app-1cd315e79b65
* https://medium.com/flutter/animation-management-with-flutter-and-flux-redux-94729e6585fa
* https://medium.com/flutter-community/let-me-help-you-to-understand-and-choose-a-state-management-solution-for-your-app-9ffeac834ee3
* https://blog.usejournal.com/flutter-replicating-the-ui-for-googles-gmail-app-a501ed8e7908
* https://stackoverflow.com/questions/51848123/how-to-implement-the-accountdetail-of-the-useraccountsdrawerheader-widget-to-be
* https://medium.com/flutterdevs/managing-the-state-of-a-widget-using-bloc-flutter-7789d6017f6b
* https://www.youtube.com/watch?v=EwHMSxSWIvQ
* https://medium.com/@dev.n/the-complete-flutter-series-article-3-lists-and-grids-in-flutter-b20d1a393e39
* https://pub.dev/packages/flutter_secure_storage
* https://redux.js.org/advanced/async-actions
* https://redux.js.org/advanced/middleware
* https://codeburst.io/make-a-material-design-login-page-with-flutter-the-basics-99d3acd80b18
* https://olvlvl.com/2018-04-command-dispatcher-pattern
* https://www.didierboelens.com/2018/08/reactive-programming---streams---bloc/
* https://www.didierboelens.com/2018/12/reactive-programming---streams---bloc---practical-use-cases/
* https://www.didierboelens.com/2019/04/bloc---scopedmodel---redux---comparison/
* https://flutter.dev/docs/development/packages-and-plugins/developing-packages#step-2c-add-ios-platform-code-hmswift
* https://medium.com/flutter-community/migrating-a-mobile-database-in-flutter-sqlite-44ac618e4897
* http://stacksecrets.com/flutter/flutter-running-migrations-on-sqlite
* https://pub.dev/packages/permission_handler#-readme-tab-
* https://git.pattle.im/pattle/app
* https://source.android.com/devices/tech/connect/block-numbers
* https://developer.android.com/reference/androidx/security/crypto/MasterKeys.html
* https://developer.android.com/jetpack/androidx
* https://pub.dev/packages/permission#-readme-tab-
* https://medium.com/@dev.n/the-complete-flutter-series-article-3-lists-and-grids-in-flutter-b20d1a393e39
* https://medium.com/@ali.muzaffar/securing-sharedpreferences-in-android-a21883a9cbf8
