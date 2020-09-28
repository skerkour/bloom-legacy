# Synchronization

One of the primary requirement for Bloom is offline support thus clients should be able to synchronize their state with server's one.

The synchronization mechanism operates on encrypted objects. For the encryption scheme, please see the
[security](security) page.

In contrary to Signal or other E2EE messaging apps that favor perfect forward secrecy, here the server is the single source of truth and it's the role of clients to converge to the same state. It allows to not lose previous data when a new device is paired (no history lost).


## Requirements

- sync when signing-in (encrypted keys, existing items like notes, calendar...) (*Cold boot*)
- sync of encrypted objects. Server should not be able to distinguish a `Note` from a `CalendarEvent`
- sync groups' objects



## Challenges

- how to avoir objects' ids collision
  - create UID client side
- how to handle conflicts (concurrently updated objects)
- how to fetch changes from server to sync down clients, and how to send local changes to syncup with the server
- how to handle deletion?
  - log of deleted objects?
  - data to `null`, and then client handle the deletion
- how to send other notifications like *kicked from group*?

## Spec

Server side, every `User` and `Group` have a `state` property`. Each time a client
does a sync operation (to create, update or delete objects), the `state` property should be updated.
Currently, is just a `BIGINT` incremented each time. The `state` property should be transmitted
through the API and stored locally as a `string` ().
Currently, it's stored locally as the `local_state` preference.


Just after Signin, clients must call the API to sync with a `state` of `null`, and perform all the
needed sync.


Server must never accept a `sync` command from a client that is not in the latest state. It's the
responsibility of client to pull the latest state before pushing updates.


When a client want to push changes, it should first pull the latest changes, resolve conflicts, and
then push it's objects which are `outOfSync`.

to apply changes clients should apply the following algorithm:
```go
for _, changedObject := changes {
  localObject := findObjectForID()
  if localObject == nil {
    // create localObject
  } else {
    if localObject.IsOutOfSync {
      // conflict
      // create a copy of localObject with a new ID
    }

    if changedObject.Data == nil {
      // delete localObject
    } else {
      // update localObject
    }
  }
}
```


### Data types

#### Server DB

```go
type EncryptedObject struct {
  id UUID!
  updatedAtState string!
  data []byte
  // ...

  user_id UUID
  group_id UUID
}
```

#### API

```graphql
type EncryptedObject {
  id: Uuid!
  lastUpdatedState: String!
  data: BYTES
  // ...
}

type Changes {
  oldState: String!
  newState: String!
  hasMoreChanges: Boolean!
  objects: ObjectEdge
}

type Query {
  changes(sinceState: String): Changes
}

type SyncParams {
  localState: String!
  objects: [Objects!]!
}

type Sync {
  newState: String!
}

