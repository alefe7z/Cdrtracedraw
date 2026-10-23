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
- Windows-only code: `rustup target add x86_64-pc-windows-msvc` then
  `cargo check -p tracedraw --target x86_64-pc-windows-msvc` (checks,
  does not link; must stay warning-free).

## Claims

One row per task in progress. Add yours before starting, remove it when
done.

| Agent | Area / files | Since | Note |
| --- | --- | --- | --- |

## Notes for other agents

Requests or warnings for whoever holds a claim, or for the next one.

- Codex reference fidelity pass: five previously unavailable entries now have
  functional integrations (Live Guides, Fit Objects to Path, Color Proof,
  Assets, Views). Catalog has 43 active reference rows; Comments, Effects
  and Macros remain in Window > Panels. Dedicated vector icons replace
  generic tool substitutions. Native validation remains blocked: the first Actions run stayed at native
  dependency installation; the bounded replacement run is queued/pending.
  Latest implementation: 71cf241 and 6188313. Validation run:
  https://github.com/alefe7z/Cdrtracedraw/actions/runs/40007487138
  No build/test/visual pass is claimed. Run the workflow or use an environment
  with Rust/Cargo to compile and inspect its rendered catalog before claiming
  visual identity. Details: docs/behavior/reference-panels.md.

- Codex selection flyout change (57030bb): checked by Claude on
  2026-10-23. It builds, the workspace tests (899) and clippy pass, and
  cargo fmt was applied (cedfe19). Only the visual check in the real app
  is still to do.
- Shortcuts (Claude, 2026-10-23): any key can now be a shortcut. Read
  `docs/behavior/shortcuts.md`, "Any key", before touching keys:
  - `shortcuts.rs` has `Code` = `Key` (egui) | `Extra` (keys egui does
    not report: Pause, PrintScreen, keypad, media...) | `Tap` (Shift,
    Ctrl, Alt, Win pressed alone: LShift, RCtrl...). Use `parse_code`,
    `format_code` and `same` for shortcut text; `parse` still returns
    egui keys only, so `interaction.rs` keeps working unchanged.
  - `extra_keys.rs` runs once a frame from `ui/mod.rs`, before
    `App::keyboard`: it reads the extra keys from Windows, detects taps,
    runs the user's shortcuts on them, and does the capture of
    Options > Customization > Commands (any key, Esc included; click the
    button again to stop waiting). `ui/options.rs` no longer captures.
  - `shortcuts::allowed` no longer refuses Esc, Tab, Enter, Backspace or
    Space alone.
  - Built-in default shortcuts (`tools.rs`, `interaction.rs`) were not
    changed.

## Log

Newest first, one line per finished task: date, agent, what, commit.

- 2026-10-23, Codex: reference fidelity pass, all five missing panels wired,
  dedicated pictograms, reference palette/geometry and stable popup anchor.
  Persistent local assets and named views, path rotation/range preferences.
  Static coverage/translation checks pass; native CI validation pending.
  Commits: 71cf241, 6188313. The current environment lacks Rust/Cargo and
  the Actions run has not reached compilation.

- 2026-10-23, Codex: implemented the grouped docker plus catalog from the
  owner's screenshot/video: compact checkboxes and vector icons, multi-toggle,
  tab icons/highlight and scrolling. All 41 existing panels remain reachable.
  Translation-key and catalog coverage checks pass; native validation pending.
  Details and limitations: docs/behavior/docker-catalog.md.

- 2026-10-23, Claude: any key as a shortcut (extra keys on Windows,
  modifier taps, Esc/Space/Enter alone), `extra_keys.rs`.
- 2026-10-23, Claude: verified and formatted Codex's selection flyout
  change (`cedfe19`).
- 2026-10-23, Codex: selection flyout and 1 / Shift+1 shortcuts
  (`57030bb`).

- 2026-10-23, Codex: implemented the two-row selection reference flyout,
  monochrome pointer/lasso icons, 1 and Shift+1 shortcuts, Space compatibility,
  Escape dismissal and regression tests. Compilation and visual validation
  are pending because Rust/Cargo and native display are unavailable.

- 2026-10-23, Claude: shared context, status file, sync scripts and
  hooks for agents on the same branch.
- 2026-10-23, Claude: Print Preview window like the reference (`6027201`).
- 2026-10-23, Claude: Print dialog with seven tabs, Windows printers
  (`52895ef`).
