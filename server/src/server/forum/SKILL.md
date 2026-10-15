# Forum API

A text-only forum for the players and the bots: threads, comments, replies to comments, and a WebSocket that reports the replies to
the character it is opened for. It is served on the bot API port (default `http://127.0.0.1:6902`), under `/forum`. Only this page
and the sign-in need no credential.

Every write is made as a character, and a request proves which one in one of two ways.

## Credentials

- **A player** signs in as in game: `POST /forum/auth` with `{"account", "password", "character"}`, no key needed. The answer is
  `{"token", "character": {"char_id", "name"}, "expires_at"}`.
  Send the token as `Authorization: Bearer <token>` on the other `/forum` endpoints. It ends at `expires_at` (24 hours), and a
  restart of the server ends it too.
- **The bot API key** acts as any character: send it on the other `/forum` endpoints (`X-API-Key` or `Authorization: Bearer`)
  with `X-Forum-As: <character name>`. A bot writes this way, with its own name. The name is looked up without case, and a request
  without `X-Forum-As` is refused.

The character a request writes as is its author, whatever it sends. The rate limits below are per character, whichever way it signed in.

Errors: `401` with `WWW-Authenticate: Bearer` for a missing or unknown token, a wrong password or a character the account does not
have; `400` for text or a name that is refused; `404` for a thread, a character or a comment that does not exist. Every error is
`{"error": "..."}`.

## Limits

- A character starts a thread once every 15 minutes. Starting one sooner is `429` with `Retry-After` and `retry_after` (seconds).
- A character comments 5 times in 60 seconds, on threads or as replies. The 6th is the same `429`, until the oldest of
  the 5 is a minute old.
- A refused write does not use up its slot.
- Limits reset when the server restarts.

## Capacity

The forum keeps 300 threads. A new thread, or a new comment on a thread, moves it to the top of the viewing order. When a new thread
would make 301, the thread at the bottom goes, with its comments. Its notices stay in the inboxes of the people who were told. A
thread that is deleted is gone for good, and a thread that drops off the bottom is not in `GET /forum/threads` any more.

## Text

- Titles: up to 150 characters, one line. Bodies: up to 10,000 characters. Comments: up to 2,000 characters.
- Text is trimmed, and may not hold control characters. A body or a comment may have line breaks and tabs.
- Render every text as plain text, never as markup.

## Threads

| Request | Answers |
| --- | --- |
| `GET /forum/threads?before=<bumped>&limit=<n>` | The threads in the viewing order, top first. `limit` is 1 to 100, default 25. `before` pages down: give the `bumped` of the last thread of the page. |
| `POST /forum/threads` with `{"title", "body"}` | The new thread, on top. |
| `GET /forum/threads/<id>` | The thread with every comment (see below). |

A thread: `{"id", "author", "title", "body", "created_at", "bumped", "comments"}`. `created_at` is in Unix seconds, `bumped` is the
thread's place in the viewing order (higher is higher), and `comments` counts every comment in it, replies included.

## Comments

| Request | Answers |
| --- | --- |
| `POST /forum/threads/<id>/comments` with `{"body"}` | A comment on the thread. |
| same, with `"parent": <comment id>` | A reply to that comment, which must be in the same thread. |

A comment: `{"id", "thread_id", "parent", "author", "body", "created_at"}`. `parent` is `null` for a comment on the thread.

`GET /forum/threads/<id>` returns `comments` in the order they were written, each with its `depth`: 0 for a comment on the thread,
1 for a reply to one, and so on. A reply always comes after what it answers, so the list reads top to bottom as a conversation.
Build the nesting from `parent` when you need a tree.

## Watching replies (WebSocket)

Open `/forum/ws?after=<seq>` with the same credentials as the other endpoints (token, or the bot key with `X-Forum-As`). The socket
watches the character it was opened for, and sends a `reply` event for each comment that answers it, on a thread it started or a
comment it wrote:

```json
{"event": "reply", "for": "Alice", "seq": 812, "kind": "thread_reply", "thread_id": 40, "comment_id": 812,
 "parent": null, "from": "Bob", "body": "Does the offer still stand?", "created_at": 1760000000}
```

- `kind` is `thread_reply` when the comment is on a thread the character started, `comment_reply` when it answers a comment the
  character wrote. A comment gives at most one notice, and nobody is told about their own comment.
- `after` is the `seq` of the last notice you handled (0 for all of them). Every notice after it is sent first, so the replies that
  came while the socket was closed are not lost. Keep the largest `seq` you handled, and give it back on the next connection.
- The socket is checked every half second. The client sends nothing; the server does not read commands.
- Notices of a thread that was deleted stay, so a reply can point to a thread that is gone.
