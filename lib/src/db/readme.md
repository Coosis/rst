# Collections
1. `users`
2. `messages`
3. `chats`

## `users`
Indexed using a `uuid` field called `id`.

## `messages`
Indexed using `uuid` fields: `id`, `from` and `to`.

## `chats`
Indexed using a `uuid` field called `id`.
Due to it holding participants' uuids, it's also multi-indexed using the `members` field.

# Deployment
```sh
db.createCollection('users')
db.users.createIndex({ id: 1 }, { unique: true })
db.users.createIndex({ phone: 1 }, { unique: true, sparse: true })
db.users.createIndex({ email: 1 }, { unique: true, sparse: true })
db.createCollection('messages')
db.messages.createIndex({ id: 1 }, { unique: true })
db.messages.createIndex({ to: 1 }, { unique: true })
db.messages.createIndex({ from: 1 }, { unique: true })
db.createCollection('chats')
db.chats.createIndex({ id: 1 }, { unique: true })
db.chats.createIndex({ members: 1 })
db.createCollection('invites')
db.invites.createIndex({ id: 1 }, { unique: true })
db.invites.createIndex({ to: 1 }, { unique: true })
db.invites.createIndex({ from: 1 }, { unique: true })
```
