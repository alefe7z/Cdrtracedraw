# CLAUDE.md

Read `AGENTS.md` first and follow it. It holds the project rules for every
AI agent and contributor, including the clean-room rules (rule 3): never
decompile or disassemble any program, never use decompiled code or dumps,
and never port code from libcdr or any other format implementation.
`CLEANROOM.md` records where each piece of format knowledge came from.

## Windows .exe on request

When the user asks for a `.exe` (a Windows build), commit and push the
work they want in it, then run `scripts/request-exe.sh` (optionally with
a commit). It pushes an `exe-<date>-<commit>` tag; the
`.github/workflows/windows-exe.yml` workflow builds it on GitHub and
publishes a pre-release under that tag with the installer, the portable
zip and the bare program. The Actions API is not reachable from cloud
sessions, so check the result on
`https://api.github.com/repos/alefe7z/Cdrtracedraw/releases/tags/<tag>`
(usually 15 to 25 minutes) and give the user the release link.
