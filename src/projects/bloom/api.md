# Api

API should be transport agnostic and based on messages (or events).

we register handlers like

```rust
server.handle("com.event.type", handler);
```


##  Soit http, avec routes et codes...

NON, beaucoup trop protocole dependant, et pas assez evolutif.


## un seul endpoint pour toutes les apps ("notes/update")
## 1 endpoint par app (/notes -> "update")
## websockets
## HTTP/2
## GRPC
## GraphQL
## SSE
## LongPolling

## Exameples de real times apis

* https://api.slack.com/rtm
* https://realtimeapi.io/hub/realtime-api-design-guide/
* https://blog.pusher.com/5-reasons-you-should-have-a-real-time-api/
* https://becominghuman.ai/getting-started-with-building-realtime-api-infrastructure-a19601fc794e
* https://www.slideshare.net/rossmason/realtime-apis
* https://stackoverflow.com/questions/22535217/whats-are-some-real-time-data-sources
* https://hackernoon.com/pushpin-an-open-source-library-that-turns-rest-apis-into-realtime-apis-ebb3253e63ce
* https://rocket.chat/docs/developer-guides/realtime-api/
* https://developers.google.com/realtime/model-events
* https://developer.metro.net/introduction/realtime-api-overview/realtime-api-examples/
* https://www.ably.io/documentation
* https://www.pubnub.com/docs
* https://www.kraken.com/features/websocket-api
* https://www.asyncapi.org/
* https://docs.bitfinex.com/docs
* https://github.com/coinsuperapi/websocket_API_docs_en
* https://docs.gemini.com/websocket-api/
* https://developers.home-assistant.io/docs/en/external_api_websocket.html
* https://www.bitmex.com/app/wsAPI
* http://signalk.org/specification/1.0.0/doc/streaming_api.html
* https://api.mattermost.com/
* https://docs.pro.coinbase.com/
* https://docs.poloniex.com/#introduction
* https://matrix.org/docs/spec/client_server/r0.5.0#events
* https://matrix.org/docs/spec/proposals
* https://www.dropboxforum.com/t5/Files-folders/Can-you-explain-the-difference-between-Smart-Sync-and-Selective/td-p/319218





# RPC vs graphQL vs REST

RPC:
separation du transport, On peut ‘facilement’ changer l’encodage et le transport
http + web socket
JSONRPC? Twirp?

Error code: String
Method not found
Invalid params
Internal error
Invalid Request: The JSON sent is not a valid Request object.
Parse error: Invalid JSON was received by the server. An error occurred on the server while parsing the JSON text.

Method format:
- Entity.Method.version
Account.Create.v1, Account.delete.v1, account.update.v1
- Entity.Method(version)
Account.Create, account.deletev2
- Service.Method(version)
Notes.Note.Updatev2


Graphql:

Requites plus longues… ca peut prendre pas mal de place en rfontend

# Pagination
from / to -> date
Offset / limit -> pagination


# events metadata

actor_id
session_id
(project_id, organization_id...)

---------------------------

* https://github.com/matrix-org/matrix-doc/blob/master/drafts/websockets.rst
* https://golang.org/src/net/rpc/jsonrpc/client.go?s=2889:2944#L108
* https://matrix.org/docs/spec/client_server/latest
* https://github.com/elpheria/rpc-websockets/blob/master/src/lib/client.js

# Messages

```json
// Adjacently tagged)
{
    "type": "com.bloom42.package.message_type",
    "message": { // or data
        // ...
    }
}

// Internally tagged)
{
    "type": "com.bloom42.package.message_type",
     // ...
}

// Externally tagged
{
    "com.bloom42.package.message_type": {
     // ...
    }
}

{
    "id": "unique_id",
    "message": {
        "type": "com.bloom42.package.message_type",
        "data": {
            // ...
        }
    },
}
```


en gros ca envoie les message, enregistre un callback dans une map pending[id]callback,
et a chaque fois que la socket recoit un message, on regarde si il y a un ID et si il est dans pending,
le cas echeant on appelle le callback (si il n'y a pas d'id c'est une notification)

+ un systeme de pubsub, l'event envoye depuis le serveur ne contient pas un champs ID, mais un champs
stream (ou subscription). Est-ce utile ? car on peut avoir cette fonctionnalite grace au typage des events

{
    "id": "12345",
    "method": "send",
    "params": {
        "room_id": "!d41d8cd:matrix.org",
        "event_type": "m.room.message",
        "content": {
            "msgtype": "m.text",
            "body": "hello"
        }
    }
}

Server response:

{
    "id": "12345",
    "result": {
        "event_id": "$66697273743031:matrix.org"
    }
}

{
    "id": "12345",
    "data": {
        "event_id": "$66697273743031:matrix.org"
    },
    "error": null <- est -ce quon le met ca ?
}

* https://matrix.org/docs/spec/client_server/latest#api-standards

Alternative server response, in case of error:

{
    "id": "12345",
    "error": {
       "code": "M_MISSING_PARAM", // "COM.BLOOM42.ERROR_TYPE"
       "message": "Missing parameter: event_type"
    },
    "data": null ?? <- est -ce quon le met ca ?
}


Comment on fait pour les uploads?

* un endpoint HTTP

# pagination:
https://developer.atlassian.com/server/confluence/pagination-in-the-rest-api/
?limit=5&start=5"


* https://github.com/dessalines/lemmy/blob/master/docs/api.md
* https://www.dropbox.com/developers/documentation/http/documentation#files-get_metadata
* https://developers.google.com/drive/api/v3/about-files
* https://www.algolia.com/doc/rest-api/search/#search-index-get
* https://docs.syncthing.net/dev/events.html
* https://gitlab.com/gitlab-org/gitlab-ce/issues/26396
* https://latacora.micro.blog/2018/06/12/a-childs-garden.html


# Polling / websockets
* https://medium.com/@hendrikwallbaum/polling-using-rxjs-54f23eb78ded
* https://journal.willhackett.com/im-polling-spotify-because-they-have-no-websockets-ws-scaling-battery-drain-3cb7d3f5f2ad
* https://journal.willhackett.com/roll-your-own-platform-as-a-service-will-hackett-6867a49ee471
* https://thedebuggers.com/client-server-communication-http-polling-sse-websockets-comets/
* https://dev.to/nwtgck/simple-chat-over-only-http-without-websocket-and-webrtc-1n4m
* https://news.ycombinator.com/item?id=4030816
* https://news.ycombinator.com/item?id=18460037
* https://www.ably.io/concepts/long-polling
* https://www.reddit.com/r/rust/comments/cj0ofp/has_anyone_written_a_gitea_like_in_rust_yet/
* https://github.com/matrix-org/dendrite/blob/master/DESIGN.md
* https://github.com/matrix-org/dendrite/blob/master/WIRING.md
* https://matrix.org/docs/spec/client_server/latest#syncing