type Mutation {
  sync(input: SyncParams!): Sync!
}
```

#### Local

```go
type EncryptedObjectData struct {
  type string
  data ANY
}
```

#### Local DB

```go
type Object struct {
  type string // the decrypted object type ex 'com.bloom42.bloom.note'
  data ANY // the actual object
  updatedAtState string
  // the IsOutOfSync indicates that the object was updated after the clients has last fetched
  // changes from server
  isOutOfSync bool
  // ...
}
```



## Research

### JMAP


> JMAP puts the burden of calculating state-related things onto the server—how exactly you do it, and the nature of your state strings, is up to you.
>
> The simplest thing possible is to not do any state management, but have the state string be something new after each modification, e.g. for an RDBMS the latest row ID in a table, and in any /changes method call respond “cannotCalculateChanges”. This makes a mockery of JMAP, though, undermining its benefits, so it’s really only useful as a get-started-quickly-and-make-it-work-properly-later measure. (Nonetheless is is useful for prototyping frontends before you bother making the backend work well.)
>
> But once you’re actually trying to calculate deltas, this means that you must design your state strings in such a way that you can calculate the deltas from them.
>
> Typically the easiest data model to think about is an append-only type, where you can create new records but no existing records may be updated. If you have an auto-incrementing ID for the type, you can use the highest ID as the state string, and calculate the changes by casting the state string to an ID and reporting all IDs greater than that one.
>
> Another common model is to add a new row for each revision of a record, rather than modifying a row. This is called journaling. (Most databases do journaling internally, incidentally.) Then instead of having id be the primary key, use a composite key of (id, revision). For best convenience, make that revision your state string. It could be a counter in the database, it could be a timestamp (which you can then co-opt as a “last updated” field), it could be something else. (JMAP is fundamentally compatible with a multi-master scenario, but it gets a lot fiddlier, so I won’t go into it here.) Presuming your revisions are ordered, this again gives you the ability to easily determine which records have changed—in SQL, roughly SELECT DISTINCT id FROM type WHERE revision > ?. I’ll leave you to figure out how other queries could only consider the latest revision of each record. (It’s doable, but can be messy.)
>
> This can tie in very nicely with the tombstone approach to deletion, and undo or user-accessible backup recovery systems.
>
> You can then do things like remove old revisions every so often, if you want to keep the database light. (RDBMSes do just that on their journaling, depending on their configuration; it’s commonly called vacuuming the database.)
>
> Time-series databases would also work very neatly in this space.
>
> There are two things in particular to remember:
>
> JMAP does not require that you be able to calculate what has changed within each record; only to identify the IDs of records that have changed. If you have diffs, that can be an easy way, but if you don’t, don’t fret.
>
> Although state strings are opaque to clients, the server can attribute meaning to them. For reference, Fastmail’s state strings are mostly incrementing integers, stringified. Some are two- or three-tuples, including the state strings of another type, when that other type changing could precipitate changes in this type (e.g. Mailbox ACL changes could make emails disappear, so the Email state string is of the form `‹autoincrementing counter for email›.‹mailbox state string›` or similar. There are similar dependencies in Topicbox too, with the messages you can view depending on your subscription to a group.
>
> I’ve skipped mention of queries. Handling state strings in queries has a tendency to be fiendishly difficult to implement thoroughly, and is much more optional than record type state strings—it lets you do things like have newly-matching messages appear in the list (and in the right place) when you’re searching for emails. If you’re going for end-to-end encryption, any queries you may have are probably simple enough that either you don’t worry about it (if you have any queries, just declare it unsupported with "canCalculateChanges": false in the /query response) or it’s not terribly difficult (e.g. if you have a query that is sorting by date only and you use date as your revision key, it’s probably straightforward).
>
> Indeed, for end-to-end encryption you probably won’t need or want half the power of JMAP, though the underlying object synchronisation primitives are probably useful enough to make using JMAP worthwhile, in my opinion.
>
> The approaches I’ve outlined here are all about offloading it to the database. That’s not the only way you could implement things, of course. As one alternative, if you remember how HN used to be where pagination was handled with persisted coroutines (so that the links would expire after ten minutes), well, that style of approach could work as well—give each user a state string and store whatever details you like about it in program memory! But that approach would have some severe disadvantages like all synchronised records having to be thrown away by clients whenever you restarted the server or whenever the state strings were cleaned up.


> >  What is the rational for using a tuple for the Invocation type, rather than an object of the following shape?
> >
> >    { "method": "", "params": {}, "clientId": "" }
> >
> > Like in JSON RPC
>
> Brevity, a modicum of efficiency, and readability, I expect. It was in this shape before I started at Fastmail, though I think client ID hadn’t been added at that time so it was just [method, params]. If you use an array, you can guarantee the source order, but if you use an object, it could be written {clientId, params, method} or any order at all. Also this is a type that definitely is not to be extended with extra properties, so mandating that it’s a three-element array is a decent way of doing that.
>
> > Is there queries that can't be made in GraphQL but can be be made in JMAP thanks to back-reference? (also, if you publish the blog post, I think a comparison with GraphQL can be great because used as an RPC mechanism, they look really similar).
>
> Yeah, GraphQL is a significant competitor in the space of web APIs, and the blog posts I’ve been writing in my mind for the last year address it. The big difference is that GraphQL is just about getting or mutating current values of things, while JMAP is an object synchronisation protocol. GraphQL is more flexible in some ways due to its graphy nature, but if you want to work offline or cache records GraphQL basically gives you nothing at all—and you probably can’t build an object synchronisation protocol in a graphy way, it just doesn’t really work, so you’d need some other kind of synchronisation design, which would be a research area.
>
> I’ve never actually built anything with GraphQL; I’ve just looked from outside and made sure I know the theory of it.
>
> With backreferences you can do things like this example on page 25 of RFC 8620: fetch from/date/subject for every Email in the first 10 Threads in the inbox (sorted newest first). It’s a tad verbose, but it falls out very naturally from the data model. If you wanted to achieve something like that in GraphQL, you’d need to hard-code the design into the server, or else make it probably-three calls to the server, round trip and all. (And a few years ago JMAP worked that way, before backreferences were invented—instead of saying /query and then /get those ids, the querying method had a boolean property “fetchFoos” which if true made it act like a get method as well, with those IDs. But that didn’t let you chain things like this.)
>
> Still further, JMAP backreferences can involve getters and setters in the same series. Not only can you do things like Foo/set followed by Foo/get in the one HTTP call (which I don’t think you can do with GraphQL, putting a mutation and a query in the same call? but I’m not sure), but you can also get things based upon the result from the server. You can express things like “find all the boxes matching this search, and destroy any widgets within them, then tell me the weight left in the boxes” in one call, whereas in GraphQL that would be at least three, something a little like `box(…) { widget { id } }` to get the IDs, then a mutation to destroy those widgets by ID, then something like `box(…) { weight }` to get their weights. (Notwithstanding this, I will mention that in Fastmail’s webmail deleting all emails matching a search is not done this way, but rather by fetching all the IDs and then deleting them in batches, because that way you can show a progress bar and offer a cancel button, which we found offered a much more satisfying experience, even if it’s strictly a tad slower.)
>
> I rather like the way Cap’n Proto brands its concept equivalent to JMAP’s backreferences: time travel. (See also “Infinitely faster”, “cerealization protocol”—Cap’n Proto’s pretty good at distinctive branding.)
>
> To temper your potential joy at how awesome backreferences are: if you’re talking end-to-end encryption, none of this is likely to be particularly useful to you, because you probably won’t be able to do much interesting method chaining on the backend, since the information is all concealed to it.
>
> But still, my comparison of GraphQL to JMAP is this:
>
> GraphQL is a easy-to-work-with hacker’s tool when you just want to get something simple working and don’t need any offline functionality. (People do things for offline functionality for it from time to time, but it’s fundamentally at least one of unsound and read-only.) For some types of problems it can work well, and some of the tooling around it is seriously great.
>
> JMAP is a serious business tool that’s less cool and takes a certain amount of academic rigour to handle correctly, but is sound. If you want solid offline support or any form of synchronisation, you need something at least like it, and JMAP is currently the only such base upon which to build web, mobile and desktop software alike. (There have been object synchronisation protocols before, e.g. IMAP in the email space, but none have been apt for web, mobile and desktop. JMAP is deliberately built on web and mobile tech.)
>
> I’ve certainly drunk the JMAP kool-aid and intend to build primarily JMAP interfaces for my own work.
>


by [Chris Morgan](https://chrismorgan.info/)

#### Resources

* https://jmap.io/spec-core.html
* https://news.ycombinator.com/item?id=18996200 (JMAP: Like IMAP but Not Really)
* https://news.ycombinator.com/item?id=20720630 (Making email more modern with JMAP)
* https://news.ycombinator.com/item?id=20477212 The JSON Meta Application Protocol (JMAP))
* https://groups.google.com/forum/#!forum/jmap-discuss
* https://groups.google.com/forum/#!topic/jmap-discuss/ja54Dr1wXWk



### CRDT

#### Resources
* https://crdt.tech/resources
* https://github.com/ept/crdt-website


### Other

#### Resources

* https://killtheradio.net/technology/turtls-new-syncing-architecture/
* https://github.com/c4milo/gsync
* https://www.linkedin.com/pulse/client-server-sync-algorithm-using-objects-diff-patch-kumar-krishna
* https://matrix.org/docs/spec/client_server/r0.6.0#syncing
* https://josephg.com/blog/api-for-changes/
* https://docs.hamoni.tech/extras/conflict-management.html
* https://www.contentful.com/developers/docs/concepts/sync/
* https://www.twilio.com/docs/sync/documents
* https://github.com/rdiff-backup/rdiff-backup
* https://janmonschke.com/projects/diffsync.html
* https://docs.realm.io/sync/using-synced-realms/syncing-data
