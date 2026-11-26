# Agents' status (generated)

Written by `scripts/update-context.sh` on every commit; do not
edit by hand. The hand-written context is `docs/agents/CONTEXT.md`.

- Branch: `claude/rebrand-cd-draw`
- Last commit before this one: `219939a7 Claim the v0.53.0 upstream merge and the bottom bar`
- Commits on the branch: 157

## Last 20 commits

- `219939a7` 2026-11-26 12:38 Claude: Claim the v0.53.0 upstream merge and the bottom bar
- `4396a68d` 2026-11-26 09:28 Codex: docs: hand off upstream update and segment delete to Claude
- `55fe6c5e` 2026-11-25 12:54 Alux7: Request installers [exe]
- `3d4ac694` 2026-11-13 07:23 Alux7: fix: match dark canvas page colours to the reference
- `a5248948` 2026-11-13 07:15 Alux7: docs: claim reference page appearance correction
- `fc518a5d` 2026-11-10 08:45 Claude: One workflow for every installer: exe, AppImage, deb, rpm
- `63040635` 2026-11-10 01:53 Claude: Request a Linux AppImage [appimage]
- `018e45ee` 2026-11-10 01:53 Claude: Linux AppImage on request, checked offline on Fedora
- `0558bc52` 2026-10-31 10:51 Claude: Build Windows exe [exe]
- `126cf34f` 2026-10-31 10:49 Claude: Light active tool icon on the lit blue; agent review lessons
- `de92ee4c` 2026-10-31 06:45 alefe7z: ui: fix all six toolbar icon review findings
- `5c335d2c` 2026-10-31 06:18 alefe7z: docs: claim fixes for Claude's toolbar icon review
- `5d428207` 2026-10-31 09:13 Claude: Add icon and toolbar rules for agents; ask Codex to fix the SVG icons
- `b6f10171` 2026-10-31 06:10 alefe7z: docs: record validated toolbar SVG integration and handoff
- `3852d84d` 2026-10-30 16:40 alefe7z: test: fix toolbar capture for egui 0.36 and apply rustfmt
- `bf6c5e47` 2026-10-30 16:35 alefe7z: ui: integrate reference SVG artwork into live toolbar controls

## Files changed in the last 10 commits

- `docs/agents/CONTEXT.md` (6)
- `scripts/request-appimage.sh` (2)
- `docs/linux-appimage.md` (2)
- `CLAUDE.md` (2)
- `.github/workflows/linux-appimage.yml` (2)
- `scripts/request-packages.sh` (1)
- `scripts/request-exe.sh` (1)
- `scripts/linux/offline_check.sh` (1)
- `scripts/linux/appimage_offline_check.sh` (1)
- `packaging/linux/tracedraw.spec` (1)
- `packaging/linux/build-rpm.sh` (1)
- `docs/reference/page-review-2026-11-13/dark.png` (1)
- `docs/packages.md` (1)
- `docs/behavior/toolbars.md` (1)
- `apps/tracedraw/src/theme.rs` (1)
- `apps/tracedraw/Cargo.toml` (1)
- `README.md` (1)
- `AGENTS.md` (1)
- `.github/workflows/windows-exe.yml` (1)
- `.github/workflows/packages.yml` (1)

## Open claims (from CONTEXT.md)

| Agent | Area / files | Since | Note |
| --- | --- | --- | --- |
| Claude | Merge of upstream `main` v0.53.0 into this branch (whole tree), then the bottom bar (page row, document palette: `ui/window_bars.rs`, `ui/palette.rs`) | 2026-11-26 | Owner's order. One merge commit, no rebase of pushed history; keep `3c3d92c4` open-end fix |

## Uncommitted when generated

    ?? docs/agents/STATUS.md.tmp
