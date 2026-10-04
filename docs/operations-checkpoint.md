# Sled, Wasmtime, and gameplay checkpoint

This checkpoint preserves the database and scripting replacements and the completed gameplay additions. It does **not** establish full pre-renewal game coverage. The remaining work below is the handoff for the next implementation session.

## Included work

- Persistent accounts, characters, inventories, skills, variables, bonuses, and supported world systems use Sled. PostgreSQL and SQLx are removed from the Rust dependency graph. A fresh database is seeded from repository assets; existing PostgreSQL data is not migrated.
- Inventory consumption, grants, resource changes, persistent script variables, world changes, and shared reward-pool receipts use multi-key/multi-tree transactions. Application code makes no explicit database flush calls.
- Wasmtime executes compiled Rust NPC, item, pet, and event code. The SDK, offline import tools, guest sources, manifests, and compiled bundle are included. An rAthena checkout is an offline reference/import input, not a runtime VM dependency.
- Item targeting and dialogs defer their source transaction until completion and reject replaced sources, insufficient resources, and applicable late map restrictions. Native Teleport pays when its menu opens and preserves the current instance for random teleport.
- Combat additions cover equipment/card effects, conditional weapon bonuses, skill resistance, Misc defense, signed elemental absorption, critical/mastery/refine ordering, incoming shields, reflection, procs, and actual monster classes. Passive range and ASPD rates are carried through calculation and client skill displays.
- Supported status, offensive, support, ground, delayed, and area skills use the existing game/map loops. Scripted unit casts carry actual source IDs, cast-time adjustments, cancellation overrides, monster messages, targeting/ground options, and source snapshots. See the casting limitations below.
- Supported party, guild membership/storage, cart, personal storage, vending, buying-store/search, fame, homunculus, mercenary, and pet operations have persistence and event handling. These are partial implementations of their larger game systems.
- Pet capture and loot use map reservations with transaction-backed completion or rollback. Cargo retains item identity, cards, refine, and damage state. Overflow remains hidden until committed, respects `NoDrop`, and can complete after logout. Pet support/combat and owner experience are configurable.
- Pre-renewal map rules are imported into the repository. Working consumers include player hostility, party/guild protection flags, siege-dependent castle hostility, GvG/BG damage rates, experience/drop restrictions, item-use/branch/teleport/return restrictions, capture/drop rules, and client map properties. Party warps preserve explicit party IDs, source-map filters, scatter radii, and destination instances.
- Ordinary player trading has a tested database transaction and runtime model, with preliminary action guards. Its client workflow is still incomplete.

## Remaining operations

| Area | Remaining work |
| --- | --- |
| Player trading | Request/accept/offer/lock/confirm/cancel packet handling, virtual offer notifications, session allocation, admission producers, timeout/death/map-change/disconnect cancellation, and complete client workflow. The database transaction is groundwork, not a playable trade system. |
| Trade pet ownership | Reconcile pet-egg custody after storage/vending handoffs before connecting trading. The transaction currently requires the cached pet owner and egg-row ID to match the offering inventory. |
| Guilds and siege | Roles and permissions, guild skills/tax, notices/emblems, alliances, castle ownership, siege start/end lifecycle, and objective-specific combat rules. Membership/storage support does not complete these systems. |
| PvP and battlegrounds | Team registration/rosters and rewards, alliance/team hostility, ranking/PK/nightmare drops, duels, and broader lifecycle rules. Map flags and damage scaling are present without the complete PvP/BG systems. |
| Map rules | Authoritative consumers for modeled flags not yet used by gameplay, especially `NoSave`, `NoWarpTo`/memo rules, `SkillDamage`, `SkillDuration`, and cart/store restrictions. Item-host map-flag reads/writes also need staging and transaction-aware validation; NPC-host calls already exist. |
| Skill coverage | Remaining native factories and pre-renewal mechanics beyond the implemented set, including NPC/Soul Linker gaps such as `NPC_MAGICALATTACK` and `SL_STUN`. Ground placement, multi-hit/area timing, support eligibility, and secondary callbacks still need broader primary-reference coverage. |
| Non-player scripted casts | Complex actor-specific lifecycles and callbacks remain unsupported, including Grand Cross/Earthquake variants, Firewall/Meteor/Storm Gust/Vermilion/Water Ball, Palm Strike, Snatch/Back Stab/Final Strike, Splasher, and Tarot/interactive/inventory skills. Unsupported paths are rejected before starting the cast. NPC recipients need canonical status/HP handling and are currently rejected; reused NPC IDs need instance-aware source selection. |
| Monster AI | Full monster skill selection, conditions, cooldowns, events, and coordinated cast behavior. Scripted actor casts provide execution machinery, not the complete monster skill AI. |
| Pet scripts | Nested `petautobonus`/`petautobonus2`/`petautobonus3` import, registration, and owner proc lifecycle; broader pet ground skills beyond the supported operations. |
| Party and social systems | Party booking/family sharing and broader social lifecycle; marriage and complete riding/falcon/mount acquisition/use workflows. Existing script state or option setters do not establish all client producers. |
| NPC scripting | Broader NPC event/startup/background lifecycles, including unattached `OnInit` execution; remaining helper/host commands, `getunitdata`/`setunitdata` bridges, array/global behavior, and coverage of additional NPC programs. Bundled programs and generated metadata are not proof of complete rAthena compatibility. |
| Item transaction coverage | Audit ordered side-effect preflight, foreign-character queries/read sets, reusable-item source identity, and receipt handling for combinations not covered by current regressions. |
| Combat and classes | Broader card/proc rounding and ordering, and class/mode immunity across status, visibility, and knockback paths. Matched pre-renewal monster assets carry their actual class; unmatched legacy rows retain a Boss/Normal mode fallback. |
| Client admission and legacy proxies | The existing `server.accounts` allowlist selects locally handled accounts; other authenticated accounts use the legacy login/char/map proxy path. Proxy listeners still start unconditionally. Broader account admission and disabling or retiring that path remain follow-up work. |
| Verification | Replace meaningful gaps behind existing ignored tests, exercise more client workflows, and validate sustained/concurrent game-loop behavior. Passing the current tests is not a full-game completion claim. |

