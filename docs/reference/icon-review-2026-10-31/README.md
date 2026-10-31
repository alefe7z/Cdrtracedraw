# Toolbar icon review, 2026-10-31

This addresses Claude's six findings recorded in `docs/agents/CONTEXT.md`.

- All 32 screenshot-derived opacity mosaics were replaced with hand-drawn
  paths, arcs and shapes on a 24-unit grid. Mirror controls show reflection
  axes and arrows. PDF has real outlined letters and an unfilled badge.
- Monochrome artwork uses the caller's colour. Optional green/cyan/red
  accents follow the coloured-icons setting; disabled controls stay dimmed.
- Both selection SVGs are white coverage masks, tinted by the caller.
- Zoom retains 25 %, 50 %, 100 %, 200 % and 400 % buttons beside the list
  and the icon buttons.
- Both SVG paths cache failed rasterization as well as successful textures.

## Captures inspected

Every `icons-<theme>-<size>-colored-<flag>.png` shows all 32 toolbar icons
and both selection tools at native 16, 24 or 48 px. The left icon in each
pair is enabled; the right one uses the disabled theme colour. Both light
and dark themes were checked with coloured icons on and off (12 sheets).

`review-4x-light-false.png` and `review-4x-dark-true.png` enlarge the native
16 px samples by nearest-neighbour for inspecting the coverage. These are
inspection enlargements, not the vector geometry scaled to a larger size.

`native-light.png` and `native-dark.png` are captures of the running native
application under Xvfb at 1920 x 1050. `native-bar-4x-*.png` are 4x crops
of the changed controls. The New accent, PDF letters, mirror axes, caller
tint and restored presets were inspected in both. `text-and-zoom-dark.png`
is the software-rendered real UI with text selected and Zoom shown.

## Local validation

Rust 1.97.0, before publishing runtime changes:

- `cargo fmt --all -- --check`: pass.
- `cargo test --workspace -- --test-threads=1`: 1,655 passed, 9 ignored.
- `cargo clippy -p tracedraw -- -D warnings`: pass.
- `cargo clippy --workspace -- -D warnings`: pass.
- `cargo build -p tracedraw`: pass.
- `reference_icon_matrix_shots` and `reference_catalog_shot_toolbar`,
  explicitly run with `--ignored --test-threads=1`: pass.
- Native `smoke_edit.py`: each editing stage left the app running.
- Native `monkey.py`, seed 23, 100 actions: survived, no panic in the log.

The checks used `RUSTFLAGS='-Ccodegen-units=1 -Cdebuginfo=0'` and
`CARGO_INCREMENTAL=0` after invalid intermediate compiler artifacts were
removed. Liberation Sans was installed to match the expected Arial
substitute; the first run's existing paragraph-frame test failed with
Nimbus Sans, then passed unchanged with Liberation Sans. No text-engine
code or tests were modified for that environment difference.

These fixes improve the reference-based icons. They do not claim the
entire program layout is pixel-identical to the reference application.
