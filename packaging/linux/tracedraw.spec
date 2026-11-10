# The .rpm of CD Draw, from programs built beforehand (see
# packaging/linux/build-rpm.sh): Source0 holds tracedraw, tracedraw-cli,
# help/, the desktop file, the icon and the licenses.
%global debug_package %{nil}
%global _build_id_links none

Name:           tracedraw
Version:        %{pkg_version}
Release:        %{pkg_release}%{?dist}
Summary:        CD Draw, a vector illustration editor that works offline
License:        MIT OR Apache-2.0
URL:            https://github.com/alefe7z/Cdrtracedraw
Source0:        tracedraw-files.tar.gz
ExclusiveArch:  x86_64
# Loaded when the program starts, so rpm cannot find them by itself.
Requires:       libxkbcommon libxkbcommon-x11 libX11 libX11-xcb libXcursor libXi libxcb
Requires:       mesa-libGL mesa-libEGL libwayland-client libwayland-egl dbus-libs
Requires:       hicolor-icon-theme
Recommends:     xdg-utils

%description
CD Draw is a vector illustration editor with a classic desktop workflow
and a .cdr reader. It makes no network connection of any kind; its Help
is installed with it and opens from disk.

%prep
%setup -q -c

%install
install -Dm755 tracedraw %{buildroot}%{_bindir}/tracedraw
install -Dm755 tracedraw-cli %{buildroot}%{_bindir}/tracedraw-cli
install -Dm644 tracedraw.desktop %{buildroot}%{_datadir}/applications/tracedraw.desktop
install -Dm644 tracedraw.png %{buildroot}%{_datadir}/icons/hicolor/256x256/apps/tracedraw.png
mkdir -p %{buildroot}%{_datadir}/tracedraw
cp -r help %{buildroot}%{_datadir}/tracedraw/help

%files
%license LICENSE-MIT LICENSE-APACHE
%{_bindir}/tracedraw
%{_bindir}/tracedraw-cli
%{_datadir}/applications/tracedraw.desktop
%{_datadir}/icons/hicolor/256x256/apps/tracedraw.png
%{_datadir}/tracedraw
