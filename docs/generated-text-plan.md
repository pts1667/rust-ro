# Plan: generated descriptions and guides for `/db`

Status: plan only. Nothing here is generated yet. The bulk pass runs later, on the local llama.cpp server.

## Goal

`/db/quests/<id>` should carry these generated fields:

- `description`: one or two sentences saying what the quest is about.
- `guide`: a short list of steps a player follows, naming the NPC and map for each step.
- `recommended_level`: the level a player should have to do the quest, or `"Unknown"`. See the Recommended level section.
- `level_basis`: one sentence saying which facts the level came from.

The text is machine-written. Every entry records that it was generated, so `/db` never presents it as authored text.

## Scope

**Quests: 956 entries generated.** These are the quests a pre-re player can see in-game and that have an excerpt:

- 956 quests with a literal call (`setquest`, `completequest`, `checkquest`, `changequest`, `questprogress`, `isbegin_quest`, `questinfo`) in a file of the pre-re load chain. The chain starts at `../rathena/npc/pre-re/scripts_main.conf` and follows its `import:` and `npc:` lines.
- Not generated in this pass: the four Merchant job-change quests, 1009–1012. The merchant job master passes them to a helper as call arguments, so the literal scan misses them (see Inputs, item 3). They will be written by hand later.

Quests outside this set are left without text. That covers:

- renewal-only quests, such as quest 1100 (only `npc/re/quests/newgears/2010_headgears.txt` starts it), and the renewal job changes (Rune Knight 3200–3220, Mechanic 10090–10101). Those jobs exist only under `npc/re/`.
- quests with no reference anywhere in rathena `npc/`, such as 2042 (Sage job change). Nothing in rathena starts it.

**Items: not in this plan.** Item descriptions are out. The client's item description table (`idnum2itemdesctable`) decodes with the same EUC-KR parser nostalro-client uses, but its Korean text is altered in a consistent way (for example `횅독` where `회복` is expected), so it is not usable as source text. The wiki has no per-item pages for most items. Revisit only after an in-game check shows the client displays clean text.

## Facts found while scoping

| Question | Result |
| --- | --- |
| Quests in `server/src/server/script/quests.json` | 2,879 |
| Referenced by a literal call in the pre-re chain | 956 |
| Referenced only outside the chain (mostly `npc/re/`) | 1,296 |
| No literal reference anywhere in `npc/` | 627 |
| Job-change quests with no literal quest call in the chain | 38: 33 renewal-only (Rune Knight, Mechanic), 2042 (unreferenced), 1009–1012 (Merchant, passed as call arguments; filled by hand later) |
| Quests with a named monster objective (`mob` set) | 349 |

The wiki is not an input to this pass. Only 36 quest titles match a wiki page, too few to rely on.

## Inputs

### Quests

Each quest gets one prompt with these parts:

1. **Facts** from `quests.json`: title, time limit, and each objective as a monster name and count. Monster names come from `config/mobs.json`.
2. **Script excerpts**: every NPC block in a pre-re chain file that contains a reference to the quest id. An NPC block is the header line (`map,x,y,dir<TAB>script<TAB>name<TAB>sprite,{`) through the next header. The header gives the NPC name, map and position, and the prompt includes it.
3. **Merchant job-change quests (1009–1012), skipped for now.** The merchant job master, `npc/pre-re/jobs/1-1/merchant.txt`, does not write these ids as quest calls. It passes each id as the last argument of `callsub S_GiveSerial,<serial>,<n>,<serial>,<n>,<id>` (lines 390–405), and the label `S_GiveSerial` (line 480) calls `setquest getarg(4)`. The same file completes the four in a `for (.@quest_id = 1009; .@quest_id <= 1012; ...)` loop (line 157). Their excerpt would be the `Merchant#mer` block, lines 34–497, about 16 KB. These four are filled in by hand later, so the generator has no override for them.

Excerpt: the rathena NPC source block (one NPC or one function, from its header line to the next header) that contains the quest reference. It is passed to the model verbatim. The model sees the dialogue and conditions of that block, nothing else.

Excerpts are capped at 24,000 characters per quest. When a quest exceeds the cap, blocks that name the quest id go first.

**NPC references.** The excerpt names the NPCs the quest involves by their human name, not only by the block's header. The name comes from the script text, and the excerpt uses these places:

- the block's own header: `Name#suffix` gives the human name `Name`, with the map and position on the same line;
- speaker tags in dialogue, `mes "[Name]"` or `lines_as("Name", ...)`, which name the NPC who is talking;
- calls to another NPC, such as `donpcevent "Name#suffix::Label"`, `enablenpc`, `disablenpc`, `hideonnpc`, `hideoffnpc` and `setnpcdisplay` with a name.

