See https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/


Vue frameworks:

* https://github.com/vuejs/awesome-vue#frameworks
* https://element.eleme.io/#/en-US/component/timeline


-------------------------

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
