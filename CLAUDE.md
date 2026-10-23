# CLAUDE.md

Read `AGENTS.md` first and follow it. It holds the project rules for every
AI agent and contributor, including the clean-room rules (rule 3): never
decompile or disassemble any program, never use decompiled code or dumps,
and never port code from libcdr or any other format implementation.
`CLEANROOM.md` records where each piece of format knowledge came from.

Other agents work on the same branch: follow "Several agents on one
branch" in `AGENTS.md` and keep `docs/agents/CONTEXT.md` current.

## Windows .exe on request

When the user asks for a `.exe` (a Windows build), commit and push the
work they want in it, then run `scripts/request-exe.sh` on the session's
branch. It pushes an empty commit whose message contains `[exe]`; the
`.github/workflows/windows-exe.yml` workflow builds it on GitHub and
publishes a pre-release under a new `exe-<date>-<commit>` tag with the
installer, the portable zip and the bare program. Cloud sessions cannot
push tags or reach the Actions API, so do not try either; check the
result with
`curl -s https://api.github.com/repos/alefe7z/Cdrtracedraw/releases`
(the build takes about 15 to 25 minutes) and give the user the release
link.
