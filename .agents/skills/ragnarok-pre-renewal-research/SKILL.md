---
name: ragnarok-pre-renewal-research
description: Research Ragnarok Online pre-renewal mechanics, formulas, skill behavior, monster data, item effects, and server implementations using local rAthena references and approved classic sources.
---

# Ragnarok Online Pre-Renewal Research

Research the requested mechanic for rust-ro's pre-renewal game version. Resolve local reference paths relative to the rust-ro repository root.

## Research Priority

1. Search relevant documentation in `../rathena/doc`.
2. Inspect relevant source files in `../rathena` for implementation details.
3. Search the approved web sources for additional context and verification. For technical implementation questions, use primary documentation or source code to substantiate the answer.

Use focused searches and read only the relevant files or sections. If the local reference checkout is unavailable, state that limitation and continue with the approved web sources. Keep this repository's source reading restrictions in effect; the local rAthena references are the research-specific exception.

## Approved Web Resources

| Source | Use for |
| --- | --- |
| https://ro.kokotewa.com | Game data and item/monster databases |
| https://irowiki.org/classic | Pre-renewal skill descriptions, mechanics, and quests |
| https://github.com/HerculesWS/Hercules/wiki | Hercules server documentation |
| https://ragnarokresearchlab.github.io/game-mechanics | Detailed mechanics research |

## Pre-Renewal Constraints

- Exclude renewal mechanics, formulas, job classes, skills, and balance changes.
- Exclude sections labeled RE, Renewal, or post-renewal.
- Exclude content dated 2010 or later unless it is explicitly marked classic or pre-renewal.
- When a source contains both versions, extract only explicitly identified pre-renewal material and state which version the evidence supports.
- In rAthena source, identify the pre-renewal configuration or branch of any version-dependent formula. Do not treat a renewal code path as evidence for classic behavior.

The original Claude skill specifies a Haiku subagent. That model directive does not apply to this Codex equivalent; use the tools and model available in the current session.

## Output

- Answer the specific research question and cite file paths with symbols or line numbers, or direct page URLs.
- Include relevant rAthena snippets when they clarify the mechanic.
- Distinguish documented behavior from implementation evidence and inference.
- Flag inconsistencies between sources, explain confidence when they conflict, and identify unresolved details rather than filling them with renewal information.