## Useful implementation entry points

- [Database and inventory transactions](../server/src/repository/script_inventory_repository.rs)
- [Ordinary trade transaction groundwork](../server/src/repository/player_trade_repository.rs) and [runtime model](../server/src/server/model/player_trade.rs)
- [Unit command parsing](../server/src/server/service/script_unit_skill_service.rs), [actor casting](../server/src/server/script/skill_actor_dispatch.rs), and [actor effects](../server/src/server/script/skill_actor_effects.rs)
- [Pet cargo service](../server/src/server/service/script_world_pet_loot.rs) and [cargo repository](../server/src/repository/game_system_pet_loot_repository.rs)
- [Map rules](../server/src/server/model/map_flags.rs), [runtime rule service](../server/src/server/service/map_flag_service.rs), and [party warps](../server/src/server/service/party_warp_service.rs)
- [Wasm architecture](adr/3-wasmtime.md) and [offline import tools](../tools/scripts-import/README.md)

## Build and validation

Run from the repository root with the project's nightly Rust toolchain. Integration tests use temporary Sled databases and require no database service.

```powershell
cargo build --workspace --release
cargo test --workspace --release --features unit_tests
cargo test --package server --bin server --release --features integration_tests
python -m unittest discover -s tools/scripts-import -p test_mapflags.py
cargo run --package tools --bin scripts-build --release
```

For a fresh local run, copy `config.template.json` to the ignored `config.json`, then run `cargo run --package server --bin server --release`. The configured database directory and build outputs are ignored. Include `config/wasm/`, `scripts/` sources and lockfile, both new script crates, import/build tooling, and the new server files in the commit.

Checkpoint validation on 2026-10-04:

| Check | Result |
| --- | --- |
| Release workspace build | Passed |
| Release workspace tests with `unit_tests` | 622 passed, 0 failed, 30 ignored |
| Release server tests with `integration_tests` | 388 passed, 0 failed, 0 ignored |
| Python map-flag importer tests | 3 passed |
| Wasm guest build | Passed; `config/wasm/game_scripts.wasm` rebuilt from the included sources |
| Fresh-database startup | Passed; Sled assets seeded, Wasm loaded, 897 maps loaded, and all three TCP listeners remained active |
| Dependency graph | No PostgreSQL or SQLx packages |
| Whitespace validation | `git diff --check` passed |

The existing ignored tests remain visible and are not counted as passing coverage. Compiler warnings remain. Startup emits one nonfatal warning for the missing warp include `pre-re/warps/other/sign.txt`; restoring that asset/include coverage is follow-up work. The startup check verifies boot and listeners without a real Ragnarok client session. Local verification logs are under the ignored `target/checkpoint-*.log` paths.