When a referenced name matches an NPC block in the pre-re chain, the excerpt also includes that block's header (name, map, x, y), so the model can say where the NPC is. A name with no matching block stays as plain text.

**No excerpt, no generation.** A quest with no excerpt gets no `description` and no `guide`. It is skipped, not sent to the model. Its `/db` entry keeps the title, time limit and objectives from `quests.json`, and `notes` says that no generated text exists for it.

## Recommended level

The generator works out the level from facts for sources 1 to 3. For source 4 (item droppers) the model picks one of the listed dropper levels; see the source 4 note and open question 1.

`recommended_level` is the Base Level a player should have for the quest. It is not the Job Level. Each quest gets `recommended_level`, an integer or `"Unknown"`, and `level_basis`, one sentence naming the source. The generator takes the first source that gives a level:

1. **Level gate in the quest's own NPC script.** A `BaseLevel >= N` or `BaseLevel > N` check in the excerpt. For `>`, the level is N+1. When there are several, the largest. Lines containing `getexp` are reward branches, not access checks, and are skipped. The game enforces a gate, so it wins over the other sources.
2. **Monsters the quest asks you to defeat.** The mob objectives in `quests.json` (357 objectives over 349 quests). The level is the highest `level` in `config/mobs.json` among them.
3. **Monsters named in the excerpt.** The `monster` and `areamonster` spawn calls in the excerpt name a monster and give its id. The generator uses that id; when the id is not in `config/mobs.json`, it falls back to the name, and a name that matches more than one mob is left out. `killmonster` and `mobcount` take event labels, not monster names, so they are not used. The highest level wins.
4. **Monsters that drop the quest items.** Item ids or aegis names the script counts or removes (`countitem`, `delitem`). `getitem` is not used: it mostly gives rewards. For each item, the monsters that drop it, from the drop tables in `config/mobs.json`, listed by name and level. An item with more than 10 droppers is not listed. Its excerpt entry says the item is a common item, assumed easily available, with no level requirement. If the listed droppers span more than 20 levels, no level is taken from them and the quest is `"Unknown"`, because a range that wide does not support one number.
5. **Otherwise `"Unknown"`.** That includes the case where every name in source 3 is ambiguous and no item has a usable dropper list.

For sources 1 to 3 the level is computed by the generator, as above. For source 4 the generator does not compute a number. The model must pick exactly one of the listed dropper levels (the excerpt lists them), and says which item and monster it came from in `level_basis`.

The excerpt carries a "referenced monsters" block: each monster's id, name and level, from sources 2 to 4. For item droppers it also names the item each monster drops. The model uses that block to write `level_basis`.

**Override.** The script may check `BaseLevel` in a condition the rules above cannot see, such as `BaseLevel >= 40` in a branch the generator did not read. The model may then set `level_override` to `{"level": 40, "quote": "<that line>"}`, and `recommended_level` must equal that number. A Job Level check and a dialogue line are never an override: a Job Level 9 requirement would turn into a Base Level 9 otherwise. The override may not be lower than a source 1 gate, because the game enforces that gate. The validator checks that the quote is a line of the excerpt and contains `BaseLevel` and the number.

## Output

Committed file: `config/quest_text.json`, keyed by quest id as a string (values are an example):

```json
{
  "1100": {
    "description": "...",
    "guide": ["Talk to ... in ... (x,y).", "..."],
    "recommended_level": 60,
    "level_basis": "Target monster Zerom (level 60) from the quest objectives.",
    "sources": ["Errende (prt_fild08 103,47)"],
    "generated": true,
    "model": "tiel-coder"
  }
}
```

The server embeds the file with `include_str!`, the same way it embeds `quests.json`. Generation runs offline. Running the server never calls the model.

## Generator

`tools/scripts-import/gen_quest_text.py`, run with `python -I -X utf8` (`python3` is the Windows Store stub on this machine; `-I` keeps the script directory off the import path, as for any Python that reads downloaded files).

- Reads `--source-root ../rathena`, the quest catalog and `config/mobs.json`.
- Calls the llama.cpp server at `http://127.0.0.1:8080/v1/chat/completions` with the `tiel-coder` model, temperature 0.2, four requests in flight to match the four slots.
- Writes one line per quest to a cache file (`tools/scripts-import/quest_text.cache.jsonl`, not committed) so an interrupted run resumes without repeating work.
- Writes `config/quest_text.json` from the cache at the end.
- Retries a quest up to two times when validation fails. Records failures with their reason; a failed quest gets no entry.

The prompt is in the script, next to the parsing code, so the rules and the checks change together.

## Prompt rules

