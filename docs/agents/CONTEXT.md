# Shared context for AI agents

More than one AI agent (and the owner) work on this repository at the
same time, often on the **same branch**. This file is the shared memory
between them: read it before starting, update it when you start, finish
or learn something the next agent needs. `docs/agents/STATUS.md` next to
it is generated from git on every commit (last commits, files touched,
open claims); do not edit that one by hand.

## Start of every session

1. `scripts/agent-setup.sh` (once per clone: hooks, rebase on pull,
   no conflicts on STATUS.md).
2. `scripts/agent-sync.sh` (fetch and rebase on the shared branch).
3. Read `AGENTS.md` (the project rules), then this file and `STATUS.md`.
4. Before touching code, add a row to **Claims** below, commit it alone
   and push it at once (`scripts/agent-sync.sh --push`), so the others
   see what you took.

## Working on the same branch

- Small commits, pushed often: `scripts/agent-sync.sh --push` rebases on
  what others pushed and retries a rejected push.
- Never `git push --force`, never rewrite pushed history, never merge
  commits (`pull.rebase` is on): rebase only.
- Sync before starting a task and before every push. If the rebase
  stops on a conflict in a file another agent claimed, keep their side,
  re-apply your change on top, and say so in the commit message.
- Stay inside your claim. To change a file someone else claimed, add a
  note in their row under **Notes for other agents** instead, or wait
  for their claim to close.
- `lang/*.toml` (18 files) and `docs/features.md` are touched by almost
  every change: append new keys at the end of the files, keep edits
  small, and rebase right before pushing to keep conflicts rare.
- When you finish: remove your claim, add a line to **Log**, update
  **State** if something changed for everyone, then push.
- Commit messages in English, signed off (`git commit -s`), with your
  own attribution trailer if your tool adds one. No em dashes.

## The owner

- Owner: alefe7z (GitHub). Writes in **Portuguese (pt-BR)**: answer in
  Portuguese.
- Standing order: **do not change anything until the owner gives the
  order**; they explain first, then ask. Questions get answers, not
  edits.
- When the owner asks for a Windows `.exe`: commit and push, run
  `scripts/request-exe.sh`, then give the release link from
  `https://api.github.com/repos/alefe7z/Cdrtracedraw/releases` (build
  takes 15 to 25 min). Cloud sessions cannot push tags.
- The owner wants the interface to look and behave like CorelDRAW
  (screenshots and videos are the reference) while staying clean-room
  (`AGENTS.md` rule 3): copy the look, never code, files or icons.

## State

- Product: **CD Draw**, fork of TraceDraw, repo
  `github.com/alefe7z/Cdrtracedraw`. Shared branch:
  `claude/rebrand-cd-draw` (no pull request yet).
- Desktop only and fully offline: no network code, no web build, no AI,
  no updates (`AGENTS.md`, "Offline only"). Help is installed with the
  program and opened from disk.
- Done lately: rebrand; `.exe` on request (`windows-exe.yml`); offline
  Help; Crop and Curve toolbox flyouts like the reference; new logo
  (`assets/icon/cd-draw-logo.svg`); Print dialog with seven tabs
  (`ui/print_dialog.rs`); Print Preview window (`ui/print_preview.rs`);
  Windows printers (`printers.rs`).
- Known gaps: several Print dialog options are kept but do not change
  the output yet (list in `docs/behavior/printing.md`, end of "Print
  dialog"). Windows printing and printer listing are untested on a real
  Windows machine.

## Useful commands

- Checks before a commit: `cargo fmt`, `cargo test --workspace`,
  `cargo clippy --workspace --exclude tracedraw -- -D warnings`,
  `cargo build -p tracedraw` warning-free.
- Screenshots of the real app: `scripts/visual/run.sh <script.py>`
  (Xvfb; helpers in `scripts/visual/drive.py`). Dark theme and pt-BR:
  put `{"theme":"dark","language":"pt-BR"}` in
  `$TRACEDRAW_SHOTS/config/settings.json` and run with
  `TRACEDRAW_KEEP_SETTINGS=1`. Language files are compiled in: rebuild
  after editing `lang/*.toml`.

## Claims

One row per task in progress. Add yours before starting, remove it when
done.

| Agent | Area / files | Since | Note |
| --- | --- | --- | --- |
| Codex | Selection flyout, icons and shortcuts | 2026-10-23 | Owner screenshot; toolbox.rs, icons.rs, tools.rs, interaction.rs, pick_drag_tests.rs |

## Notes for other agents

Requests or warnings for whoever holds a claim, or for the next one.

- (none)

## Log

Newest first, one line per finished task: date, agent, what, commit.

- 2026-10-23, Claude: shared context, status file, sync scripts and
  hooks for agents on the same branch.
- 2026-10-23, Claude: Print Preview window like the reference (`6027201`).
- 2026-10-23, Claude: Print dialog with seven tabs, Windows printers
  (`52895ef`).
