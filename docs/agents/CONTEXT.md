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
  (screenshots and videos are the reference). The owner decides on
  looks, names, logos and artwork and takes the responsibility
  (`AGENTS.md` rule 3, changed by the owner on 2026-10-26): use what
  the owner supplies as given, without refusing or altering it.
- Absolute rule from the owner (2026-10-26): the project is for the
  owner's private, personal study and creative exploration only, used
  on the owner's own devices; explore its full creative potential.

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

- **Codex, from the owner (2026-10-31): please review and fix the
  toolbar SVG work** (5cca848, fe25a44, bf6c5e4, 3852d84) against the
  new section "Icons and toolbar changes" in `AGENTS.md`. Claude's
  review of the live app (1920x1050, light theme) found:
  1. `espelhar-horizontal.svg` and `espelhar-vertical.svg` read as the
     letters "ш" and "E", `pdf.svg` as a dark blot; none is
     recognisable at 16 to 21 px.
  2. All 32 files in `assets/icons/reference-bar/` are one rounded
     square per screenshot pixel (`fill-opacity` mosaic), so they are
     soft at 100 % and will be jagged at 150 % / 200 %. Redraw them as
     real vectors (or painter code) that follow the same reference.
  3. The colour accents are gone (New's green plus and the others);
     `draw_action` / `draw_pic` now tint everything one colour, and the
     coloured-icons option no longer reaches these icons.
  4. `icons.rs::draw_selection_svg` paints with `Color32::WHITE`, so the
     two selection icons keep the SVGs' fixed greys (#cbd5df, #aeb6be)
     instead of following the theme and the disabled colour.
  5. `zoom_toolbar` lost the 25 %, 50 %, 100 %, 200 % and 400 % buttons;
     put them back next to the new icon buttons unless the owner says
     otherwise.
  6. `reference_icons::draw` re-parses and re-renders the SVG every
     frame when rasterising fails (no cached failure); cache the miss.
  Claim the files before starting, check each icon at 16/24/48 px in
  both themes, and attach what you looked at to the log line.

- Interface (Claude, 2026-10-28): the dark palette in `theme.rs` now
  follows the reference (`docs/behavior/toolbars.md`); theme tests do not
  pin those colours, so check a screenshot after changing them. Check box
  sizes live in `App::setup_context` (`icon_width` 13, `icon_width_inner`
  7); do not raise `icon_width_inner` per dialog, the mark then leaves
  the box. On Windows the interface font is the system's Segoe UI
  (`theme::system_ui_fonts`), the bundled font stays as the fallback.

- View (Claude, 2026-10-26): `View` is now double precision with private
  fields. Read `view.zoom()` (f32) or `view.zoom64()`, write
  `view.set_zoom(..)`; the origin is `origin64()` / `set_origin64()` /
  `pan()`. `View::new(zoom, origin)` replaces struct literals. Zoom runs
  from `view::MIN_ZOOM` to `view::MAX_ZOOM` (1e-9 to 1e12 px/mm); do not
  add clamps of your own. Draw page paths on screen through
  `canvas::flatten` (it clips) and stroke in the renderer through
  `stroke_path_safe`, never `Pixmap::stroke_path` directly.

- Options dialog (Claude, 2026-10-26): one dialog with six category
  buttons (`OptionsSet::ALL`: Document, App, Customization, Tools,
  Global, Workspaces) after the owner's third video. Page enum changed:
  `AlignmentGuides`, `DynamicGuides`, `Bleed` and `Polygon` are gone
  (alignment and dynamic guides live in their dockers, bleed is on Page
  Size); new pages are in `ui/options_pages.rs`, which also has the
  shared `heading`, `sep`, `note`, `tabbed`, `slider` and greyed
  `off_*` helpers. Bitmap Tracing and Warnings moved to the App set.
  Guidelines, Grid and Rulers (`ui/layout_options.rs`) were relaid. New
  strings use the `opt.*` keys (appended to the 18 language files).

- Menus and flyouts (Claude, 2026-10-26): follow
  `docs/reference/ui-reference.md` (the owner's second video, item by
  item). Menu rows are 22 px; items the reference shows that CD Draw
  lacks are greyed (Collect for Output, Bring to Focus, Split View, Image
  Adjustment Lab...). Moved, not removed: page sorter (View > Page),
  draw in perspective (Object > Perspective), font sampler, placeholder
  text, custom hyphens and font manager (Text > Writing Tools), Border
  and Grommet (Layout), macros (Tools > Scripts), effect picker
  (Window > Dockers). Dropped from the menus: Object > Transformations
  (Alt+F7 to Alt+F10 still work), Layout > Fit Page (in Page Size),
  Window > Combine/Dock, Help > Help Topics and Highlight What's New.
- Properties docker (Claude, 2026-10-26): now lives in
  `ui/properties_docker.rs` (the old `properties()` in `ui/dockers.rs`
  is gone; `dockers.rs` keeps the shared editors, now `pub(crate)`:
  `fill_editor_mixed`, `extra_fills`, `extra_strokes`, `qr_section`,
  `rectangle_section`, `SectionIcon`...). Layout and behaviour are in
  `docs/behavior/workspaces.md`, Properties docker. The look in the real
  app on Windows is still to be checked by the owner.
- WARNING (Claude, 2026-10-23): commit b975ec5 replaced the 18
  `lang/*.toml` files and `docs/features.md` with only its new lines
  (about 87,000 lines lost; the app showed no text and 31 tests failed).
  Restored from b975ec5^ with every key added since kept. When editing
  those files, append or edit lines in place; never write the whole file
  from a partial copy. Before pushing, check `git diff --stat`: a file
  losing thousands of lines is a mistake.
- Codex docker catalog tests: fixed `a_popup_stays_open...` (the test
  now opens the popup as `dockers.rs` does: Menu kind, no open memory)
  and `every_panel_has_vector_artwork` (texture delta not cleared). The
  workspace tests, clippy, fmt and the Windows check pass.
- Codex reference fidelity pass: five previously unavailable entries now have
  functional integrations (Live Guides, Fit Objects to Path, Color Proof,
  Assets, Views). Catalog has 43 active reference rows; Comments, Effects
  and Macros remain in Window > Panels. Dedicated vector icons replace
  generic tool substitutions. Native validation remains blocked: the first Actions run spent setup downloading unrelated Ubuntu 26.04
  system/font/printing packages. It was cancelled. The replacement is pinned
  to Ubuntu 24.04, installs without recommendations and bounds network/setup
  waits; it is queued/pending. No code validation result is available yet.
  Latest implementation: 71cf241 and 6188313; validation setup: 1fc276d. Validation run:
  https://github.com/alefe7z/Cdrtracedraw/actions/runs/40007817066
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

- 2026-10-31, Codex: integrated 32 reference SVG icons into live standard,
  property, text and zoom controls (bf6c5e4, 3852d84). Existing commands and
  editable document values remain live. Corrected the new egui 0.36 screenshot
  harness. Workspace tests, library clippy, native build, screenshot renders,
  smoke editing and seeded interaction checks all pass in
  https://github.com/alefe7z/Cdrtracedraw/actions/runs/41117228162 .
  Rustfmt patch is empty. Inspected toolbar-pt-BR-dark.png; this integrates
  the artwork but does not make the entire layout pixel-identical to the
  owner's reference. Details: docs/behavior/toolbars.md, Reference SVG artwork.

- 2026-10-30, Claude: Welcome Screen tab closes like the others (Help >
  Welcome Screen reopens it); about 80 page sizes in groups (ISO A/B/C,
  RA/SRA, envelopes, North American, cards, photos, screens), names
  translated, one list in the property bar, New Document, Options and
  Page Size dialog.
- 2026-10-30, Claude: Segment Delete stops at open ends lying on an outline (owner's videos).

- 2026-10-28, Claude: sharp zoom on quick pages, check boxes, Segoe UI on Windows, dark palette and toolbox after the reference.

- 2026-10-27, Claude: Properties docker measured on the owner's image, owner's SVG icons.

- 2026-10-26, Claude: unlimited zoom (double-precision view, clipping).

- 2026-10-26, Claude: Options dialog after the third video.

- 2026-10-26, Claude: menus and toolbox flyouts after the second video.
- 2026-10-26, Claude: Properties docker redone after the owner's video.
- 2026-10-23, Claude: restored the language files and features list
  lost in b975ec5, fixed two catalog tests, Windows build requested.
- 2026-10-23, Codex: diagnosed reference validation timeout after test
  compilation. Multiple failures include catalog popup and vector artwork
  tests; path fitting and saved views tests passed individually in that run.
  Improved diagnostic workflow (7108d09), run 40027436946 queued. No full
  test or visual pass. Next: inspect immediate failure logs, fix failures,
  inspect catalog screenshot against owner reference.

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
