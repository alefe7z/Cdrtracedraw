#!/bin/bash
# usage: appimage_offline_check.sh <CD Draw AppImage> <output folder>
# Runs the AppImage with no network at all (a fresh network namespace,
# where only a loopback that is down exists), under strace, uses it
# (new drawing, a rectangle, an ellipse, a text, undo and redo) and
# fails if
#   - the program is not running at the end, or
#   - it opened any socket other than a local Unix one (AF_UNIX: the X
#     server, D-Bus): no AF_INET, AF_INET6, AF_PACKET or anything else.
# Leaves a screenshot, the program's log and the strace log in the
# output folder. Needs: Xvfb, xdotool, strace, unshare (util-linux),
# ImageMagick's import. Run as root (unshare -n) or it stops.
set -u
APP=$(realpath "$1")
OUT=$(mkdir -p "$2" && realpath "$2")
DISPLAY_NUM=${OFFLINE_CHECK_DISPLAY:-:97}
export DISPLAY=$DISPLAY_NUM
FAIL=0

if ! unshare -n true 2>/dev/null; then
    echo "FAIL: unshare -n is not allowed here (run as root)"
    exit 1
fi

Xvfb "$DISPLAY_NUM" -screen 0 1600x1000x24 >/dev/null 2>&1 &
XPID=$!
sleep 1

# The AppImage is unpacked and run (no FUSE needed), with no network.
unshare -n env DISPLAY="$DISPLAY_NUM" WINIT_UNIX_BACKEND=x11 TRACEDRAW_NO_SPLASH=1 \
    LIBGL_ALWAYS_SOFTWARE=1 TRACEDRAW_CONFIG_DIR="$OUT/config" \
    strace -f -qq -e trace=socket,connect,bind -o "$OUT/strace.log" \
    "$APP" --appimage-extract-and-run >"$OUT/app.log" 2>&1 &
RUNNER=$!

# A slow drag with the left button, as a hand would make it.
drag() {
    xdotool mousemove "$1" "$2" mousedown 1
    for i in 1 2 3 4 5 6; do
        sleep 0.05
        xdotool mousemove $(($1 + ($3 - $1) * i / 6)) $(($2 + ($4 - $2) * i / 6))
    done
    sleep 0.1
    xdotool mouseup 1
    sleep 0.5
}

# Wait for the main window.
WID=""
for _ in $(seq 1 120); do
    sleep 0.5
    WID=$(xdotool search --name "CD Draw" 2>/dev/null | tail -n 1)
    [ -n "$WID" ] && break
done
if [ -z "$WID" ]; then
    echo "FAIL: no CD Draw window after 60 s"
    FAIL=1
else
    echo "window $WID: $(xdotool getwindowname "$WID")"
    sleep 3
    xdotool windowfocus --sync "$WID" 2>/dev/null || true
    # New drawing (Welcome screen > Ctrl+N > OK), then draw.
    xdotool mousemove 800 600 click 1 key ctrl+n
    sleep 1.5
    xdotool key Return
    sleep 1.5
    xdotool key F6
    drag 480 260 600 340
    xdotool key F7
    drag 640 260 760 340
    xdotool key F8 mousemove 470 450 click 1 type "Offline"
    xdotool key Escape
    sleep 0.5
    xdotool key ctrl+z ctrl+shift+z
    sleep 2
    import -window root "$OUT/screenshot.png" || true
fi

# Still running: its launcher has not exited and its window is there.
if kill -0 "$RUNNER" 2>/dev/null && xdotool search --name "CD Draw" >/dev/null 2>&1; then
    echo "program still running: ok"
else
    echo "FAIL: the program is not running"
    FAIL=1
fi
kill "$XPID" 2>/dev/null
sleep 2
kill "$RUNNER" 2>/dev/null
wait 2>/dev/null

if grep -qi panic "$OUT/app.log"; then
    echo "FAIL: panic in the log"
    FAIL=1
fi
echo "sockets opened (by address family):"
grep -o 'socket(AF_[A-Z0-9]*' "$OUT/strace.log" | sort | uniq -c || true
# Every network use starts with socket(); a connect or bind names its
# address family too.
other=$(grep -E 'socket\(AF_|sa_family=' "$OUT/strace.log" | grep -vE 'socket\(AF_UNIX|sa_family=AF_UNIX' || true)
if [ -n "$other" ]; then
    echo "FAIL: network use other than local Unix sockets:"
    echo "$other" | head -20
    FAIL=1
else
    echo "no network sockets: ok"
fi
exit $FAIL
