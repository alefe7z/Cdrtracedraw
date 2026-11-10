# Linux AppImage

`.github/workflows/linux-appimage.yml` makes `CD-Draw-<version>-linux-x86_64.AppImage`
on request (`scripts/request-appimage.sh`, a commit message with
`[appimage]`, or by hand from the Actions tab).

## What is inside

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
`scripts/linux/appimage_offline_check.sh` proves it on the built file:
it runs the AppImage in a network namespace with no network, under
strace, draws a rectangle, an ellipse and a text, and fails unless the
program is still running and every socket it opened is a local Unix one
(the X server, D-Bus). The workflow runs it on Fedora before publishing
and attaches the output and a screenshot to the release.

The program only starts other programs when asked: `xdg-open` for the
Help and for a file to show, `lp`/`lpstat` for printing, and the system
file dialog (xdg-desktop-portal, or zenity). Printing to a network
printer goes through the system's CUPS, outside CD Draw.
