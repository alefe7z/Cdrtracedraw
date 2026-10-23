# Agents' status (generated)

Written by `scripts/update-context.sh` on every commit; do not
edit by hand. The hand-written context is `docs/agents/CONTEXT.md`.

- Branch: `claude/rebrand-cd-draw`
- Last commit before this one: `e8ec67b Claim the any-key shortcuts task`
- Commits on the branch: 22

## Last 20 commits

- `e8ec67b` 2026-10-23 18:06 Claude: Claim the any-key shortcuts task
- `57030bb` 2026-10-23 15:03 alefe7z: Match the selection flyout reference and selection shortcuts
- `9811073` 2026-10-23 15:02 alefe7z: Claim selection flyout reference task
- `29c5979` 2026-10-23 17:50 Claude: Leave the tree clean on a plain agent sync
- `da6f96f` 2026-10-23 17:39 Claude: Add a shared context and sync scripts for agents on one branch
- `6027201` 2026-10-23 17:27 Claude: Print preview: a window of its own like the reference
- `52895ef` 2026-10-23 17:12 Claude: Print dialog: seven tabs laid out like the reference
- `2154ed8` 2026-10-23 16:06 Claude: Request a Windows build [exe]
- `d87efb6` 2026-10-23 16:05 Claude: Use the new CD Draw logo for every icon
- `d6fb1b7` 2026-10-23 15:47 Claude: Rework the Curve flyout after the reference look
- `5e5d353` 2026-10-23 15:37 Claude: Redraw the Crop flyout icons after the reference look
- `7d32888` 2026-10-22 17:59 Claude: Allow following and naming other programs
- `8a2a324` 2026-10-22 17:49 Claude: Format the knife icon helper
- `74a6e9e` 2026-10-22 17:48 Claude: Reorder the Crop flyout and redraw the Knife, Segment Delete and Eraser icons
- `5201356` 2026-10-22 14:21 Claude: Make CD Draw desktop only and fully offline
- `12dc441` 2026-10-22 13:23 Claude: Request a Windows build [exe]
- `0098f43` 2026-10-22 13:23 Claude: Trigger the Windows build from an [exe] commit as well as a tag
- `fb3edcd` 2026-10-22 13:22 Claude: Add an on-request Windows build triggered by exe-* tags

## Files changed in the last 10 commits

- `docs/agents/CONTEXT.md` (4)
- `scripts/agent-sync.sh` (2)
- `apps/tracedraw/src/ui/print_preview.rs` (2)
- `apps/tracedraw/src/ui/print_dialog.rs` (2)
- `apps/tracedraw/src/ui/dialogs.rs` (2)
- `apps/tracedraw/lang/zh-TW.toml` (2)
- `apps/tracedraw/lang/zh-CN.toml` (2)
- `apps/tracedraw/lang/tr.toml` (2)
- `apps/tracedraw/lang/ru.toml` (2)
- `apps/tracedraw/lang/pt-BR.toml` (2)
- `apps/tracedraw/lang/pl.toml` (2)
- `apps/tracedraw/lang/nl.toml` (2)
- `apps/tracedraw/lang/ja.toml` (2)
- `apps/tracedraw/lang/it.toml` (2)
- `apps/tracedraw/lang/id.toml` (2)
- `apps/tracedraw/lang/hi.toml` (2)
- `apps/tracedraw/lang/fr.toml` (2)
- `apps/tracedraw/lang/es.toml` (2)
- `apps/tracedraw/lang/en.toml` (2)
- `apps/tracedraw/lang/de.toml` (2)
- `apps/tracedraw/lang/cs.toml` (2)
- `apps/tracedraw/lang/bn.toml` (2)
- `apps/tracedraw/lang/ar.toml` (2)
- `scripts/update-context.sh` (1)
- `scripts/agent-setup.sh` (1)
- `docs/features.md` (1)
- `docs/behavior/printing.md` (1)
- `docs/behavior/pick-tool.md` (1)
- `apps/tracedraw/src/ui/toolbox.rs` (1)
- `apps/tracedraw/src/ui/mod.rs` (1)

## Open claims (from CONTEXT.md)

| Agent | Area / files | Since | Note |
| --- | --- | --- | --- |
| Claude | Any key as a shortcut: shortcuts.rs, new extra_keys.rs, ui/options.rs (shortcut capture), ui/mod.rs (one call before app.keyboard), docs | 2026-10-23 | Leaves interaction.rs and tools.rs alone; new key kinds are dispatched from extra_keys.rs. Also checking Codex's selection flyout commit (build, tests) |

## Uncommitted when generated

    ?? docs/agents/STATUS.md.tmp
