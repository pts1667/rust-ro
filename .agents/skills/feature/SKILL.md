---
name: feature
description: Plan a rust-ro feature from a user-specified task file under doc/tasks/, with concrete changes grounded in existing code and reuse of the server architecture.
---

# Feature Planning

Create an actionable implementation plan from the supplied task file, preserving the repository's preference for reuse and consolidation.

## Input

Invoke as `$feature <task-file>` or provide a path under `doc/tasks/`. Resolve a filename relative to `doc/tasks/`, and a supplied `doc/tasks/...` path relative to the repository root. Treat the text following the skill name as the task input; skills do not expand Claude's `$ARGUMENTS` placeholder.

If no task file is supplied or the file is missing, ask for the intended task file instead of inventing requirements.

## Workflow

1. Read the repository's `AGENTS.md`, especially the packet handling and bitflag guidance, and the specified task file. Extract the requested behavior, design constraints, and acceptance criteria.
2. Inspect relevant existing code before proposing changes. Identify concrete files, services, events, models, repository methods, and tests that can be reused or extended. Stay within the repository's reading restrictions and the task-file exception.
3. Build the implementation plan from that analysis. Prefer extending existing services and components, consolidating duplicated logic, and refactoring over rewrites. Before proposing a new file, examine the relevant existing files and explain specifically why extending them cannot meet the requirement. Justify any proposed rewrite.
4. Describe the technical changes with specific file paths and symbols: API changes, event arguments, integration points, state ownership, persistence, and client notifications as applicable. Follow the message-passing architecture, keep main-loop work below the documented latency budget, and use only pre-renewal mechanics.
5. Specify tests and validation checkpoints against the acceptance criteria. Include migration or deployment considerations when the feature requires them. Apply the repository's comment policy to any proposed code snippets.

Complete the requirements analysis before the code analysis, and ground each subsequent step in the earlier findings. Flag missing information or conflicts between the task design and existing code.

## Deliverable

Provide the current behavior and reuse opportunities, an ordered implementation plan with concrete code changes, the reasons for any new files or rewrites, and a testing strategy. Reference the existing code supporting each recommendation rather than giving generic advice.

This workflow produces a plan. If the user also requests implementation, carry out the plan within that authorization.
