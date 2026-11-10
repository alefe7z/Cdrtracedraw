# Installers

`.github/workflows/packages.yml` makes the installers on request:
`scripts/request-packages.sh exe appimage deb rpm` (or `linux`, `all`),
a commit message carrying `[exe]`, `[appimage]`, `[deb]`, `[rpm]`,
`[linux]` or `[all]`, or by hand from the Actions tab. One pre-release
under a `build-<date>-<commit>` tag holds every format asked for:

| Format | File | Made on | Checked on |
| --- | --- | --- | --- |
| Windows | `CD-Draw-<v>-windows-x86_64-setup.exe`, `-portable.zip`, `.exe` | Windows | (builds only) |
| AppImage | `CD-Draw-<v>-linux-x86_64.AppImage` | Ubuntu 22.04 | Fedora, run with no network |
| Debian / Ubuntu | `CD-Draw-<v>-linux-x86_64.deb` (`cargo deb`) | Ubuntu 22.04 | Ubuntu 24.04, installed with apt, run with no network |
| Fedora | `CD-Draw-<v>-linux-x86_64.rpm` (`packaging/linux/build-rpm.sh`, `tracedraw.spec`) | Fedora | Fedora, installed with dnf, run with no network |

The Linux programs need glibc 2.35 or later: Fedora 36+, Debian 12+,
Ubuntu 22.04+. The `.deb` and `.rpm` install `tracedraw` and
`tracedraw-cli` in `/usr/bin`, the Help in `/usr/share/tracedraw/help`,
the menu entry and the icon, and depend on the window system libraries
the program loads when it starts.

## The AppImage

- `usr/bin/tracedraw` and `usr/bin/tracedraw-cli`, built on Ubuntu 22.04.
  They link only glibc (2.35 or later), libm and libgcc_s; the window
  system libraries (libX11, libxcb, libxkbcommon, libwayland-client,
  libEGL/libGL) and libdbus are loaded from the system at start, as every
  Linux desktop has them. Fedora 36 and later work.
- `usr/share/tracedraw/help`: the Help, opened from disk in the browser.
- The desktop file and icon. No update information is embedded, so
  AppImage tools never look for updates.
- The static AppImage runtime (type2-runtime): it needs `fusermount` or
  `fusermount3`, not the old libfuse2. Without FUSE, run it with
  `--appimage-extract-and-run`.

## Using it on Fedora

```sh
chmod +x CD-Draw-*.AppImage
./CD-Draw-*.AppImage
```

## Offline

CD Draw makes no network connection (see "Offline only" in `AGENTS.md`):
the workspace has no HTTP, TLS, socket or DNS dependency, the code uses
no network API, and the Help has no remote resource.
`scripts/linux/offline_check.sh` proves it on every Linux package:
it runs the AppImage, or the program a package installed, in a network namespace with no network, under
strace, draws a rectangle, an ellipse and a text, and fails unless the
program is still running and every socket it opened is a local Unix one
(the X server, D-Bus). The workflow runs it for each package before
publishing and puts the output and a screenshot in the release.

The program only starts other programs when asked: `xdg-open` for the
Help and for a file to show, `lp`/`lpstat` for printing, and the system
file dialog (xdg-desktop-portal, or zenity). Printing to a network
printer goes through the system's CUPS, outside CD Draw.
