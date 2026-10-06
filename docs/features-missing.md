# Missing features: parity with rathena (PACKETVER 20120307, pre-renewal)

Task list for bringing this fork to feature parity with [rathena](https://github.com/rathena/rathena) built with `PACKETVER=20120307` and `RENEWAL` undefined. It complements [operations-checkpoint.md](operations-checkpoint.md), which tracks what has already been done (most of its own list is complete) and is not repeated here except for the items it still marks open.

Everything below is an unchecked task (`- [ ]`) unless stated otherwise. Tasks are ordered by priority inside each section; the suggested global order is in [Suggested order](#suggested-order).

## How this list was produced

- **Reference:** `rathena/rathena` `master` at commit `d4b8e7b8` (2026-10-06), cloned shallow. rAthena's `pre-re` data and `#ifndef RENEWAL` code paths are the source of truth. Reference paths below are relative to the rathena repository root.
- **Packet scope:** rathena's `src/map/clif_packetdb.hpp` was preprocessed with `PACKETVER=20120307`, `PACKETVER_MAIN_NUM=20120307` and every other client-variant number set to 0. That yields the exact client-to-server packet set (232 parseable id entries mapping to 190 distinct `clif_parse_*` handlers) used in [Appendix A](#appendix-a-client-packets-rathena-parses-that-the-fork-does-not).
- **Fork coverage is measured by static search**, not by running a client: a packet counts as handled when its `Packet*` type or its wire id appears in `server/src` outside tests. This can miss handlers that decode ids through other tables (the world decoder canonicalises some ids) and can give false positives when an id appears for an unrelated reason. Rows that need a human check say **verify**.
- **Data parity** (items, mobs, maps, skills, NPC scripts, commands) was measured by diffing rathena's `db/pre-re/*`, `npc/**`, `conf/**` and `doc/**` against `config/**`, `server/src/server/script/skill_metadata.json` and the `lib/script-sdk` `Function` enum.
- **Not verified with a live client.** Counts are exact for what they measure; "no code reference" is an indicator, not proof that behaviour is absent.
- **Out of scope** features (gated above 20120307 or renewal-only) are listed once in [Section 0](#0-out-of-scope-at-packetver-20120307-pre-renewal) so they are not re-reported.

### Snapshot

| Area | Reference (rathena, pre-re @ 20120307) | Fork |
|---|---|---|
| Client packet handlers | 190 distinct `clif_parse_*` | 113 handled, 77 not (see App. A) |
| Stock NPC definitions | 4,422 `script` + 186 `shop` + 96 shared `function`s | about 550 placements, almost all battleground, castle and custom NPCs |
| Quests (`db/pre-re/quest_db.yml`) | 2,879 | 0 (no quest log) |
| Atcommands (`conf/atcommands.yml`) | 314 | about 21 (plus aliases) |
| Battle options (`conf/battle/*.conf`) | about 570 | about 25 config fields |
| Script commands (`doc/script_commands.txt`) | 674 | 165 typed `Function` variants |
| Skills in the classic range (player, `NPC_*`, guild; IDs 1-1018 and 10000+) | 650 | 627 with metadata, 13 with neither metadata nor code |
| Items (`db/pre-re/item_db_*.yml`) | 6,169 | 5,003 (1,167 missing) |
| Maps (`conf/maps_athena.conf`) | 1,265 | 897 (386 missing, 18 fork-only) |
| Mobs (`db/pre-re/mob_db.yml`) | 1,004 | 1,008 (complete) |
| Status changes (`db/pre-re/status.yml`) | 699 | 729 (complete) |
| Item bonuses (`doc/item_bonus.txt`) | 241 | 207 recognised by name search; none of the 34 not found appears in a stock pre-re item script |

## Suggested order

1. **Playability blockers:** [login/char server gaps](#1-accounts-login-and-character-server), the NPC dialog close packet, [town NPC content](#12-npc-content) that needs no new engine feature (Kafra, healers, tool dealers, inns), refine/repair, [natural gameplay packets](#2-client-packets-not-handled) (emote, whisper, chat room, stop attack, change direction, view equipment).
2. **Engine features that unlock whole script families:** quests, instances (memorial dungeons), mail, auction, chat rooms/waiting rooms, item combos, battle configuration layer, GM groups and permissions, [script SDK families](#13-script-sdk-parity).
3. **Skill gaps:** performances/ensembles, Star Gladiator, Auto Spell, Making Arrow, Potion Pitcher, steal/snatch family, audit of metadata-only skills.
4. **Content and assets:** missing maps, missing items, bulk NPC conversion by category, quest NPCs.
5. **Verification and tech debt:** live-client sweeps, upstream issue leftovers.

---

## 0. Out of scope at PACKETVER 20120307 / pre-renewal

Checked against the `#if PACKETVER` gate in rathena (`src/map/clif.cpp`) or against the absence of the class in `db/pre-re/job_stats.yml` and `db/pre-re/skill_tree.yml`. Do not implement these for this target.

| Feature | Why it is out |
|---|---|
| Third jobs (Rune Knight … Shadow Chaser, `RK_`, `WL_`, `GC_`, `AB_`, `RA_`, `NC_`, `SC_`, `LG_`, `SR_`, `WA_`, `MI_`, `WM_`, `SO_`, `GN_`, `RL_`, `SJ_`, `SP_`), Kagerou/Oboro (`KO_`, `KG_`, `OB_`, `ECL_`), Rebellion, Summoner/Doram (`SU_`), Homunculus S (`MH_`), Elemental (`EL_`), Expanded Super Novice | Not in `db/pre-re/job_stats.yml` or `skill_tree.yml`. `db/pre-re/skill_db.yml` still defines their ids (several hundred entries); ignore them. |
| Achievements | Packets gated at `PACKETVER >= 20141016`/`20141022` |
| RODEX mail | Replaces classic mail from 20140616 (classic mail is in scope) |
| Clan system | `PACKETVER >= 20131223` |
| Roulette, Navigation (`navigateto`), hotkey v2 | `PACKETVER >= 20150513` |
| Equipment switch | `PACKETVER >= 20160525` |
| Bank | `PACKETVER_MAIN_NUM >= 20200916` |
| Barter shop (20181219), Laphine synthesis/upgrade and bank (2020), item enchant UI, enchant grade, reputation, refine UI, captcha, attendance, stylist UI | Packets introduced well after 20120307 (verified for barter, Laphine, bank; the others follow the same pattern) |
| Random item options | Introduced after 20120307 |
| Skill cooldown list packet | `PACKETVER_MAIN_NUM >= 20181212` |
| Renewal-only item bonuses (`bPAtkRate`, `bSMatkRate`, `bResRate`, `bMResRate`, `bAllTraitStats`, `bFixedCastrate`, `bVariableCastrate`, `bWeaponMatkRate`, `bSkillRatio`, `bMaxAPrate`, `bIgnoreResRaceRate`, `bIgnoreMResRaceRate`, trait stats, AP) | `RENEWAL` code paths only |
| Baby exp sharing, `WE_CHEERUP`, `WE_ONEFOREVER`, `WE_CALLALLFAMILY`, `NV_BREAKTHROUGH`, `NV_HELPANGEL`, `NV_TRANSCENDENCE` | Not in pre-renewal rathena (some already noted in the checkpoint) |
| Item Reform, Item Packages, Cash shop sales (`PACKETVER_SUPPORTS_SALES`, ≥ 20131223), official Guild Storage skill (≥ 20131223), Magic Gear fuel (`bNoMadoFuel`) | Later packet versions or Mado-only |

Cash shop itself **is** in scope (see [Section 6](#6-items-crafting-and-economy)); only the sales UI is not.

---

## 1. Accounts, login and character server

Evidence: `server/src/server/request_handler/login.rs` and `char.rs`; reference `src/login/*`, `src/char/*`.

- [ ] **User level and sex come from the account, not random values.** `authenticate` fills `user_level` with `rng.gen::<u32>()` and `sex` with 1. Read both from the account record (`login/account.hpp`).
- [ ] **Account creation** (`_M`/`_F` suffix registration, `login_athena.conf: new_account`, `allowed_regs`, `time_allowed`), password handling, `group_id_to_connect`/`min_group_id_to_connect` login gating.
- [ ] **Account state checks:** ban until time (refuse reasons in `login/loginclif.cpp`), `@ban`/`@unban`/`@charban`, IP ban (`ipban_enable`, `ipban_dynamic_pass_failure_ban*`, `ipban_cleanup_interval`, `use_dnsbl`; `login/ipban.cpp`), last IP/login time, login log (`log_login`, `login/loginlog.cpp`).
- [ ] **Duplicate login handling:** kick the existing session when the same account logs in again (`login/loginchrif.cpp`).
- [ ] **Server list data:** real server name, user count with `usercount_*` thresholds, per-IP subnet mapping (`conf/subnet_athena.conf`), `char_maintenance`/`char_new`/`char_new_display`, `max_connect_user`. The list is hard coded to `127.0.0.1` and "Rust ragnarok".
- [ ] **Character slots and selection list:** `chars_per_account`, `vip_char_increase`, `PacketHcAcceptEnter` content for 20120307 (`char_clif.cpp`), ordering, `char_new`/`gm_allow_group`.
- [ ] **Character creation rules:** name validation (`char_name_min_length`, `char_name_option`, `char_name_letters`, `name_ignoring_case`, `unknown_char_name`), starting items/zeny/point (`start_point_pre`, `start_items_pre`, `start_zeny`; the `_pre` variants are the pre-renewal values), hair/colour checks and the stat-allocation check of the create packet.
- [ ] **Character deletion:** rathena at this version uses the e-mail confirmed delete (`PACKETVER_CHAR_DELETEDATE` is false for 20120307); the fork implements `PacketChDeleteChar4Reserved`. **Verify** which packet the 20120307 client sends and add the missing path (`char_del_level`, `char_del_delay`, `char_del_option`, `char_del_restriction`; party/guild leave on delete, `clear_parties`).
- [ ] **PIN code** (`PACKETVER >= 20110309`, in scope): `pincode_enabled`, `pincode_changetime`, `pincode_maxtry`, `pincode_force`, repeated/sequential rules, PIN state packets in `char_clif.cpp`.
- [ ] **Character rename** (`PACKETVER >= 20111101` path in `chclif_parse_rename`, `char_rename_party`, `char_rename_guild`) and **slot move** (`char_move_enabled`, `char_movetoused`, `char_moves_unlimited`, `chclif_parse_moveCharSlot`).
- [ ] **Character server settings:** `default_map`/`default_map_x`/`default_map_y`, `autosave_time`, `save_log`, `char_checkdb`, `fame_list_*` sizes, `guild_exp_rate`, mail return/delete days (`mail_return_days`, `mail_delete_days`, `mail_retrieve`, `mail_return_empty`).
- [ ] **Map-server hand-off for every account:** `server.enable_legacy_proxy` proxies unknown accounts to an external rathena. Make the built-in login/char path the only path and remove the proxy (also an open item in the checkpoint).
- [ ] **Config default:** `config.template.json` ships `packetver: 20120229`. The framing tables support 20120229 and 20120307; set the default and tests to 20120307 and decide whether 20120229 stays supported.
- [ ] **Message of the day** (`conf/motd.txt`, `@reloadmotd`), welcome message on enter.
- [ ] **Permission groups** (`conf/groups.yml`, `doc/permissions.txt`, `src/map/pc_groups.cpp`): group id per account, permission flags (`can_trade`, `can_party`, `all_skill`, `receive_requests`, `hide_session`, `disable_pvm`, …), command/atcommand lists per group, group id for `getgmlevel`/`getgroupid`. There is no GM or permission model today (`server/src` has no group/permission concept).
- [ ] **Persistence parity for state rathena keeps in the char server:** `sc_data` (persistent status changes across logout), `skill_cooldown`, `mapreg` variables (`$`, `$@`, `#`, `##` script variable scopes), `hotkeys`, `feel/hate` records, `friends`, `bound` items, `memo` points (done), `show_equip`, `disable_call`, per-character `ignore` list. Cross-check each against `src/char/int_*.cpp` and `sql-files/main.sql`.
- [ ] **Multiple map servers / inter-server messages** are not needed (single process by design); document the intentional deviation so it is not re-reported.

## 2. Client packets not handled

Full table with ids and rathena handler names: [Appendix A](#appendix-a-client-packets-rathena-parses-that-the-fork-does-not). Grouped by feature; the feature sections below add the server logic.

### 2.1 Gameplay-critical

- [ ] `CZ_CLOSE_DIALOG` (0x0146, `clif_parse_NpcCloseClicked`): the dialog close button is never handled, so the conversation stays open until `conversation_timeout_secs`. Wire it to the conversation service.
- [ ] `CZ_CANCEL_LOCKON` (0x0118, `clif_parse_StopAttack`): stop auto attack. Verify how the fork stops attacks today.
- [ ] `CZ_CHANGE_DIRECTION` (0x0085/0x0361 family, `clif_parse_ChangeDir`): head and body direction broadcast (`ZC_CHANGE_DIRECTION` to the area).
- [ ] `CZ_REQ_EMOTION` (0x00BF, `clif_parse_Emotion`): player emotes. The existing `emotion` code is for NPCs and mobs, and 0x01A9 (decoded) is the pet performance packet, not this one. Rules from rathena: ignore ids ≥ `ET_MAX`, require Basic Skill level 2 when `basic_skill_check` is on, refuse the mute emote, at most one per second, then broadcast `ZC_EMOTION` (0x00C0) to the area.
- [ ] `CZ_REQNAME_BYGID` (0x0368, `clif_parse_SolveCharName`): name lookup by id (used by party/guild/search windows).
- [ ] `CZ_REQ_DISCONNECT` (0x018A) and `CZ_CLOSE_STORE` (0x0193, `clif_parse_CloseKafra`): **verify**. The fork decodes only the other close/disconnect ids; confirm what the 20120307 client sends when closing the Kafra window and when quitting from the Esc menu.
- [ ] `CZ_REQ_MOVETO_MAP`/GM warp (0x0140, `clif_parse_MapMove`): used by GM commands.
- [ ] `CZ_REQ_PVPPOINT` (0x020F, `clif_parse_PVPInfo`): PvP info window. The rank packet 0x019A is sent; the request is not answered.
- [ ] `CZ_EQUIPWIN_MICROSCOPE` (0x02D6, `clif_parse_ViewPlayerEquip`): view another player's equipment (respects `show_equip`).
- [ ] `CZ_CONFIG` (0x02D8, `clif_parse_configuration`): equipment window visibility, call-permission toggle, pet/homunculus auto-feed.
- [ ] `CZ_LESSEFFECT` (0x021D, `clif_parse_LessEffect`): reduce-effects toggle (persisted flag).
- [ ] `CZ_RESET` (0x0197, `clif_parse_ResetChar`): GM stat/skill reset packet.
- [ ] `CZ_REQ_USER_COUNT` (0x00C1, `clif_parse_HowManyConnections`): `/who`.
- [ ] `CZ_CLIENT_VERSION` (0x044A) and progress bar answer `CZ_PROGRESS` (0x02F1, `clif_parse_progressbar`).
- [ ] `CZ_STANDING_RESURRECTION` (0x0292, `clif_parse_AutoRevive`): Token of Siegfried auto revive.
- [ ] `CZ_SELECT_AUTOSPELL` (0x01CE, `clif_parse_AutoSpell`): Auto Spell menu (see [Section 7](#7-skills)).
- [ ] `CZ_REQ_MAKINGARROW` (0x01AE, `clif_parse_SelectArrow`): arrow crafting menu.
- [ ] `CZ_AGREE_STARPLACE` (0x0254, `clif_parse_FeelSaveOk`): Star Gladiator Sun/Moon/Star place confirmation.
- [ ] `CZ_DORIDORI` (0x01E7) and `CZ_CHOPOKGI` (0x01ED): Novice "Doridori" and Spirit explosion actions.
- [ ] `CZ_REQ_WEAPONREFINE` (0x0222, `clif_parse_WeaponRefine`): Whitesmith weapon refine selection (see [Section 6](#6-items-crafting-and-economy)).
- [ ] `CZ_ACK_STORE_PASSWORD` (0x023B/0x0281, `clif_parse_StoragePassword`): storage password dialogs.
- [ ] `CZ_REQ_ACCOUNTNAME`, `CZ_REQ_STATUS_GM` (0x0213, `clif_parse_Check`) and the other GM packets listed in [Section 14](#14-gm-and-administration).

### 2.2 Social and communication (details in [Section 3](#3-social-and-communication))

Chat rooms (0x00D5, 0x00D9, 0x00DE, 0x00E0, 0x00E2, 0x00E3), whisper and ignore (0x0096, 0x00CF, 0x00D0, 0x00D3), friends (0x0368/0x0369 family, 0x0203, 0x0208), mail (0x023F-0x0248, 0x0273), auction (0x024B-0x0251, 0x025C, 0x025D), broadcast (0x0099, 0x019C), memorial dungeon command (0x02CF), quest activation (0x02B6).

### 2.3 Shop UI

- [ ] `CZ_REQ_CASHSHOP_TAB` (0x0846 family, `clif_parse_CashShopReqTab`) and `CZ_PC_BUY_CASH_POINT_ITEM` (0x0288, `clif_parse_npccashshop_buy`): cash shop (see [Section 6](#6-items-crafting-and-economy)).

## 3. Social and communication

Reference: `src/map/chat.cpp`, `src/map/clif.cpp` (`clif_parse_WisMessage`, `clif_parse_FriendsList*`, `clif_parse_PMIgnore*`), `src/map/channel.cpp`, `src/char/int_party.cpp`.

- [ ] **Chat rooms:** create (0x00D5), enter (0x00D9), change settings (0x00DE), change owner (0x00E0), kick (0x00E2), leave (0x00E3), room list entries in the area (`ZC_ROOM_NEWENTRY`, `ZC_DESTROY_ROOM`, …), password and limit, `chatroom` NPC interaction, `ZC_MEMBER_NEWENTRY/EXIT`, chat inside the room, `checkchatting`, `nochat`-related flags. Also the base for **waiting rooms** used by arena and event scripts (`waitingroom`, `enablewaitingroomevent`, `waitingroom2bg`).
- [ ] **Whispers:** `/w`, `/ex`, `/in`, whisper to NPC (`npc_chat.cpp`), long-distance delivery across maps, error replies (offline, blocked), `PMIgnore`, `PMIgnoreAll`, `PMIgnoreList` (0x00CF, 0x00D0, 0x00D3), whisper log, `@wis`.
- [ ] **Friends list** (0x0369 family): add request, accept/decline (0x0208), remove (0x0203), online/offline notice, list on login, 40-entry cap, `friend` persistence.
- [ ] **Emotion** (see [2.1](#21-gameplay-critical)) and shortcut `/` commands that the client sends as chat text: `/where`, `/who`, `/sit`, `/stand`, `/effect`, `/mineffect`, `/nc`, `/ex`, `/in`, `/exp`, `/memo`, `/organize`, `/leave`, `/invite`, `/hi`, `/guild`, `/doridori`.
- [ ] **Broadcast and announcements:** `@broadcast`, `@kami*`, `@localbroadcast`, GM broadcast packets 0x0099 and 0x019C, `mapannounce`/`areaannounce` colours and `/` flags, NPC `announce` flags (`bc_map`, `bc_area`, `bc_self`, colours).
- [ ] **Channels** (`conf/channels.conf`, `src/map/channel.cpp`, `@channel`, `#<name>` chat): server-side only, no packet gate. The `nomapchannelautojoin` map flag needs this first.
- [ ] **Mail (classic):** open box (0x0246/0x023F), read (0x0241), send (0x0248), attachment add/remove (0x0247/0x0244), delete (0x0243), return (0x0273), new-mail notification, zeny attachments, `mail`/`openmail` script commands, mail NPCs (`npc/other/mail.txt`). Reference: `src/map/mail.cpp`, `src/char/int_mail.cpp`.
- [ ] **Auction:** window, register item, bid, buy-now, search, my-info, cancel (0x024B-0x0251, 0x025C, 0x025D), expiry and mail payout, auction NPC (`npc/other/auction.txt`), `openauction`. Reference: `src/char/int_auction.cpp`, `src/map/clif.cpp` (`clif_Auction_*`).
- [ ] **Party features still open:** party item share rules and even-share bonus check against `battle/party.conf`, `@partyoption`, `@partyrecall`, party exp share levels (`@partysharelvl`), party member map/HP refresh on map change (`clif_party_hp`), `party_create`/`party_destroy`-style script commands.
- [ ] **Guild features still open** beyond the checkpoint: guild storage (`GuildOpenStorage` exists; log, permission and capacity rules per `battle/guild.conf` need checking), `@guildrecall`, `@guildlevelup`, `@breakguild`, guild position permission edge cases, guild alliance limits (`guild_max_alliances` done), guild leave/expulsion message packets, guild member login notifications.

## 4. Quests

Reference: `src/map/quest.cpp`, `src/char/int_quest.cpp`, `db/pre-re/quest_db.yml` (2,879 quests), `doc/script_commands.txt` (`setquest`, `completequest`, `erasequest`, `changequest`, `checkquest`, `questinfo`, `showevent`, `isbegin_quest`).

- [ ] Quest data model and persistence (quest id, state, expiry time, mob hunting counters, target list).
- [ ] Quest log packets for 20120307 (`ZC_ADD_QUEST`, `ZC_DEL_QUEST`, `ZC_UPDATE_MISSION_HUNT`, `ZC_ACTIVE_QUEST` 0x02B6 reply, quest list on login, `ZC_QUEST_NOTIFY_EFFECT` for NPC quest icons).
- [ ] Hunting counters wired into mob death (party share rule per `battle_config`), quest expiry timers, `changequest`.
- [ ] `quest_db.yml` import tool and loader (same pattern as `import_battlegrounds.py`).
- [ ] SDK functions: `SetQuest`, `CompleteQuest`, `EraseQuest`, `ChangeQuest`, `CheckQuest`, `QuestInfo`, `ShowEvent`, `IsBeginQuest`.
- [ ] **Quest-granted skills** (`questskill`, `@questskill`, `IsQuest` flag on skills): enforce that skills flagged `IsQuest` need the quest NPC grant before they can be raised. See [Section 7](#7-skills).
- [ ] Then the quest NPC content: 1,980 reference scripts in `npc/quests/**`, including first-class quests, job quests, skill quests (`npc/quests/skills`), seals, new gears.

## 5. Memorial dungeons and instances

Reference: `src/map/instance.cpp`, `db/pre-re/instance_db.yml` (4 pre-renewal entries), `npc/instances/*.txt` (Endless Tower, Nydhogg's Nest, Sealed Shrine, Orc's Memory; 131 scripts), `conf/battle/instance.conf`.

The fork already has numbered map instances (`MapInstance`, `create_map_instance`) used by battlegrounds, castles and party warps. What is missing is the memorial dungeon layer.

- [ ] Instance model: owner (party/guild), booking, lifetime and idle timers, `instance_db` loader.
- [ ] Script API: `instance_create`, `instance_destroy`, `instance_enter`, `instance_npcname`, `instance_mapname`, `instance_id`, `instance_warpall`, `instance_announce`, `instance_check_party`, `instance_check_guild`, `instance_info`, `instance_live_info`, `getinstancevar`, `setinstancevar`, `instance_list` (all absent from the SDK).
- [ ] Packets: `ZC_MEMORIAL_DUNGEON_INFO/NOTIFY`, `ZC_MEMORIAL_DUNGEON_COMMAND` handler 0x02CF (`clif_parse_MemorialDungeonCommand`), the "booked / created / failed" messages already present in `client_messages.rs`.
- [ ] Per-instance mob/NPC/warp spawn (`duplicate_dynamic`, `OnInstanceInit` is registered but no instance creator calls it for scripts), instance map flag inheritance, return-point on expire, `@reloadinstancedb`.
- [ ] The four instance NPC scripts and their quests.
- [ ] Missing instance maps (`1@*`, `2@*`, `3@*`) are in [Section 11](#11-maps-warps-and-spawns); only those present in the 20120307 client are needed.

## 6. Items, crafting and economy

Reference: `src/map/itemdb.cpp`, `src/map/pc.cpp`, `db/pre-re/item_db_*.yml`, `db/pre-re/item_combos.yml`, `db/pre-re/refine.yml`, `db/pre-re/produce_db.txt`, `db/create_arrow_db.yml`, `db/abra_db.yml`, `db/magicmushroom_db.yml`, `db/item_cash.yml`.

- [ ] **Refining:** refiner NPC flow (Hollgrehenn and the other refiners in `npc/merchants/refine.txt`, 37 scripts, plus `npc/merchants/advanced_refiner.txt`), SDK `getequipisenableref`, `getequiprefinecost`, `getequippercentrefinery`, `successrefitem`, `failedrefitem`, `downrefitem`, `getequipweaponlv`/`getequiparmorlv`, refine rate and bonus tables from `refine.yml`, safe limit and break rules, refine effect packets, `Whitesmith` `WS_WEAPONREFINE` selection (0x0222) with `ZC_ACK_WEAPONREFINING`.
- [ ] **Repair:** `RepairItem` list/selection packets (`ZC_REQ_ITEMREPAIR_LIST`, `CZ_REQ_ITEMREPAIR`), repair NPC, `repair`/`repairall` script commands, `BS_REPAIRWEAPON`, Mado repair kits are out of scope.
- [ ] **Item combos** (`item_combos.yml`, 106 sets): not present anywhere in the fork. Needs a combo index, equip/unequip recalculation and the combined script.
- [ ] **Arrow crafting** (`AC_MAKINGARROW`, `create_arrow_db.yml`) and its selection packet 0x01AE.
- [ ] **Crafting databases:** `produce_db.txt` is partially consumed by `Produce`/`Cooking`; check pharmacy (`AM_PHARMACY`), forging/weapon (`BS_*`), `cooking` and `makerune` (renewal, skip) against the file and add missing recipe types (`CR_SYNTHESISPOTION`, `AM_CP_*`, `WS_CREATECOIN`, `WS_CREATENUGGET`, `ASC_...`).
- [ ] **Abra/Hocus-pocus** (`abra_db.yml`) and **Magic Mushroom** (`magicmushroom_db.yml`) random skill tables.
- [ ] **Cash shop:** `item_cash.yml`, cash point balance (`#CASHPOINTS`, `#KAFRAPOINTS`), buy packet 0x0288, tab request 0x0846, `cashshop` NPC type, `CashShop_Functions.txt` (`npc/other/CashShop_Functions.txt`, loaded by `scripts_main.conf`), `@cash`.
- [ ] **Rental items** (`rentitem*`, `rentalcountitem*`, expiry packets `ZC_CASH_TIME_COUNTER`, `ZC_CASH_ITEM_DELETE`) and `bound` items (`getitembound*`, `itembound`, `countbound`, bound trade rules).
- [ ] **Item use rules:** `enable_items`/`disable_items`, `consumeitem`, `@itemreset`, usage delay between uses of the same item class.
- [ ] **Identify and compose edge cases:** `identifyall`, `successremovecards`/`failedremovecards` (Ancient Cards), `mergeitem` (renewal; skip), unique ids (`getequipuniqueid`).
- [ ] **Missing items:** 1,167 of 6,169 pre-renewal items are absent from `config/items.json`: ids 12000-12999 (170), 13000-13999 (527), 14000-14999 (415), plus 54 above 15000. They are mostly boxes, cash and event consumables. Regenerate `config/items.json` from `db/pre-re/item_db_*.yml` and rerun `tools/scripts-import/import_items.py`, then drop anything the 20120307 client does not know.
- [ ] **Item db flags and rules (verify each is read from `config/items.json`):** trade restrictions, `Stack` limits, `Flags` such as `BuyingStore`, `DeadBranch`, `Container`, `NoConsume`, `DropAnnounce`, `TreasureAnnounce`, `NoUse`, expiry, `Delay`/`Group` usage cooldowns, and `Buy`/`Sell` pricing with Overcharge/Discount.
- [ ] **Shops:** `callshop`, dynamic shops (`npcshopitem`, `npcshopadditem`, `npcshopdelitem`, `npcshopattach`), `shop` with item count limits, `cashshop` item currency, quest-item shops, `setiteminfo`, `setitemscript`.
- [ ] **Storage:** storage password (0x023B/0x0281), `storagecountitem`/`storagedelitem`, cart/guild storage count/delete commands, storage size from `battle/items.conf`.
- [ ] **Vending/buying-store polish:** `autotrade` (no `@autotrade`, no persistent shop), `checkvending`, `vending_*` limits in `battle/items.conf`, vend zeny cap check.
- [ ] **Autoloot** (`@autoloot`, `@autolootitem`): `character_service.rs` has a `// TODO check autoloot` stub with a hard-coded `false`.
- [ ] **Item usage side effects** not yet audited: `item_check_equip`, `use_item` restrictions by map flag (`noitem`), `item_use_interval`, `itemheal` variants, `pet` food/lure items, scroll items that cast skills (`itemskill` partially present).
- [ ] **Equipment edge rules:** `unequip` on map change (`noreturn`, `job_noenter_map.txt`), two-handed/shield rules, `item_noequip.txt`, gender/class restrictions at equip time, headgear view overrides.

## 7. Skills

Reference: `db/pre-re/skill_db.yml`, `db/pre-re/skill_tree.yml`, `db/pre-re/skill_nocast_db.txt`, `src/map/skill.cpp`, `src/map/skills/**` (note: renamed layout; per-skill files in `acolyte/`, `archer/`, `swordman/`, …).

Only player skills in the classic id range (IDs 1-1018, 10000-10015 guild skills) plus `NPC_*` monster skills matter. 650 are in range: 627 have metadata in `server/src/server/script/skill_metadata.json`, **13 have neither metadata nor any code reference**.

- [ ] **No metadata and no code:** `KN_CHARGEATK` (Charge Attack, quest skill), `CR_SHRINK`, `AS_VENOMKNIFE`, `RG_CLOSECONFINE`, `WZ_SIGHTBLASTER`, `SA_ELEMENTWATER`/`SA_ELEMENTGROUND`/`SA_ELEMENTFIRE` (Elemental Change via Create Converter), `BA_PANGVOICE`, `BS_UNFAIRLYTRICK`, `PR_REDEMPTIO`, `MO_KITRANSLATION`, `MO_BALKYOUNG`. All 13 are in the pre-renewal `skill_tree.yml` and are granted through quest handling in `npc/other/Global_Functions.txt`, so they are in scope. (`SA_ELEMENTWIND` and `SA_CREATECON` also lack metadata but have a code reference.)
- [ ] **Performances, ensembles and dances are no-ops.** `SkillType::Performance => {}` in `skill_service.rs`, and the item-skill path returns "needs an interactive handler" for them. Needs the performance ground unit (`BA_*`, `DC_*`, `BD_*`, `CG_*` Marionette/Longing/Hermode/Moonlit), aura radius, movement while performing, SP drain per interval, `BD_ADAPTATION`/`BD_ENCORE`, ensemble partner requirement, "sing/dance interrupt on attack", `@killer`-aware targeting, song/dance bonuses applied through `SC_*`.
- [ ] **Auto Spell (`SA_AUTOSPELL`)** selection packet 0x01CE and proc rules. Proc code exists for card/item auto-spells (`combat_trigger_service.rs`) but not the skill's menu.
- [ ] **Abracadabra (`SA_ABRACADABRA`), Cast Cancel, Free Cast, Volcano/Deluge/Violent Gale, Land Protector**: check each; ground and status code exist only for some. `SA_QUESTION`, `SA_GRAVITY`, `SA_LEVELUP`, `SA_INSTANTDEATH`, `SA_FULLRECOVERY`, `SA_COMA`, `SA_MONOCELL`, `SA_CLASSCHANGE`, `SA_SUMMONMONSTER`, `SA_TAMINGMONSTER`, `SA_DEATH`, `SA_FORTUNE` are GM/Abracadabra-only skills; implement through the Abracadabra table.
- [ ] **Making Arrow (`AC_MAKINGARROW`)**, **Potion Pitcher / Berserk Pitcher (`AM_POTIONPITCHER`, `AM_BERSERKPITCHER`)**, **Slim Pitcher/Synthesis/Acid Demonstration craft menus**, **Steal/Mug/Snatcher/Steal Coin (`TF_STEAL`, `RG_STEALCOIN`, `RG_SNATCHER`, `RG_GANGSTER`, `RG_COMPULSION`, `RG_PLAGIARISM`, `RG_FLAGGRAFFITI`)**: no code reference found.
- [ ] **Star Gladiator:** the whole feel/hate system. `SG_FEEL` (Sun/Moon/Star memory, `FeelSaveOk`, map-based `feel` records), `SG_HATE` (target selection, bonus damage vs hated monster), `SG_SUN/MOON/STAR_WARM/COMFORT/ANGER/BLESS`, `SG_DEVIL`, `SG_FRIEND`, `SG_KNOWLEDGE`, `SG_FUSION`, Star Gladiator daily rank, `resetfeel`/`resethate`/`@feelreset`/`@hatereset`, Sun/Moon/Star rate on `SG_*` mobs.
- [ ] **Soul Linker linked skills (`SL_*`, 31 skills):** `SL_KAIZEL`, `SL_KAAHI`, `SL_KAUPE`, `SL_KAITE`, `SL_SWOO`, `SL_SKE`, `SL_SKA`, class-specific links (`SL_ALCHEMIST`…`SL_NINJA`), `SL_DEATHKNIGHT`, `SL_COLLECTOR`, `SL_GUNNER`, `SL_STAR`. Only Estin/Estun/Esma are documented as done.
- [ ] **Taekwon:** `TK_READYSTORM/DOWN/TURN/COUNTER` combo states, `TK_STORMKICK/DOWNKICK/TURNKICK/COUNTER`, `TK_DODGE`, `TK_JUMPKICK`, `TK_HPTIME/SPTIME` (Union of Sun/Moon), `TK_POWER`, `TK_SEVENWIND`. (`TK_RUN`, `TK_MISSION` and the taekwon rank list have code references; verify them rather than reimplement.)
- [ ] **Gunslinger (`GS_*`, 22):** `GS_TRIPLEACTION`, `GS_BULLSEYE`, `GS_MADNESSCANCEL`, `GS_ADJUSTMENT`, `GS_INCREASING`, `GS_MAGICALBULLET`, `GS_CRACKER`, `GS_DISARM`, `GS_PIERCINGSHOT`, `GS_DESPERADO`, `GS_GATLINGFEVER`, `GS_DUST`, `GS_FULLBUSTER`, `GS_GROUNDDRIFT`, `GS_FLING`, coin/spirit-sphere handling, ammo type rules.
- [ ] **Ninja (`NJ_*`, 23):** only metadata, check `NJ_KASUMIKIRI`, `NJ_UTSUSEMI`, `NJ_BUNSINJYUTSU`, `NJ_KIRIKAGE`, `NJ_SHADOWJUMP`, `NJ_NINPOU` mastery, `NJ_TOBIDOUGU`, element scrolls (`NJ_KAENSIN` and the rest), Huuma and Kunai handling.
- [ ] **Monk/Champion combos and spirit spheres:** `MO_TRIPLEATTACK`, `MO_CHAINCOMBO`, `MO_COMBOFINISH`, `MO_BLADESTOP`, `MO_STEELBODY`, `MO_ABSORBSPIRITS`, `MO_SPIRITSRECOVERY`, `CH_TIGERFIST`, `CH_CHAINCRUSH`; `addspiritball`/`delspiritball`/`countspiritball` script commands (absent from SDK).
- [ ] **Metadata-only skills with no bespoke code reference (316 total).** Each may rely on the generic metadata path or may be silently incomplete; audit each against `src/map/skills/**` and the skill's `Requires` block. The class-grouped list is in [Appendix C](#appendix-c-skills-with-metadata-but-no-bespoke-code-reference).
- [ ] **Skill requirements and flags to enforce:** `skill_nocast_db.txt` (map-flag-dependent bans such as Endure in PvP/GvG), `NoNearNPC`, `AlterRangeVulture/SnakeEye/Magnum…`, `AllowReproduce`, `CopyFlags` (Intimidate/Plagiarism), `IsQuest`, `AlterRange*` for Vulture's Eye and Snake Eyes on ground skills, `CastDefenseReduction`, `AfterCastWalkDelay`, `Knockback` on every skill that defines it.
- [ ] **Cast mechanics:** cast cancel on damage (`CastCancel`), cast time reductions (DEX, `bCastrate`, Suffragium, Poem of Bragi), `bNoCastCancel`, `bNoWalkDelay`, instant cast (`bonus bUseSPrate`/`bSkillUseSP`), SP consumption modifiers.
- [ ] **Open items from the checkpoint:** actor Back Stab/Raid/Splasher/Issen callbacks, non-player Remove Trap/Spring Trap, multi-hit area timing and support eligibility, pet ground skills, `Water` riding state, guild skill cast times (instant today), Item Emergency Call reuse delay, GvG `vs_traps_bctall` target override, Land Protector placement refusal.
- [ ] **Skill tree and quest-skill access:** `skill_tree.yml` inherit/max rules per class, skill point rules per job and baby/rebirth/high-class level caps (job level 70 caps etc.), `@skilltree`, `@allskill`, `@skillpoint`, `@lostskill`.

## 8. Battle system and server configuration

Reference: `conf/battle/*.conf` (about 570 options: `battle.conf` 42, `client.conf` 41, `drops.conf` 63, `exp.conf` 27, `gm.conf` 9, `guild.conf` 17, `homunc.conf` 18, `instance.conf` 4, `items.conf` 27, `misc.conf` 46, `monster.conf` 59, `party.conf` 10, `pet.conf` 25, `player.conf` 82, `skill.conf` 83, `status.conf` 6, `battleground.conf` 8), `src/map/battle.cpp` (`battle_config_read`), `conf/map_athena.conf`, `conf/char_athena.conf`, `conf/inter_athena.conf`.

The fork exposes about 25 fields under `game.*` (rates, `max_base_level`, `max_inventory`, `mob_density`, GvG/BG damage rates, drop lock times). Almost every rathena balance and rule option is hard coded or absent.

- [ ] **Introduce a battle configuration layer** in `lib/configuration` (typed, with rathena names and defaults) and a loader for `conf/battle/*.conf` or an equivalent JSON, so rules stop being constants. Start with the groups below.
- [ ] `exp.conf`: base/job/quest/mvp exp rates, `exp_calc_type`, `death_penalty_*` (partly done), `exp_bonus_attacker`, `exp_bonus_max_attacker`, `max_exp_gain_rate`, `base_exp_rate` per-map.
- [ ] `drops.conf`: item drop rates by type (`item_rate_*` with `_min`/`_max`), `drops_by_luk`, `drops_by_luk2`, `drop_rate0item`, `rare_drop_announce`, `item_drop_*`/`mob_item_ratio.yml`, `loot_range` and `autoloot` options.
- [ ] `skill.conf`: `skill_delay_attack_enable`, `skill_min_damage`, `skill_out_range_consume`, `skillfree`, `player_skill_partner_check`, `gvg_*` skill rules, `pk_*` skill rules, `castrate_dex_scale`, `vcast_stat_scale`, `no_skill_delay`, `skill_reiteration`, `skill_nofootset`, `skill_wall_check`, `traps_setting`, `finger_offensive_type`, `defunit_not_enemy`, `skill_log`.
- [ ] `player.conf`: HP/SP regen options (`natural_healhp_interval`, `natural_heal_skill_interval`, `natural_heal_weight_rate`, `max_hp`/`max_sp`/`max_parameter` caps, `max_aspd`, `max_walk_speed`, `max_cart_weight`, `max_def`, `min_chat_delay`, `idle_no_share`), `restart_hp_rate`/`restart_sp_rate` (done), `zeny` caps, `mvp_tomb_enabled`/`mvp_tomb_delay`.
- [ ] `monster.conf`: `monster_ai` (aggressive/assist/chase flags), `mob_count_rate`, `mob_skill_rate`, `monster_damage_delay_rate`, `monster_hp_bars_info`, `mob_warp`, `mob_spawn_delay`, `mvp_hp_rate`, `monster_loot_search_type`, `slaves_inherit_*`, `override_mob_names`, `show_mob_info`.
- [ ] `status.conf`/`items.conf`: status duration resistance rules (`*_def_cap`), refine rules (`weapon_produce_rate`, `produce_item_name_input`), `cardillustration`, `item_auto_get`, `item_first_get_time`, `item_second_get_time`, `item_third_get_time` (loot priority timings; fork has `mob_dropped_item_locked_to_owner_duration_in_secs`).
- [ ] `guild.conf`, `party.conf`, `pet.conf`, `homunc.conf`, `gm.conf`, `client.conf` (`display_*`), `misc.conf` (`pk_mode`, `duel_*`, `mail_*`, `chat_warpportal`, `warp_point_debug`), `instance.conf`, `battleground.conf`.
- [ ] `conf/map_athena.conf` server options (autosave, `motd`, GM/ban rules, logging), `conf/log_athena.conf` (item/chat/atcommand logging).
- [ ] **Natural regen details** (`natural_*`): HP/SP regen runs in the game loop; verify sit/stand, skill regen (`SM_RECOVERY`, `MG_SRECOVERY`, `MO_SPIRITSRECOVERY`), weight and overweight 50%/90% rules and `Regeneration` status.
- [ ] **Movement speed stacking (upstream #65):** Increase AGI vs item `bSpeedRate`, speed potions overriding other bonuses, caps. Compare with `status_calc_speed` in `src/map/status.cpp`.
- [ ] **Status snapshot (upstream #49)** and **stats calc integration (upstream #44):** confirm the status pipeline matches `status_calc_pc` for every `bonus` family.
- [ ] **Battle edge rules to check against `battle.cpp`:** crit vs flee, size/element fix tables (`size_fix.yml`, `attr_fix.yml`), `equip_natural_break_rate`, `left_cardfix_to_right`, `player_skill_reflect_*`, `magic_defense_type`, `vit_penalty_*`, `agi_penalty_*`, `weapon_defense_type`, `cardillustration`, `attack_attr_none`, `pet_*`, `homunculus_*`, `battle_config.mobs_level_up` (aggressive mob level scaling).
- [ ] **Level penalty:** the main `db/level_penalty.yml` is data for exp/drop penalties against higher or lower-level mobs; confirm it is applied the same way as `battle_calc_..._penalty` (pre-re values exist in the shared file).

## 9. Monsters, drops and MVP

Reference: `src/map/mob.cpp`, `db/pre-re/mob_db.yml`, `db/pre-re/mob_skill_db.txt`, `db/mob_chat_db.yml`, `db/mob_item_ratio.yml`, `db/map_drops.yml`, `db/mob_summon.yml` (pre-re copy in `db/pre-re/mob_summon.yml`).

All 1,004 monsters and the 5,494-entry skill database are imported. Remaining:

- [ ] **MVP rewards leftovers (checkpoint):** floor drop when the inventory cannot take the prize, rare-drop broadcast (`rare_drop_announce`), attacker-count exp bonus.
- [ ] **MVP tombs** (`notomb` flag, `mvp_tomb_enabled`): spawn tomb NPC on MVP death with killer name and time.
- [ ] **Mob chat** (`mob_chat_db.yml`, `npc_talk`): the id travels as `message_id`; verify the text table is loaded and sent.
- [ ] **Mob display** (`mob_avail`, `Disguise` in `mob_db`): clients that cannot show a monster sprite use the display class; `disguise`/`@disguise` for players.
- [ ] **Monster HP bar** (`hidemobhpbar`, `show_mob_info`, `monster_hp_bars_info`): no `ZC_HP_INFO`/`ZC_NOTIFY_MONSTER_HP` for monster HP, and `damage sprite` flags.
- [ ] **Slave monsters and summons:** `slaves_inherit_mode/speed`, `mob_summon.yml` (Summon Monster by random), `Dead Branch/Bloody Branch/Poring Box` item groups (data exists), `SA_SUMMONMONSTER`, `AL_SUMMON`.
- [ ] **Monster behaviour options:** `mob mode` flags for assist/looter/chase/target-weak already partly done; verify `MD_*` against `mob_db` for all 1,004, boss/mvp status immunity list (`MD_STATUS_IMMUNE`), `ai_type` (`monster_ai` bits) and mob skill state `angry`/`follow`.
- [ ] **Dynamic mob commands:** `areamonster`, `areamobuseskill`, `killmonsterall`, `clone`, `summon`, `addmonsterdrop`/`delmonsterdrop`, `mob_setidleevent`, `getmonsterinfo`, `getmobdrops`, `getrandmobid`, `monster` with custom drops/AI, `@monster*`, `@killmonster*`, `@mobinfo`, `@mobsearch`, `@whodrops`.
- [ ] **Spawn reconciliation:** the loaded spawn lines (3,393 by `monster`/`boss_monster` match in `config/npc`) differ from the reference count in `npc/pre-re/mobs`, `npc/mobs`, `npc/quests`, `npc/jobs` (2,638 matched lines). The regexes differ, so first confirm which monsters in `scripts_monsters.conf` are missing or extra, then align.
- [ ] **Pet catalogue is empty:** checkpoint note: "catalog is empty until an override supplies programs". Import `db/pre-re/pet_db.yml` (57 pets: loyalty, hunger, accessories, auto bonus scripts, `EquipScript`, `SupportScript`, `AttackRate`, `DefendRate`, `ChangeTargetRate`, `Evolution` is renewal).

## 10. Companions (pets, homunculus, mercenaries)

- [ ] **Pets:** full import of `pet_db.yml`; egg creation/hatch/return, hunger/intimacy timers, `pet_rename`, pet talk (`@pettalk`), pet performance packet (0x01A9 is decoded; it is `CZ_PET_ACT`, not player emote), catching rates, `@petfriendly`/`@pethungry`, `petloot` limits, `autofeed` via the 0x02D8 config packet.
- [ ] **Homunculus:** pre-re `homunculus_db.yml` (Lif/Amistr/Filir/Vanilmirth skills and evolution), growth from `exp_homun.yml`, `homshuffle`, `hommutate` (S-class is out of scope; `hommutate` is Homunculus S so skip), `homfriendly`/`homhungry`, `checkhomcall`, `gethominfo`, `addhomintimacy`, `@homlevel` etc. Several are checked off in the checkpoint; re-audit against `src/map/homunculus.cpp`.
- [ ] **Mercenaries:** pre-re `mercenary_db.yml` is header-only, so rathena reads the shared `db/mercenary_db.yml` (44 entries); mercenary rental NPCs (`npc/pre-re/other/mercenary_rent.txt`, 3 scripts) and `mercenary_*` commands (`mercenary_get_calls`, `mercenary_set_calls`, `mercenary_get_faith`, `mercenary_set_faith`, `mercenary_delete`, `mercenary_sc_start`, `getmercinfo`) are absent. The `MS_*`, `MA_*`, `ML_*`, `MER_*` skills (62 skill entries) need a status/combat path.

## 11. Maps, warps and spawns

- [ ] **386 reference maps have no map cache** in `config/maps/pre-re` (897 loaded vs 1,265 in `conf/maps_athena.conf`; 18 fork-only: `force_map1-3`, `job_hunter`, `job_knight`, `job_priest`, `job_wizard`, `mjolnir_04_1`, `moc_fild04/05/06/08/09/10/14/15`, `old_moc`, `siege_test`). Generate the missing `.mcache` files (`lib/map-cache`) from the 20120307 GRF; maps that were not in that client can be skipped. Missing categories that matter for scripts already in rathena: PvP rooms and arenas (`pvp_n_*`, `pvp_y_*`, `pvp_c_room`, `pvp_n_room`, `pvp_y_room`, `new_1-2…new_5-4`), turbo track (`turbo_*`, `p_track02`), `guild_vs*`/`g_room*` guild arenas, `06guild_*`, `nguild_*`, Guild dungeons (`gld_dun01_2`…), castles (`arug_cas04/05`, `schg_cas04/05`, `schg_que01`), quest maps (`que_*`, `job3_*` skip), BG second arenas (`bat_a02`, `bat_b02`, `bat_c02`, `bat_c03`), Instance maps (`1@*`, `2@*`, `3@*`), `ordeal_*`, `force_*`, `sword_*`, `hero_*`, `iz_in_*`, `jor_*`, `ra_pol01`, `ygg_*`, `uknw_ruin*`, `moc_akhet`, `z_agit`.
- [ ] **Reconcile warp counts:** rathena's shared plus pre-re warp files hold about 3,000 `warp` lines (`npc/warps/**`, `npc/pre-re/warps/**`); the fork loads 2,765. Find which warps are dropped (likely missing destination maps) and restore them once the maps exist.
- [ ] **Warp-related scripts, not just `warp` lines:** the warp directories also hold 79 `script` definitions (portals with conditions, airship stops). See [Section 12](#12-npc-content).
- [ ] **Map flags:** `config/wasm/map_flags.json` carries the 3,595 reference entries. Flags with no feature behind them are listed in the checkpoint: `nochat`, `allowks`, `autotrade`, `notomb`, `hidemobhpbar`, `nobank`, `nodynamicnpc`, `nomapchannelautojoin`, `specialpopup`.
- [ ] **Map zones** (`conf/map_zones` equivalent), `mapflag zone`: rathena applies per-zone skill/item bans and `mapflag nosave`-style rules from `db/pre-re/skill_nocast_db.txt` and zone definitions; the fork has no zone concept.
- [ ] **Cell types and map data:** water/shootable/walkable are used; verify `cell_chkwall`, `cell_nochat`, `cell_novending`, `cell_icewall`, landprotector, basilica, `checkcell`, `setwall`/`delwall` script commands.
- [ ] **Weather and day/night:** weather flags and `nightenabled` exist; add `day`/`night` commands and the night/day timer (`night_duration`, `day_duration`) and `ZC_NOTIFY_EFFECT` for night mode.

## 12. NPC content

Reference: `npc/scripts_main.conf` chain (pre-re), 527 files. The fork runs compiled Rust NPCs (see [adr/3-wasmtime.md](adr/3-wasmtime.md)); rAthena script is only an import input. Converted today: battleground arenas (Flavius, KvM, Tierra, `bg_common`), castle stewards, Kafras and levers, wedding NPCs, Peco Peco and falcon breeders, a custom warper, job master, stylist and test NPCs, weapon/armor shop templates.

Reference counts of `script`/`shop`/`cashshop`/`duplicate` definitions per category, with the engine features each category needs:

| Category (`npc/…`) | Count | Needs first |
|---|---|---|
| `cities` | 633 | quests, kafra, mail, pets, healers: mostly dialogue and shops; largest single blocks are `lighthalzen.txt` (136), `veins.txt` (46), `einbroch.txt` (38) |
| `quests` | 1,980 | quest log ([Section 4](#4-quests)), items, SDK commands |
| `jobs` | 316 | job change quests, `jobchange`, `changebase`, class tests, second-class and rebirth quests, baby/Taekwon/SG/SL/ninja/gunslinger quests |
| `merchants` | 269 | refine ([Section 6](#6-items-crafting-and-economy)), socket enchant, dye makers, inns (`inn.txt`), renters, pet food |
| `other` | 556 | arenas (`arena/*`, 284), turbo track (49), monster race (34), museum (62), poring war (27), bingo (13), bulletin boards (29), books, PvP, auction, mail, fortune, gambling, gympass |
| `battleground` | 252 | done except the shared Kafra and Repairman (see checkpoint) |
| `guild` and `guild2` | 217 | castle scripts exist for 20 castles; `agit_main*`, `guild_flags.txt`, castle controllers for all 20 castle maps, `arug_cas04/05` and `schg_cas*` maps need the missing castle map caches |
| `instances` | 131 | [Section 5](#5-memorial-dungeons-and-instances) |
| `kafras` | 63 | Kafra services (storage, save, teleport, cart rental; `npc/kafras/kafras.txt` 42, `dts_warper.txt` 17) |
| `airports` | 38 | airship NPCs and `airships.txt` (24), needs timed map transitions |
| `events` | 44 | guild-war event stages (`gdevent_*`), festivals |
| `guides` | 28 | town guide NPCs with `viewpointmap`/`viewpoint` marks |
| `warps` (scripts) | 79 | conditional portals |
| `mapflag` | 3,595 | already imported as `map_flags.json` |

- [ ] **Kafra in towns** (`npc/kafras`, 63 scripts): storage, save point, teleport service (price and free-Kafra discounts), cart rental, Kafra points, `OnInit` menus. Upstream issue #5 asks for the 13 native functions (`cutin`, `percentheal`, `viewpoint`, `implode`, `countitem`, `delitem`, `basicskillcheck`, `getskilllv`, `getcharid`, `getguildinfo`, `savepoint`, `emotion`, `logmes`); `CountItem`, `DelItem`, `GetSkillLv`, `GetCharacterId`, `GetGuildInfo`, `SavePoint` and `Cutin`/`PercentHeal` exist as `Function` variants; `emotion`, `viewpoint`, `basicskillcheck`, `logmes` do not (`implode` is plain Rust). The Kafra staff NPC in castles is inert unless hired.
- [ ] **Healers, tool dealers, inns, general shops** (about 300 definitions in `cities/*` and `merchants/*`): mostly shop/menu scripts needing nothing new. Good first conversions.
- [ ] **Job change flow** (`npc/jobs`, upstream #60): first-class quests (`npc/quests/first_class`), second-class tests, rebirth , Valkyrie, Taekwon, Star Gladiator, Soul Linker, Gunslinger, Ninja quests. A custom Job Master exists for testing only.
- [ ] **Bulk conversion tooling:** `import_npcs.py` only recognises a fixed set of migrated NPCs and "rejects unrecognised NPC behavior". Either extend it into a real rAthena-script-to-Rust translator for the dialogue/shop subset (`mes`, `next`, `select`, `if/else`, `set`, `getitem`, `countitem`, `warp`, `callfunc`) or hand-port by category. Decide once and document in `tools/scripts-import/README.md`.
- [ ] **Dynamic NPC creation and lifecycle:** `duplicate`, `duplicate_dynamic`, `unloadnpc`, `loadnpc`, `@loadnpc`, `@unloadnpc`, `cloakonnpc`/`cloakoffnpc`, `setnpcdisplay`, `npctalk`, `chatmes`, `npcspeed`, `npcwalkto`, `npcstop`, `movenpc`, NPC `OnInit` order, NPC labels with `OnClock*`/`OnMinute*`/`OnHour*`/`OnDay*`/`OnSun`… clock events (the fork has `OnTimer`, `OnInit`, `OnAgit*`, `OnPCLoadMapEvent`, `OnTouch`).
- [ ] **NPC event labels missing from the runtime:** `OnClock####`, `OnMinute##`, `OnHour##`, `OnDay####`, `OnPCDieEvent`, `OnPCKillEvent`, `OnPCLogoutEvent`, `OnPCLoginEvent`, `OnPCBaseLvUpEvent`, `OnPCJobLvUpEvent`, `OnNPCKillEvent`, `OnPCStatCalcEvent`, `OnTalkClick`, `OnMyMobDead`, `OnBuyItem`/`OnSellItem`, `OnTouchNPC`, `OnTouch_` clicking fallback (`touch` currently fires on entry only and does not fall back to a click). Check each against `src/map/npc.cpp` (`script_event`).
- [ ] **Global variable scopes and arrays:** `$`, `$@`, `#`, `##`, `.`, `.@`, `'`, `@`, with arrays and `getarraysize`, persisted `mapreg`. The Wasm host gives typed storage; confirm each scope has a Rust equivalent, especially account (`#`, `##`) and permanent global (`$`) variables and `getd`/`setd`.
- [ ] **Remaining script commands** are in [Section 13](#13-script-sdk-parity).

## 13. Script SDK parity

`lib/script-sdk/src/lib.rs` defines 165 `Function` variants; `doc/script_commands.txt` documents 674 commands. Many rathena commands collapse into one typed call here (`getitem2/3/4` into richer `GetItem` arguments, string/array/math helpers into Rust), so name matching under-counts. The families below have **no equivalent** in the SDK after reading the whole `Function` list; implement as typed functions, grouped by the feature they depend on.

- [ ] **Quest:** `setquest`, `completequest`, `erasequest`, `changequest`, `checkquest`, `questinfo`, `showevent`, `isbegin_quest`, `open_quest_ui`.
- [ ] **Instance:** `instance_*` family (14 commands), `getinstancevar`, `setinstancevar`.
- [ ] **Party/guild management:** `party_create`, `party_destroy`, `party_addmember`, `party_delmember`, `party_changeleader`, `party_changeoption`, `getpartyname`, `getpartymember`, `getpartyleader`, `is_party_leader`, `getguildname`, `getguildmember`, `getguildmaster`, `getguildmasterid`, `is_guild_leader`, `getguildalliance`, `guildchangegm`, `guildgetexp`, `guildskill`, `guild_has_permission`, `requestguildinfo`, `getcastlename`, `getgdskilllv`, `flagemblem`, `guardian`, `guardianinfo`, `maprespawnguildid`, `agitstart2/3`, `agitend2/3`, `gvgon3`/`gvgoff3`, `getmapguildusers`.
- [ ] **Item queries and mutation:** `getitem2/3/4`, `getitembound*`, `getnameditem`, `makeitem*`, `rentitem*`, `delitem2/3/4`, `delitemidx`, `countitem2/3/4`, `cartcountitem*`, `cartdelitem*`, `storagecountitem*`, `storagedelitem*`, `guildstoragecountitem*`, `guildstoragedelitem*`, `rentalcountitem*`, `countbound`, `getequipname`, `getequipisequiped`, `getequipcardcnt`, `getequipcardid`, `getequiprefinecost`, `getequipweaponlv`, `getequiparmorlv`, `getequipisenableref`, `getequippercentrefinery`, `getequipuniqueid`, `getbrokenid`, `getinventorylist`, `getitemslots`, `getitempos`, `getareadropitem`, `searchitem`, `cardscnt`, `isequippedcnt`, `checkequipedcard`, `successrefitem`, `failedrefitem`, `downrefitem`, `successremovecards`, `failedremovecards`, `repair`, `repairall`, `unequip`, `delequip`, `breakequip`, `equip`, `autoequip`, `clearitem`, `identifyall`, `enable_items`, `disable_items`, `consumeitem`, `groupranditem`, `setiteminfo`, `setitemscript`.
- [ ] **Character state:** `changebase`, `classchange`, `changesex`, `changecharsex`, `changelook`, `setoption`, `checkoption*`, `setdragon`/`checkdragon`, `setmadogear` (out of scope), `setmounting` (done as `SetRiding`), `checkwug`, `checkvending`, `checkchatting`, `checkidle*`, `nude`, `sit`, `stand`, `disguise`/`undisguise`, `transform`, `pcfollow`/`pcstopfollow`, `pcblockmove`, `pcblockskill`, `unitblockmove`, `unitblockskill`, `statusup`/`statusup2`, `resetlvl` (exists as `ResetLevel`), `resetstatus`, `resetfeel`, `resethate`, `plagiarizeskill`/`plagiarizeskillreset`, `addtoskill`, `getexp2`, `getbaseexp_ratio`, `getjobexp_ratio`, `needed_status_point`, `jobcanentermap`, `getskilllist`, `skillpointcount`, `recalculatestat`, `healap` (renewal; skip), `recovery`, `bonus_script`, `bonus_script_clear`, `sc_start2`, `sc_end_class`, `getstatus`, `npcskilleffect`, `specialeffect2`, `removespecialeffect*`, `addspiritball`, `delspiritball`, `countspiritball`, `getcharip`, `getunits`/`getmapunits`/`getareaunits`, `getmapxy`, `getusers`, `getmapusers`, `getareausers`, `gettimetick`, `gettimestr`, `convertpcinfo`.
- [ ] **Monsters and units:** `areamonster`, `areamobuseskill`, `killmonsterall`, `clone`, `summon`, `addmonsterdrop`, `delmonsterdrop`, `mob_setidleevent`, `getmonsterinfo`, `getmobdrops`, `getrandmobid`, `unitwalk`, `unitwalkto`, `unitattack`, `unitkill`, `unitwarp`, `unitstopattack`, `unitstopwalk`, `unittalk`, `unitskilluseid`, `unitskillusepos` (partly as `UnitSkill*`), `unitexists`, `getunittype`, `getunitname`, `setunitname`, `setunittitle`, `getunittitle`.
- [ ] **NPC and map:** `hideonnpc`/`hideoffnpc` (partly as `EnableNpc`/`DisableNpc`), `cloakonnpc*`, `isnpccloaked`, `unloadnpc`, `duplicate*`, `cmdothernpc`, `npctalk`, `chatmes`, `setnpcdisplay`, `npcspeed`, `npcwalkto`, `npcstop`, `movenpc`, `waitingroom*` family, `enablearena`, `disablearena`, `mapannounce`, `areaannounce`, `cleanarea`, `cleanmap`, `checkcell`, `setwall`, `delwall`, `checkwall`, `getfreecell`, `callshop`, `npcshop*`, `sleep`, `sleep2`, `awake`, `progressbar`, `progressbar_npc`, `emotion`, `misceffect`, `soundeffect`, `soundeffectall`, `playBGM`, `playBGMall`, `viewpoint`, `viewpointmap`, `showscript`, `readbook`, `mapid2name`, `mapname2id`, `strnpcinfo`, `getvariableofnpc`, `isday`, `isnight`, `day`, `night`, `setbattleflag`.
- [ ] **Pet/homunculus/mercenary:** `bpet`, `catchpet`, `makepet`, `morphembryo`, `checkhomcall`, `gethominfo`, `homshuffle`, `addhomintimacy`, `mercenary_*` (6), `getmercinfo`, `setpcblock`, `getpcblock`.
- [ ] **Admin and misc:** `atcommand`, `charcommand`, `bindatcmd`, `unbindatcmd`, `useatcmd`, `enable_command`, `disable_command`, `permission_check/add/remove`, `getgmlevel`, `getgroupid`, `debugmes`, `errormes`, `logmes`, `globalmes`, `itemlink`, `mesitemlink`, `mesitemicon`, `meshyperlink`, `mesemotion`, `channel_*` (11), `query_sql`/`query_logsql`/`escape_sql` (no SQL backend; decide whether to drop), `open_roulette`, `openbank`, `openmail`, `mail`, `openauction`, `openstylist`, `refineui`, `laphine_*`, `enchantgradeui`, `reputationui`, `achievement*` (out of scope ones can be dropped).
- [ ] **Compile-time ergonomics:** a `Function` documentation index mapping each rathena command to its Rust equivalent (or to "not applicable") would stop this list from needing recomputation.

## 14. GM and administration

Reference: `conf/atcommands.yml` (314 commands), `conf/groups.yml` (Player, Super Player, Support, Script Manager, Event Manager, VIP, Law Enforcement, Admin …), `src/map/atcommand.cpp`, `src/map/pc_groups.cpp`, `src/map/clif.cpp` (`clif_parse_GM*`).

- [ ] **Atcommands:** about 21 are implemented (`go`, `warp`/`rura`/`warpto`, `item`, `inspect`, `blvl`/`jlvl` families, `job`, `rate`, `reload script`, `resetskills`, `resetstats`, `speed`, `heal`, duel family, `killer`/`pk`). The permission model in Section 1 must exist first so that commands can be restricted. Full list in [Appendix B](#appendix-b-rathena-atcommands). Highest value groups: character (`str`/`agi`/…, `stat_all`, `statuspoint`, `skillpoint`, `zeny`, `heal`, `alive`, `resurrect`, `kill`, `die`, `doom`, `option`, `mount_peco`, `hide`, `jump`, `jumpto`, `recall`, `mapmove`, `warp` variants, `save`, `memo`), items (`item2`, `itemreset`, `storage`, `guildstorage`, `produce`, `refine`, `repairall`, `delitem`, `dropall`, `storeall`), monsters (`monster`, `killmonster`, `mobinfo`, `mobsearch`, `whodrops`, `clone`), world (`mapinfo`, `mapflag`, `day`/`night`, `skillon/off`, `pvpon/off`, `gvgon/off`, `loadnpc`, `unloadnpc`, `reloadnpcfile`), social (`wis`, `broadcast`, `kami*`, `mute`, `jail`, `ban`, `kick`, `who*`, `users`, `whomap*`), pets/homunculi (`hatch`, `makeegg`, `pet*`, `hom*`), settings (`autoloot*`, `showexp`, `showzeny`, `showdelay`, `noask`, `autotrade`, `allowks`, `monsterignore`, `commands`, `help`, `refresh`, `refreshall`).
- [ ] **Upstream #20:** `autoloot`, `showexp`, `rates`, `item`, `blvl`, `jlvl`, `job`, `warp`, `go`. Last five exist; `autoloot` and `showexp` do not.
- [ ] **`@refresh` / reload of warps and mobs (upstream #39):** lock map instances, remove map items, reload spawns and warps. `@reload script` exists; the rest of the `@reload*` family (`reloadmobdb`, `reloaditemdb`, `reloadskilldb`, `reloadstatusdb`, `reloadquestdb`, `reloadinstancedb`, `reloadbattleconf`, `reloadmotd`, `reloadatcommand`, `reloadpcdb`, `reloadnpcfile`, `reloadscript`) does not.
- [ ] **Server-side GM packets:** `CZ_DISCONNECT_CHARACTER` (0x00CC kick), `CZ_DISCONNECT_ALL_CHARACTER` (0x00CE), `CZ_REQ_GIVE_MANNER_BYNAME` (0x0212), `CZ_REQ_GIVE_MANNER_POINT` (0x0149), `CZ_RECALL`/`CZ_RECALL_GID` (0x01BC/0x01BD), `CZ_REMOVE_AID` (0x01BA), `CZ_SHIFT` (0x01BB), `CZ_CHANGE_EFFECTSTATE` (0x019D hide), `CZ_CHANGE_MAPTYPE` (0x0198), `CZ_GM_FULLSTRIP` (0x07F5), `CZ_ITEM_CREATE` (0x013F), `CZ_REQ_ACCOUNTNAME` (0x01DF), plus 0x0842/0x0843 recall/remove by name, `LocalBroadcast` (0x019C).
- [ ] **Logging:** `conf/log_athena.conf` (item, chat, atcommand, mvp, npc logs).
- [ ] **Visual debugger** and websocket support exist; document the GM/ban equivalents if kept.

## 15. PvP, GvG and siege leftovers

Most is complete per the checkpoint. Remaining:

- [ ] PvP arena rooms and maps (`npc/pre-re/other/pvp.txt`, `guildpvp.txt`, `arena/*`): needs [Section 11](#11-maps-warps-and-spawns) maps and waiting rooms ([Section 3](#3-social-and-communication)).
- [ ] Blocking Warp Portal/teleport items inside a duel; rathena keeps duels across map changes.
- [ ] PvP ranking window request (0x020F, [Section 2](#21-gameplay-critical)); `pvpon`/`pvpoff` and `battle/misc.conf` PK options.
- [ ] War of Emperium: `agit_main` events, castle economy and guardian behaviours are done; verify WoE schedule (`doc/woe_time_explanation.txt`), `OnClock` driven start/stop, castle flag/emblem refresh, treasure drop rules (`GuildTreasure`), guild break during siege, guardian respawn.
- [ ] Battleground limits listed in the checkpoint: second arenas (need maps), shared Kafra/Repairman, GM switch NPC, `readbook`, carry-weight and bound reward items, the Tierra `A_CODE` quest touch.
- [ ] Guild war event stages (`npc/events/gdevent_*`).

## 16. Verification and technical debt

From the checkpoint (still open):

- [ ] Regression tests for riding numbers, guild skills, alliance flows, duels and monster conditions.
- [ ] Live-client sweeps: trading, hit packet timing and counts, traps, Talkie Box/Graffiti, Warp Portal, guild/alliance, duels, riding, client admission and logout hand-off.
- [ ] Audit item-transaction ordering and card/proc rounding; class/mode immunity across status, visibility and knockback.
- [ ] Retire the legacy login proxy ([Section 1](#1-accounts-login-and-character-server)).
- [ ] `character_service_tests::test_change_map_should_defer_position_update_in_db` has a 200 ms latch and can flake under load; make it deterministic.
- [ ] About 100 compiler warnings in the `server` crate.
- [ ] `docs/operations-checkpoint.md` says "Compiler warnings remain (about 100)" and "skip exhaustive testing": once the feature list above shrinks, restore full test runs.

From upstream issues (open on `nmeylan/rust-ro` as of this writing, most are stale):

- [ ] #58 `STATUS_ACCESS_VIOLATION` after repeated `@warp` while other sessions walk. The state model has since been rewritten to remove `unsafe` access; write the integration test to confirm it cannot recur.
- [ ] #46 clean `unwrap()`/`expect()` in request handlers (for example `request_handler/atcommand.rs` still unwraps `find(':')` and regex captures).
- [ ] #32 build on stable instead of nightly (`rust-toolchain.toml` pins nightly), #47 faster build time, #12 VM monitor (the Wasm runtime replaced the planned rathena VM; decide whether a monitor is still wanted).
- [ ] #65 movement speed stacking ([Section 8](#8-battle-system-and-server-configuration)).

### Upstream issue reconciliation

| # | Title | State in this fork |
|---|---|---|
| 3 | rathena script lang VM integration | Superseded: NPCs and items run as compiled Wasm (ADR 3). Close. |
| 5 | Kafra support | Open: town Kafras not converted ([Section 12](#12-npc-content)). |
| 9 | Inventory checklist | Mostly done (weight, loot, shops, drop, `@item`); the remaining items are `@getitem`-style commands and unaudited edge cases in [Section 6](#6-items-crafting-and-economy). |
| 11 | Skills (407 listed) | Superseded by [Section 7](#7-skills) and the checkpoint. |
| 12 | VM monitor | Open, likely obsolete. |
| 19 | Meta issue | Categories map to Sections 3, 7, 12 here. |
| 20 | Atcommands | Partly done ([Section 14](#14-gm-and-administration)). |
| 25 | Current todo | Done in substance: damage, refinement, element, magic and ranged formulas exist. |
| 32, 46, 47 | Stable toolchain, unwrap cleanup, build time | Open ([Section 16](#16-verification-and-technical-debt)). |
| 39 | `@refresh` mobs/warp | Open ([Section 14](#14-gm-and-administration)). |
| 42, 44, 49 | Stats and bonus, stats calc, status snapshot | Open as audit ([Section 8](#8-battle-system-and-server-configuration)). |
| 45 | Battle mechanism | Largely done; remaining parity is the battle config layer ([Section 8](#8-battle-system-and-server-configuration)). |
| 58 | Crash on repeated `@warp` | Re-test ([Section 16](#16-verification-and-technical-debt)). |
| 60 | Job changer script | Custom Job Master exists; stock job quests missing ([Section 12](#12-npc-content)). |
| 65 | Movement speed calculation | Open ([Section 8](#8-battle-system-and-server-configuration)). |

## Appendix A: client packets rathena parses that the fork does not

Generated from `clif_packetdb.hpp` at `PACKETVER=20120307`. Wire ids are the ones rathena registers (some handlers accept several ids; legacy ids are listed too). The third column is the fork packet type with that id at 20120307, where `lib/packets` has one. Section numbers point to the task that covers the feature. Treat the list as a strong hint, not proof: ids the fork canonicalises before decoding (`script_world_protocol.rs`) may be mis-reported. Specific suspects are called out in [Section 2.1](#21-gameplay-critical). `clif_parse_UseSkillToPosMoreInfo` (Talkie Box) is decoded through `talkie_box_packets.json` and is excluded; `FriendsListAdd` and `RepairItem` were added by hand (an unrelated `0x0369` frame matched the former, and the latter has no id table entry).

| rathena handler | ids | fork packet type | Section |
|---|---|---|---|
| `clif_parse_Auction_bid` | 0x024F | `PacketCzAuctionBuy` | 3 |
| `clif_parse_Auction_buysell` | 0x025C | `PacketCzAuctionReqMyInfo` | 3 |
| `clif_parse_Auction_cancel` | 0x024E | `PacketCzAuctionAddCancel` | 3 |
| `clif_parse_Auction_cancelreg` | 0x024B | `PacketCzAuctionCreate` | 3 |
| `clif_parse_Auction_close` | 0x025D | `PacketCzAuctionReqMySellStop` | 3 |
| `clif_parse_Auction_register` | 0x024D | `PacketCzAuctionAdd` | 3 |
| `clif_parse_Auction_search` | 0x0251 | `PacketCzAuctionItemSearch` | 3 |
| `clif_parse_Auction_setitem` | 0x024C | `PacketCzAuctionAddItem` | 3 |
| `clif_parse_AutoRevive` | 0x0292 | `PacketCzStandingResurrection` | 2.1 |
| `clif_parse_AutoSpell` | 0x01CE | `PacketCzSelectautospell` | 7 |
| `clif_parse_Broadcast` | 0x0099 | `PacketCzBroadcast` | 3 |
| `clif_parse_CashShopReqTab` | 0x0846 |  | 6 |
| `clif_parse_ChangeChatOwner` | 0x00E0 | `PacketCzReqRoleChange` | — |
| `clif_parse_ChangeDir` | 0x0085, 0x0361, 0x0890 | `PacketCzChangeDirection` | 2.1 |
| `clif_parse_ChatAddMember` | 0x00D9 | `PacketCzReqEnterRoom` | 3 |
| `clif_parse_ChatLeave` | 0x00E3 | `PacketCzExitRoom` | 3 |
| `clif_parse_ChatRoomStatusChange` | 0x00DE | `PacketCzChangeChatroom` | 3 |
| `clif_parse_Check` | 0x0213 | `PacketCzReqStatusGm` | 14 |
| `clif_parse_CloseKafra` | 0x0193 | `PacketCzCloseStore` | 2.1 |
| `clif_parse_CreateChatRoom` | 0x00D5 | `PacketCzCreateChatroom` | — |
| `clif_parse_Emotion` | 0x00BF | `PacketCzReqEmotion` | 2.1 |
| `clif_parse_FeelSaveOk` | 0x0254 | `PacketCzAgreeStarplace` | 7 |
| `clif_parse_FriendsListAdd` | 0x0202, 0x0436, 0x0369 | `PacketCzAddFriends` | 3 |
| `clif_parse_FriendsListRemove` | 0x0203 | `PacketCzDeleteFriends` | 3 |
| `clif_parse_FriendsListReply` | 0x0208 | `PacketCzAckReqAddFriends` | 3 |
| `clif_parse_GMChangeMapType` | 0x0198 | `PacketCzChangeMaptype` | 14 |
| `clif_parse_GMFullStrip` | 0x07F5 | `PacketCzGmFullstrip` | 14 |
| `clif_parse_GMHide` | 0x019D | `PacketCzChangeEffectstate` | 14 |
| `clif_parse_GMKick` | 0x00CC | `PacketCzDisconnectCharacter` | 14 |
| `clif_parse_GMKickAll` | 0x00CE | `PacketCzDisconnectAllCharacter` | 14 |
| `clif_parse_GMRc` | 0x0212 | `PacketCzReqGiveMannerByname` | 14 |
| `clif_parse_GMRecall` | 0x01BC, 0x01BD | `PacketCzRecall` | 14 |
| `clif_parse_GMRecall2` | 0x0842 |  | 14 |
| `clif_parse_GMRemove2` | 0x0843 |  | 14 |
| `clif_parse_GMReqAccountName` | 0x01DF | `PacketCzReqAccountname` | 14 |
| `clif_parse_GMReqNoChat` | 0x0149 | `PacketCzReqGiveMannerPoint` | 14 |
| `clif_parse_GMShift` | 0x01BA, 0x01BB | `PacketCzRemoveAid` | 14 |
| `clif_parse_GM_Item_Monster` | 0x013F | `PacketCzItemCreate` | 14 |
| `clif_parse_HowManyConnections` | 0x00C1 | `PacketCzReqUserCount` | 2.1 |
| `clif_parse_ItemListWindowSelected` | 0x07E4, 0x0870 | `PacketCzItemlistwinRes` | out of scope |
| `clif_parse_KickFromChat` | 0x00E2 | `PacketCzReqExpelMember` | 3 |
| `clif_parse_LessEffect` | 0x021D | `PacketCzLesseffect` | 2.1 |
| `clif_parse_LocalBroadcast` | 0x019C |  | 3 |
| `clif_parse_Mail_delete` | 0x0243 | `PacketCzMailDelete` | 3 |
| `clif_parse_Mail_getattach` | 0x0244 | `PacketCzMailGetItem` | 3 |
| `clif_parse_Mail_read` | 0x0241 | `PacketCzMailOpen` | 3 |
| `clif_parse_Mail_refreshinbox` | 0x023F | `PacketCzMailGetList` | 3 |
| `clif_parse_Mail_return` | 0x0273 | `PacketCzReqMailReturn` | 3 |
| `clif_parse_Mail_send` | 0x0248 | `PacketCzMailSend` | 3 |
| `clif_parse_Mail_setattach` | 0x0247 | `PacketCzMailAddItem` | 3 |
| `clif_parse_Mail_winopen` | 0x0246 | `PacketCzMailResetItem` | 3 |
| `clif_parse_MapMove` | 0x0140 | `PacketCzMovetoMap` | 14 |
| `clif_parse_MemorialDungeonCommand` | 0x02CF | `PacketCzMemorialdungeonCommand` | 5 |
| `clif_parse_NoviceDoriDori` | 0x01E7 | `PacketCzDoridori` | 2.1 |
| `clif_parse_NoviceExplosionSpirits` | 0x01ED | `PacketCzChopokgi` | 2.1 |
| `clif_parse_NpcCloseClicked` | 0x0146 | `PacketCzCloseDialog` | 2.1 |
| `clif_parse_PMIgnore` | 0x00CF | `PacketCzSettingWhisperPc` | 3 |
| `clif_parse_PMIgnoreAll` | 0x00D0 | `PacketCzSettingWhisperState` | 3 |
| `clif_parse_PMIgnoreList` | 0x00D3 | `PacketCzReqWhisperList` | 3 |
| `clif_parse_PVPInfo` | 0x020F | `PacketCzReqPvppoint` | 2.1 |
| `clif_parse_QuitGame` | 0x018A | `PacketCzReqDisconnect` | 2.1 |
| `clif_parse_RepairItem` | n/a |  | 6 |
| `clif_parse_ResetChar` | 0x0197 | `PacketCzReset` | 2.1 |
| `clif_parse_SelectArrow` | 0x01AE | `PacketCzReqMakingarrow` | 6/7 |
| `clif_parse_SkillSelectMenu` | 0x0443 | `PacketCzSkillSelectResponse` | out of scope |
| `clif_parse_SolveCharName` | 0x00A2, 0x0368 | `PacketCzReqnameBygid` | 2.1 |
| `clif_parse_StopAttack` | 0x0118 | `PacketCzCancelLockon` | 2.1 |
| `clif_parse_StoragePassword` | 0x023B, 0x0281, 0x0861 | `PacketCzAckStorePassword` | 6 |
| `clif_parse_ViewPlayerEquip` | 0x02D6 | `PacketCzEquipwinMicroscope` | 2.1 |
| `clif_parse_WeaponRefine` | 0x0222 | `PacketCzReqWeaponrefine` | 6 |
| `clif_parse_WisMessage` | 0x0096 | `PacketCzWhisper` | — |
| `clif_parse_client_version` | 0x044A | `PacketCzClientVersion` | 2.1 |
| `clif_parse_configuration` | 0x02D8 | `PacketCzConfig` | 2.1 |
| `clif_parse_npccashshop_buy` | 0x0288 | `PacketCzPcBuyCashPointItem` | 6 |
| `clif_parse_progressbar` | 0x02F1 | `PacketCzProgress` | 2.1 |
| `clif_parse_questStateAck` | 0x02B6 | `PacketCzActiveQuest` | 4 |
| `clif_parse_ranklist_killer` | 0x0237 |  | out of scope (not in 20120307 pre-re) |

## Appendix B: rathena atcommands

All 314 commands in `conf/atcommands.yml`. A check mark means the fork has a command or alias of that name in `request_handler/atcommand.rs`; the match is by name and approximate (`@rates`, `@jobchange`, `@baselevelup` are matched by alias).

<details><summary>314 commands</summary>

✔ `@accept`, `@accinfo`, `@addfame`, `@addperm`, `@addwarp`, `@adjgroup`, `@adopt`, `@agi`, `@agitend`, `@agitend2`, `@agitend3`, `@agitstart`, `@agitstart2`, `@agitstart3`, `@alive`, `@allowks`, `@allskill`, `@auction`, `@autoloot`, `@autolootitem`, `@autoloottype`, `@autotrade`, `@ban`, ✔ `@baselevelup`, `@bodystyle`, `@breakguild`, `@broadcast`, `@camerainfo`, `@cart`, `@cartlist`, `@cash`, `@changedress`, `@changegm`, `@changeleader`, `@channel`, `@changecharsex`, `@changelook`, `@changesex`, `@char_ban`, `@char_block`, `@char_unban`, `@char_unblock`, `@charcommands`, `@checkquest`, `@clanspy`, `@cleanarea`, `@cleanmap`, `@clearcart`, `@cleargstorage`, `@clearstorage`, `@clearweather`, `@clone`, `@cloneequip`, `@clonestat`, `@clouds`, `@clouds2`, `@commands`, `@completequest`, `@con`, `@costume`, `@crt`, `@day`, `@delitem`, `@dex`, `@disguise`, `@disguiseall`, `@disguiseguild`, `@displayskill`, `@displayskillcast`, `@displayskillunit`, `@displaystatus`, `@divorce`, `@doom`, `@doommap`, `@dropall`, ✔ `@duel`, `@dye`, `@effect`, `@email`, `@enchantgradeui`, `@erasequest`, `@evilclone`, `@exp`, `@fakename`, `@feelreset`, `@fireworks`, `@fog`, `@font`, `@fontcolor`, `@follow`, `@fullstrip`, `@gat`, `@guildlevelup`, `@gmotd`, ✔ `@go`, `@grade`, `@guild`, `@guildrecall`, `@guildspy`, `@guildstorage`, `@gvgoff`, `@gvgon`, `@hair_color`, `@hair_style`, `@hatch`, `@hatereset`, ✔ `@heal`, `@healap`, `@help`, `@hide`, `@hidenpc`, `@homevolution`, `@homfriendly`, `@homhungry`, `@hominfo`, `@homlevel`, `@hommutate`, `@homshuffle`, `@homstats`, `@homtalk`, `@identify`, `@identifyall`, `@idsearch`, `@int`, ✔ `@invite`, ✔ `@item`, `@item2`, `@itembound`, `@itembound2`, `@iteminfo`, `@itemlist`, `@itemreset`, `@jail`, `@jailfor`, `@jailtime`, ✔ `@jobchange`, ✔ `@joblevelup`, `@join`, `@jump`, `@jumpto`, `@kami`, `@kamib`, `@kamic`, `@kick`, `@kickall`, `@kill`, `@killable`, ✔ `@killer`, `@killmonster`, `@killmonster2`, `@ksprotection`, `@langtype`, ✔ `@leave`, `@leaves`, `@limitedsale`, `@lkami`, `@load`, `@loadnpc`, `@localbroadcast`, `@lostskill`, `@luk`, `@macrochecker`, `@mail`, `@makeegg`, `@makehomun`, `@mapexit`, `@mapflag`, `@mapinfo`, `@mapmove`, `@marry`, `@me`, `@memo`, `@misceffect`, `@mobinfo`, `@mobsearch`, `@model`, `@monster`, `@monsterbig`, `@monsterignore`, `@monstersmall`, `@mount_peco`, `@mount2`, `@mute`, `@mutearea`, `@night`, `@noask`, `@npcmove`, `@npctalk`, `@nuke`, `@option`, `@party`, `@partyoption`, `@partyrecall`, `@partysharelvl`, `@partyspy`, `@petfriendly`, `@pethungry`, `@petrename`, `@pettalk`, `@points`, `@pow`, `@produce`, `@pvpoff`, `@pvpon`, `@questskill`, `@raise`, `@raisemap`, ✔ `@rates`, `@recall`, `@recallall`, `@refine`, `@refineui`, `@refineui`, `@refresh`, `@refreshall`, ✔ `@reject`, ✔ `@reload`, `@reloadachievementdb`, `@reloadatcommand`, `@reloadattendancedb`, `@reloadbarterdb`, `@reloadbattleconf`, `@reloadcashdb`, `@reloadinstancedb`, `@reloaditemdb`, `@reloadlogconf`, `@reloadmobdb`, `@reloadmotd`, `@reloadmsgconf`, `@reloadnpcfile`, `@reloadpcdb`, `@reloadquestdb`, `@reloadscript`, `@reloadskilldb`, `@reloadstatusdb`, `@repairall`, `@request`, `@reset`, `@resetcooltime`, ✔ `@resetskill`, ✔ `@resetstat`, `@resurrect`, `@rmvperm`, `@roulette`, `@sakura`, `@save`, `@send`, `@servertime`, `@set`, `@setbattleflag`, `@setcard`, `@setquest`, `@showdelay`, `@showexp`, `@showmobs`, `@shownpc`, `@showrate`, `@showzeny`, `@size`, `@sizeall`, `@sizeguild`, `@skillid`, `@skilloff`, `@skillon`, `@skillpoint`, `@skilltree`, `@slaveclone`, `@snow`, `@soulball`, `@sound`, ✔ `@speed`, `@spiritball`, `@spl`, `@sta`, `@stat_all`, `@stats`, `@statuspoint`, `@stockall`, `@storage`, `@storeall`, `@storagelist`, `@str`, `@stylist`, `@summon`, `@tonpc`, `@trade`, `@trait_all`, `@traitpoint`, `@unban`, `@undisguise`, `@undisguiseall`, `@undisguiseguild`, `@unjail`, `@unloadnpc`, `@unloadnpcfile`, `@unmute`, `@uptime`, `@users`, `@useskill`, `@version`, `@vip`, `@vit`, `@where`, `@whereis`, `@who`, `@who2`, `@who3`, `@whodrops`, `@whogm`, `@whomap`, `@whomap2`, `@whomap3`, `@wis`, `@zeny`

</details>

## Appendix C: skills with metadata but no bespoke code reference

Skills in the classic range that exist in `server/src/server/script/skill_metadata.json` but whose enum variant (`SkillEnum::X`) and name string never appear in non-test server code (316 of 650). They may be fully handled by the generic metadata path (damage, status, duration, requirements), or silently incomplete. Audit against `src/map/skills/**` in rathena. Prefixes: player classes (`NV` Novice … `NJ` Ninja), `GS` Gunslinger, `TK`/`SG`/`SL` Taekwon family, `NPC` monster skills, `ALL`/`CASH`/`ITM`/`WE`/`GM` miscellaneous.

| Prefix | Count | Skills |
|---|---|---|
| `NPC` | 71 | `NPC_MENTALBREAKER` `NPC_RANGEATTACK` `NPC_ATTRICHANGE` `NPC_COMBOATTACK` `NPC_GUIDEDATTACK` `NPC_SPLASHATTACK` `NPC_BLINDATTACK` `NPC_SILENCEATTACK` `NPC_STUNATTACK` `NPC_PETRIFYATTACK` `NPC_CURSEATTACK` `NPC_SLEEPATTACK` `NPC_RANDOMATTACK` `NPC_GROUNDATTACK` `NPC_BLOODDRAIN` `NPC_ENERGYDRAIN` `NPC_KEEPING` `NPC_DARKBLESSING` `NPC_BARRIER` `NPC_LICK` `NPC_HALLUCINATION` `NPC_DARKCROSS` `NPC_DARKTHUNDER` `NPC_STOP` `NPC_CHANGEUNDEAD` `NPC_AGIUP` `NPC_SIEGEMODE` `NPC_INVISIBLE` `NPC_FIREBREATH` `NPC_ICEBREATH` `NPC_THUNDERBREATH` `NPC_ACIDBREATH` `NPC_DARKNESSBREATH` `NPC_BLEEDING` `NPC_WIDESIGHT` `NPC_INVINCIBLE` `NPC_INVINCIBLEOFF` `NPC_VENOMFOG` `NPC_MILLENNIUMSHIELD` `NPC_COMET` `NPC_PULSESTRIKE2` `NPC_DANCINGBLADE` `NPC_DANCINGBLADE_ATK` `NPC_MAXPAIN` `NPC_MAXPAIN_ATK` `NPC_JACKFROST` `NPC_WIDEWEB` `NPC_WIDESUCK` `NPC_STORMGUST2` `NPC_FIRESTORM` `NPC_REVERBERATION` `NPC_REVERBERATION_ATK` `NPC_LEX_AETERNA` `NPC_ARROWSTORM` `NPC_CHEAL` `NPC_SR_CURSEDCIRCLE` `NPC_DRAGONBREATH` `NPC_FATALMENACE` `NPC_MAGMA_ERUPTION` `NPC_MAGMA_ERUPTION_DOTDAMAGE` `NPC_MANDRAGORA` `NPC_PSYCHIC_WAVE` `NPC_RAYOFGENESIS` `NPC_VENOMIMPRESS` `NPC_CLOUD_KILL` `NPC_IGNITIONBREAK` `NPC_PHANTOMTHRUST` `NPC_POISON_BUSTER` `NPC_HALLUCINATIONWALK` `NPC_ELECTRICWALK` `NPC_FIREWALK` |
| `SL` | 20 | `SL_ALCHEMIST` `SL_STAR` `SL_SAGE` `SL_SUPERNOVICE` `SL_KNIGHT` `SL_WIZARD` `SL_BARDDANCER` `SL_BLACKSMITH` `SL_SOULLINKER` `SL_KAIZEL` `SL_KAAHI` `SL_KAUPE` `SL_KAITE` `SL_SWOO` `SL_SKE` `SL_SKA` `SL_DEATHKNIGHT` `SL_COLLECTOR` `SL_NINJA` `SL_GUNNER` |
| `NJ` | 19 | `NJ_TOBIDOUGU` `NJ_SYURIKEN` `NJ_KUNAI` `NJ_TATAMIGAESHI` `NJ_KASUMIKIRI` `NJ_SHADOWJUMP` `NJ_KIRIKAGE` `NJ_UTSUSEMI` `NJ_BUNSINJYUTSU` `NJ_NINPOU` `NJ_KOUENKA` `NJ_KAENSIN` `NJ_BAKUENRYU` `NJ_HYOUSENSOU` `NJ_SUITON` `NJ_HYOUSYOURAKU` `NJ_HUUJIN` `NJ_RAIGEKISAI` `NJ_KAMAITACHI` |
| `SA` | 18 | `SA_CASTCANCEL` `SA_FREECAST` `SA_AUTOSPELL` `SA_VOLCANO` `SA_VIOLENTGALE` `SA_ABRACADABRA` `SA_MONOCELL` `SA_CLASSCHANGE` `SA_SUMMONMONSTER` `SA_DEATH` `SA_FORTUNE` `SA_TAMINGMONSTER` `SA_QUESTION` `SA_GRAVITY` `SA_LEVELUP` `SA_INSTANTDEATH` `SA_FULLRECOVERY` `SA_COMA` |
| `SG` | 18 | `SG_FEEL` `SG_SUN_WARM` `SG_MOON_WARM` `SG_STAR_WARM` `SG_SUN_COMFORT` `SG_MOON_COMFORT` `SG_STAR_COMFORT` `SG_HATE` `SG_SUN_ANGER` `SG_MOON_ANGER` `SG_STAR_ANGER` `SG_SUN_BLESS` `SG_MOON_BLESS` `SG_STAR_BLESS` `SG_DEVIL` `SG_FRIEND` `SG_KNOWLEDGE` `SG_FUSION` |
| `GS` | 15 | `GS_FLING` `GS_TRIPLEACTION` `GS_BULLSEYE` `GS_MADNESSCANCEL` `GS_ADJUSTMENT` `GS_INCREASING` `GS_MAGICALBULLET` `GS_CRACKER` `GS_DISARM` `GS_PIERCINGSHOT` `GS_DESPERADO` `GS_GATLINGFEVER` `GS_DUST` `GS_FULLBUSTER` `GS_GROUNDDRIFT` |
| `TK` | 14 | `TK_READYSTORM` `TK_STORMKICK` `TK_READYDOWN` `TK_DOWNKICK` `TK_READYTURN` `TK_TURNKICK` `TK_READYCOUNTER` `TK_COUNTER` `TK_DODGE` `TK_JUMPKICK` `TK_HPTIME` `TK_SPTIME` `TK_POWER` `TK_SEVENWIND` |
| `BS` | 11 | `BS_ORIDEOCON` `BS_SWORD` `BS_TWOHANDSWORD` `BS_AXE` `BS_MACE` `BS_KNUCKLE` `BS_SPEAR` `BS_FINDINGORE` `BS_REPAIRWEAPON` `BS_SKINTEMPER` `BS_ADRENALINE2` |
| `CR` | 11 | `CR_TRUST` `CR_SHIELDCHARGE` `CR_HOLYCROSS` `CR_PROVIDENCE` `CR_SPEARQUICKEN` `CR_ALCHEMY` `CR_SYNTHESISPOTION` `CR_SLIMPITCHER` `CR_FULLPROTECTION` `CR_ACIDDEMONSTRATION` `CR_CULTIVATION` |
| `BD` | 10 | `BD_ADAPTATION` `BD_ENCORE` `BD_LULLABY` `BD_RICHMANKIM` `BD_ETERNALCHAOS` `BD_DRUMBATTLEFIELD` `BD_RINGNIBELUNGEN` `BD_ROKISWEIL` `BD_INTOABYSS` `BD_SIEGFRIED` |
| `AM` | 7 | `AM_POTIONPITCHER` `AM_CP_WEAPON` `AM_CP_SHIELD` `AM_CP_ARMOR` `AM_CP_HELM` `AM_BIOETHICS` `AM_BERSERKPITCHER` |
| `MO` | 7 | `MO_SPIRITSRECOVERY` `MO_ABSORBSPIRITS` `MO_TRIPLEATTACK` `MO_STEELBODY` `MO_BLADESTOP` `MO_CHAINCOMBO` `MO_COMBOFINISH` |
| `PF` | 7 | `PF_HPCONVERSION` `PF_SOULCHANGE` `PF_SOULBURN` `PF_MINDBREAKER` `PF_MEMORIZE` `PF_FOGWALL` `PF_DOUBLECASTING` |
| `WS` | 7 | `WS_MELTDOWN` `WS_CREATECOIN` `WS_CREATENUGGET` `WS_CARTBOOST` `WS_SYSTEMCREATE` `WS_WEAPONREFINE` `WS_OVERTHRUSTMAX` |
| `RG` | 6 | `RG_SNATCHER` `RG_STEALCOIN` `RG_FLAGGRAFFITI` `RG_GANGSTER` `RG_COMPULSION` `RG_PLAGIARISM` |
| `DC` | 6 | `DC_THROWARROW` `DC_UGLYDANCE` `DC_HUMMING` `DC_DONTFORGETME` `DC_FORTUNEKISS` `DC_SERVICEFORYOU` |
| `BA` | 5 | `BA_DISSONANCE` `BA_WHISTLE` `BA_ASSASSINCROSS` `BA_POEMBRAGI` `BA_APPLEIDUN` |
| `MC` | 4 | `MC_INCCARRY` `MC_DISCOUNT` `MC_OVERCHARGE` `MC_CHANGECART` |
| `KN` | 4 | `KN_BRANDISHSPEAR` `KN_SPEARSTAB` `KN_SPEARBOOMERANG` `KN_ONEHAND` |
| `HT` | 4 | `HT_STEELCROW` `HT_BLITZBEAT` `HT_DETECTING` `HT_POWER` |
| `LK` | 4 | `LK_PARRYING` `LK_TENSIONRELAX` `LK_FURY` `LK_HEADCRUSH` |
| `HW` | 4 | `HW_SOULDRAIN` `HW_MAGICCRASHER` `HW_MAGICPOWER` `HW_NAPALMVULCAN` |
| `CG` | 4 | `CG_MOONLIT` `CG_MARIONETTE` `CG_LONGINGFREEDOM` `CG_HERMODE` |
| `ALL` | 4 | `ALL_INCCARRY` `ALL_CATCRY` `ALL_DREAM_SUMMERNIGHT` `ALL_WEWISH` |
| `AC` | 3 | `AC_SHOWER` `AC_MAKINGARROW` `AC_CHARGEARROW` |
| `PR` | 3 | `PR_BENEDICTIO` `PR_SLOWPOISON` `PR_MAGNUS` |
| `WE` | 3 | `WE_MALE` `WE_FEMALE` `WE_BABY` |
| `ASC` | 3 | `ASC_EDP` `ASC_BREAKER` `ASC_METEORASSAULT` |
| `SN` | 3 | `SN_SIGHT` `SN_FALCONASSAULT` `SN_SHARPSHOOTING` |
| `CASH` | 3 | `CASH_BLESSING` `CASH_INCAGI` `CASH_ASSUMPTIO` |
| `TF` | 2 | `TF_STEAL` `TF_SPRINKLESAND` |
| `AS` | 2 | `AS_GRIMTOOTH` `AS_ENCHANTPOISON` |
| `SM` | 2 | `SM_MOVINGRECOVERY` `SM_AUTOBERSERK` |
| `HP` | 2 | `HP_BASILICA` `HP_MANARECHARGE` |
| `CH` | 2 | `CH_TIGERFIST` `CH_CHAINCRUSH` |
| `ST` | 2 | `ST_REJECTSWORD` `ST_PRESERVE` |
| `MG` | 1 | `MG_NAPALMBEAT` |
| `AL` | 1 | `AL_DP` |
| `WZ` | 1 | `WZ_SIGHTRASHER` |
| `ITM` | 1 | `ITM_TOMAHAWK` |
| `PA` | 1 | `PA_GOSPEL` |
| `GM` | 1 | `GM_SANDMAN` |
