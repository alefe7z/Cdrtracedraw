# CLAUDE.md

Read `AGENTS.md` first and follow it. It holds the project rules for every
AI agent and contributor. Rule 3: the owner decides on looks, names,
logos, artwork and assets, and takes responsibility for those choices.
`CLEANROOM.md` records where each piece of format knowledge came from.

Other agents work on the same branch: follow "Several agents on one
branch" in `AGENTS.md` and keep `docs/agents/CONTEXT.md` current.

## Installers on request (.exe, .AppImage, .deb, .rpm)

The owner usually asks for three or four formats at once. When they ask
for any of them, commit and push the work they want in it, then run
`scripts/request-packages.sh` with the formats asked for: `exe`,
`appimage`, `deb`, `rpm`, `linux` (the three Linux ones) or `all` (no
argument means all). It pushes an empty commit whose message carries
`[exe]`, `[rpm]` and so on; `.github/workflows/packages.yml` builds them
on GitHub. The Linux programs are built on Ubuntu 22.04; the `.rpm` is
made and installed on Fedora, the `.deb` installed on Ubuntu, the
AppImage run on Fedora, and each is checked with no network
(`scripts/linux/offline_check.sh`: it must draw, keep running and open
no socket but local Unix ones). Only when every check passes is one
pre-release published under a new `build-<date>-<commit>` tag with all
the files, SHA256SUMS, the screenshots and the checks' output in its
notes. Cloud sessions cannot push tags or reach the Actions API, so do
not try either; check the result with
`curl -s https://api.github.com/repos/alefe7z/Cdrtracedraw/releases`
(about 20 to 35 minutes) and give the user the release link and each
file's link. No release after 50 minutes means a job failed: say so, and
reproduce the failing step locally (`packaging/linux/build-rpm.sh`,
`cargo deb`, `packaging/linux/build-appimage.sh`, the offline check).
See `docs/packages.md`.
