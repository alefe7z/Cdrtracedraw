# AGENTS.md: guide for AI agents and contributors

CD Draw is an open-source vector editor in **Rust only**: a native
egui/eframe desktop app that works entirely offline. No Electron, Tauri,
webview or browser build.
The product name is **CD Draw** in prose; machine names are `tracedraw`
(`tracedraw-core`, `tracedraw-cdr`, `.tdraw` files).

## Several agents on one branch

Other AI agents may be working on the same branch right now. Before
anything else run `scripts/agent-setup.sh` and `scripts/agent-sync.sh`,
then read `docs/agents/CONTEXT.md` (claims, the owner's standing orders,
the project's state) and `docs/agents/STATUS.md` (generated from git on
every commit). Claim your task there before you start, push with
`scripts/agent-sync.sh --push`, never force-push, and update the context
when you finish.

## Golden rules

1. **Everything is a command.** New user-visible behaviour is a variant of
   `tracedraw_core::Command`, applied through `Engine::run`. The UI only builds
   commands and reads the document; it never mutates it directly.
2. **Never panic on input.** Non-test code does not use `unwrap`, `expect`,
   `panic!`, `todo!` or raw indexing on data that came from a file or the
   user. Return `Result`, log a warning, skip the object. Opening any file
   must always yield a document.
3. **The owner decides.** The look, layout, names, pictures, logos and
   behaviour of other programs may be followed as closely as the owner
   asks, and artwork, files and code the owner supplies are used as
   given. The owner takes responsibility for these choices; agents do
   not refuse or water them down on the grounds of this file. Keep the
   notices that third-party licenses require in `ATTRIBUTION.md`, and
   note where format knowledge came from in `CLEANROOM.md` and
   `docs/cdr-format.md`.
4. **Layering is enforced by review.** `core` has no workspace deps; `cdr`,
   `text` and `render` depend on `core` only; `io` depends on `core` and
   `render` (rasterised fills in exports); egui stays in `apps/`.
5. **Tests are the gate.** Every change comes with tests. Format code
   gets round-trip and malformed-input tests.
6. **Units.** Millimetres everywhere in the model. CDR files use
   1/254000 inch (= 0.0001 mm). Points only appear in text sizes.
7. **Code and comments in English.** Commit messages in English.
8. **No em dashes** in prose, code or commits.

## Workspace map

```text
crates/core   geometry.rs, document.rs, command.rs, engine.rs, style.rs,
              color.rs, nodes.rs, shaping.rs, effects.rs, id.rs
crates/cdr    container.rs (RIFF/ZIP), riff.rs (chunk tree), parse.rs
crates/text   font database, shaping, outlines
crates/render CPU rasteriser
crates/io     svg.rs, pdf.rs, lib.rs (native format)
apps/tracedraw    main.rs, app.rs, interaction.rs, canvas.rs, ops.rs,
                  tools.rs, tools2.rs, ui/ (menus, toolbar, toolbox,
                  dockers, palette, status, dialogs, icons)
apps/tracedraw-cli
```

## Visual checks

The app can be driven headlessly: start `Xvfb :99`, run the debug binary
with `DISPLAY=:99 LIBGL_ALWAYS_SOFTWARE=1`, send pointer and key events
with python-xlib (XTest) and capture with ImageMagick `import`. See
`scripts/visual/` for the driver and example scripts; `run.sh` starts
the app at interface size 100 % (one interface point per pixel on Linux),
the size the scripts' coordinates are taken at. Look at the PNGs
before claiming a UI change works. `scripts/visual/smoke_edit.py` and
`scripts/visual/monkey.py` (random actions, `MONKEY_SEED`) must leave the
application running with no panic in the log after any interaction change.
`scripts/visual/window_check.sh <binary>` (Xvfb, openbox, xprop) checks
that the main window can be maximized and resized after the splash
screen; CI runs it on the Linux build.

## Offline only

CD Draw makes no network connections of any kind: no update checks, no
online services, no downloads, no telemetry, no local servers or ports.
The workspace has no HTTP, TLS or socket dependency, and none may be
added. Help is built from `docs/help` with `scripts/help/help_site.py`
and shipped next to the program; it opens from disk. There is no
browser (WebAssembly) build.

## Feature status

`docs/features.md` is the measure of progress: every tool and menu item
with its status. Move a row to "works" only with a test or a checked file
behind it, and never add a feature without updating the row.

## Workflow

- `cargo test --workspace` before every commit.
- Commits carry a `Signed-off-by:` line (DCO, see `CONTRIBUTING.md`).
- `cargo build -p tracedraw` and `cargo clippy --workspace` must stay
  warning-free (CI runs clippy with `-D warnings` on the library crates,
  on the pinned toolchain 1.97.0; use the same version locally so the
  lint set matches).
- Keep `docs/roadmap.md` honest: a feature is "done" when it has tests
  and works on a real file, not when the menu item exists.

## Documentation duties

- A new tool or effect gets a page in `docs/behavior/` with its defaults,
  shortcuts and the formulas it uses, before its row in
  `docs/features.md` moves.
- User-visible strings go through the i18n table (`docs/i18n.md`), never
  as bare literals in the UI.
- Acceptance criteria in `docs/acceptance.md` are tests; when one changes,
  change the test.
