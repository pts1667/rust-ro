# Embedded sled storage

**Date**: 2026-10-04

**Status**: Accepted; supersedes the previous database decision.

The unified server stores persistent state in sled. The `database` crate owns the shared record format, schema version, indexes, and ID sequences. Server repositories and account provisioning use the same database and transaction helpers.

Accounts and character names and slots have explicit unique indexes. Inventory records are grouped by character, with an index mapping inventory IDs to their owner. Purchases and sales update inventory and zeny in one transaction. Card composition updates the equipment and consumes the card in one transaction. Script variables group array entries by scope, owner, and variable name.

New databases import the JSON item and monster assets and the example account seed. Seeding is idempotent and does not reset player progress on subsequent starts. The schema version is checked before reading stored records.

All writes use sled transactions. There are no explicit application flushes, including on shutdown. Sled's default background syncing determines when committed writes become durable on disk. Transactions preserve atomicity but do not remove the interval in which recent writes can be lost during a power failure.

One process owns the database directory. Administrative account tools must run while the server is stopped. Integration tests use isolated temporary databases.