- Use only the facts and excerpts given. Do not invent names, places, items, rewards or numbers.
- If the excerpts do not show what the player must do, say that in `description` and give fewer steps.
- Reply with JSON only, in the shape `{"description": string, "guide": [string, ...], "recommended_level": integer or "Unknown", "level_basis": string, "level_override": {"level": integer, "quote": string} | null}`. Use `null` unless the excerpt states a requirement (see Override).
- `guide` has one to six steps. Each step names the NPC and map from the excerpt where it applies, and says only what the excerpt shows the player doing or being told. A step does not say the player must collect or hunt an item or monster unless the excerpt shows it.
- `recommended_level` is the Base Level only. A Job Level requirement is not a level for this field; it may be mentioned in `description`.
- `level_basis` is one sentence. When the level is a number, it names the monster or gate the level came from, using only the referenced monsters block and the gate. When it is `"Unknown"`, it says "No monster, item or level gate in the facts sets a level." The prompt gives this sentence exactly.
- The Recommended level section of the prompt says in plain words which case applies, with an example for each of gate, item droppers and Unknown. The examples are made-up names and are followed by the same rules.

## Validation

Checked by the script, not by reading:

1. The reply parses as JSON with the required keys, `description` as a non-empty string and `guide` as one to six strings.
2. The description is at most two sentences.
3. Without an override, `recommended_level` follows the rules: for sources 1 to 3 it equals the derived value; for source 4 it is one of the listed dropper levels; and it is `"Unknown"` exactly when no source gave a level. Any other value means the model contradicted the facts, and the entry is rejected.
4. `description`, `guide` and `level_basis` name only monsters and items that appear in the facts or the excerpt. Names are matched whole-word and case-sensitively, so the verb "drops" does not match the monster Drops. The description and guide must not contain `#` or runs of question marks.
5. With an override, `level_override.quote` is a line of the excerpt that contains `BaseLevel`, `level_override.level` appears in that quote, and the level is not lower than a source 1 gate. If the quote or gate check fails, the override is dropped and the entry falls back to rule 3. If the quote and gate pass but `recommended_level` differs from the override, the entry is rejected and the model is asked again.

NPC names, map names, counts, ids and coordinates are not checked. The model is trusted with them, and the excerpt is the only source it is given. NPC names from the script that contain question marks are dropped, because they are encoding damage. A person reviews the output (see below).

Then a person reads a random sample of 50 entries, and every entry that mentions a reward, a prerequisite or a place outside the excerpts.

## Server changes

- `server/src/server/reference/db/quests.rs` reads `config/quest_text.json` and adds `description`, `guide`, `recommended_level` (integer or `"Unknown"`), `level_basis`, `sources` and `generated` to the entry.
- The entry's `notes` text says the description and guide are generated, and lists what the excerpts did not cover.
- `docs/reference-api.md` documents the fields and marks them as generated.
- A test checks that every key in `quest_text.json` is a quest in the catalog, and that each entry has `generated: true`.

## Running it

```shell
curl http://127.0.0.1:8080/health
python -I -X utf8 tools/scripts-import/gen_quest_text.py --source-root ../rathena
cargo test --release -p server reference
```

Before the full run, time 20 quests and check: reply length, whether reasoning tokens appear in the reply, and the rejection rate. If the model emits a reasoning block, turn it off for the request (`chat_template_kwargs: {"enable_thinking": false}`) and check again.

Timing is unmeasured for quests. A single item took 3–8 seconds. Quest prompts are larger, so expect longer. Four slots in parallel should finish about 1,000 quests in well under a day; measure before committing to that.

## Open questions

1. **Model choice for item-only quests.** Sources 1 to 3 stay computed in code. For source 4 the model picks a level within the listed dropper range, which is a deliberate exception: no median, and no fixed formula. Confirm this exception, or have item-only quests become `"Unknown"` and leave the judgement to a person.
2. **Dropper cap.** The 10-dropper cap is a first guess, meant to keep common items such as Jellopy out. Check it on a sample of item-based quests before the full run.
3. **Override reliability.** Whether the model finds real requirements in dialogue, and does not invent them, is untested. Check a sample of overrides against the excerpts before the full run. Rejected overrides fall back to the derived value, so a bad override cannot silently change a level.
3. **Thinking mode.** Keep the model's reasoning on. Before the full run, compare a few quests with and without it and check whether the quality is acceptable.

## Not in this plan

- Quest rewards, prerequisites and the giver NPC as structured links. The text covers them only where the excerpts show them.
- Quests outside the 956 set, including the Merchant job-change quests 1009–1012 (filled by hand later).
- Item descriptions of any kind, including the client item description table (see Scope).
- Wiki text as input.
