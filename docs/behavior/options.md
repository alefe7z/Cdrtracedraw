# Options dialogs

Tools > Options (Ctrl+J) is one dialog, "Options", with six categories:
Document, CD Draw, Customization, Tools, Global and Workspaces. The
Tools > Options submenu opens it at a category; Layout > Document
Options (also a double-click on a ruler, the page border or its shadow)
opens it at Document. The layout follows the owner's third reference
video (`docs/reference/ui-reference.md`, "Options").

## Layout

- 802 x 676 px at 100 % interface size, centred, framed like every
  dialog (title bar with the title and the close button).
- Header row: the category's name in bold at the left; at the right the
  six category buttons (26 px, 6 px apart: a page, the program's mark,
  sliders, the pointer, a bulb, a screen), the current one filled with
  the accent colour, each with its name as tip. A click shows that
  category at its first page.
- The page list on the left (180 px; rows 25 px; the current page a grey
  bar, a lighter one under the pointer). The Tools list shows each
  tool's picture before its name. Document has Save as Default... under
  the list; Workspaces lists the workspaces instead of pages (a green
  check at the one in use) with Import... under the list.
- The page at the right in a framed panel; a page longer (or wider)
  than the panel scrolls, the dialog never grows.
- Sections: a bold title, a thin line between sections. Controls the
  reference shows that CD Draw has no feature for are greyed.
- Bottom row: the help button (24 x 24) at the left, OK (accent colour)
  and Cancel (80 x 24) at the right.
- Changes show at once. OK keeps them and writes the preferences; Cancel,
  the close button and Esc put back everything the dialog changed,
  including document settings (their commands are undone).

## CD Draw Options

| Page | Settings (defaults) |
|---|---|
| General | Getting started: On start-up (Welcome Screen, Start a new document, Open last edited document); Show New Document dialog box (on); Show the splash screen (on); Undo levels, Regular (150) |
| Tablet Mode | greyed (CD Draw has no tablet mode) |
| Display | Proof colors; Other controls in two columns: Show tooltips (on), Hide bounding box for curve tools (off), the rest greyed; Full-screen preview: Show page border (on); Navigation: default action for the mouse wheel (Zoom or Scroll), zoom rates greyed |
| Edit | Constrain angle (15 degrees); Drawing precision (3 decimal places); Visibility of inactive objects (20 %, Reset) and double-click for Focus mode |
| Nodes and Handles | Node size; node shape per type with Reset; Show curve direction; Enable node tracking (see `pick-tool.md`); pick distance; Colors: scheme Default or Custom (Default puts both colours back; the colours can be changed under Custom), main and secondary colour; Show highlight (unselected nodes filled, Ctrl+Shift+G) |
| Clip frames | Drag into a frame (greyed); Auto-center new content: when completely outside (default), always, never; Show lines in empty clip frames (on) |
| Bitmap Tracing | Quick trace method (one of the eight presets, Clipart); merging colours (greyed); what every trace starts with (below) |
| Snapping | Snap to objects, Snap to page, snapping radius (10 px), snap location marks and the screen tip; Modes: a framed list, each mode with its mark, Select All, Deselect All. Snapping to guidelines, the grids and pixels is on the Document pages |
| Save | Backup: make a backup on save, in the same folder or a chosen one; Auto-backup every 1 to 60 minutes (20), in the user's temporary folder or a chosen one (see `recovery.md`); saving older versions (greyed) |
| Text | Tabs Display (keyboard text increment, 1 pt), Paragraph Text (text frames, hyphenation, non-printing characters), Fonts (default font and size), Spelling (greyed), Quick Correct (correct while typing, capitals, typographic quotes) |
| Pen Settings, Macros, Plug-ins | greyed: CD Draw reads the pen as the system reports it, runs its own scripts and hosts no plug-ins |
| Warnings | the messages that show (below) |

Alignment and dynamic guides are set in their dockers (Window >
Dockers).

## Customization

Appearance (Center Dialog Boxes, interface size, theme, desktop
colour), Command Bars (the standard toolbar, property bar, toolbox,
status bar, colour palette, Text, Zoom and Transform toolbars, with
Reset; Lock Toolbars and the Outline flyout in the toolbox), Commands
(shortcuts of your own for every menu command and the tool keys; see
`shortcuts.md`), Color Palette (document palette, No Color well, right
mouse button; see `color-palettes.md`).

## Tools

Selection (cross hair cursor; treat all objects as filled, on; marquee
selects touched objects, off; the Ctrl and Shift choice greyed), Eraser
(thickness, round or square nib), Zoom, Pan (Zoom tool's secondary
button: zoom out or context menu; Calibrate Zoom... shows the ruler
calibration below), Freehand/Bézier (smoothing 0 to 100 with a slider,
auto-join 1 to 50 px; the Pen's preview, automatic add and delete, the
constrain key; see `curve-tools.md`), Smart Drawing (greyed),
Rectangle (corner style buttons, the four sizes with the lock between
them, Scale corners; see `shape-tools.md`), Ellipse (type, angles),
Graph Paper (columns, rows), Spiral (type with a picture, revolutions),
Table (rows, columns, what Tab does, tab order), Dimension and Angular
Dimension (precision, units, prefix, suffix of new dimensions), Connector
(rounded corner radius), Mesh Fill (columns, rows of new mesh fills).

## Global and Workspaces

Global > General: user interface language; Reset all settings... (asks
first, then puts every preference back to how CD Draw comes, keeping
the recent drawings, the last folder and the window's size; drawings are
not touched). Global > File Locations: a table of the auto-backup and
backup folders (Edit picks another, Reset the default) and where the
preferences are kept. Global > Color: greyed. Workspaces: pick one in the
list and Set as Current (a double-click does the same); Export... and
the parts exported and imported (see `workspaces.md`); Import... under
the list. Account and online pages (user ID, privacy) are left out: CD
Draw is offline.

### Printing

What the Print dialog starts with (`printing` in the preferences): Fit
to page, Print separations, Bleed, Crop marks, Registration marks, Colour
bars, Densitometer scale, Page information (all off), the sheet (fitted
to the pages) with Landscape, and the space between pages (5 mm). The
rest starts as always: one copy, all pages, 100 %, as in the document.

- Remember the settings of the last print (off): Print in the dialog
  keeps its choices above for the next time.
- Check the drawing in the Print dialog (on): the preflight summary and
  its "Preflight..." button; off, the dialog runs no checks.
- A sheet out of the list or a space that is not a number from a file
  falls back to the defaults (the space is kept within 0 to 100 mm).

### File Formats

What File > Export (Ctrl+E) starts with (`formats`): the format (SVG),
the resolution of bitmap formats (300 dpi, 1 to 2400), the JPEG and
WebP quality (90, 1 to 100), Transparent background (on), the PDF
standard (PDF 1.4), the PDF/X output condition (FOGRA39, at most 64
characters) and the PDF bleed (0 mm, 0 to 50). Remember the settings of
the last export (off): exporting keeps the dialog's choices for the next
time. Export for Web and Export for Office keep their own presets and
never change these.

Imported objects go (File > Import, Ctrl+I): where the file has them
(the default), or centred in the middle of the view; the move is part of
the import's undo step.

Saving .cdr files: the Version new drawings are saved in (`cdr_version`,
27 by default, 14 to 27; a value out of range from a file is brought
into it). Saving a `.cdr` file sets it to the version chosen there; a
drawing opened from a `.cdr` file keeps its own when CD Draw saves
that version (`files.md`).

### Bitmap Tracing

Quick Trace uses (`tracing`): one of the eight presets (Clipart by
default). Every trace starts with: Remove background (on) and Delete
original image (off); Quick Trace and the Trace dialog's presets take
them, and the dialog can still change them for one trace.

### Warnings

Show a message (`warnings`, all on):

- When objects of an opened file could not be read: after a `.cdr` file
  opens with objects left out, a message gives their number and a Report
  button that opens a new issue for the file.
- When an opened drawing uses fonts that are not installed: the message
  names them (the text shows in a similar font and keeps its font name).
- Before an item is removed from the Shelf: Remove or Cancel.

Both parts after opening share one message. Each message has "Do not show
this message again" (the Shelf's: "Do not ask again"), which turns off
the matching choice here; ticking it again brings the message back. A
message never replaces another dialog that is open.

## Interface size

Customization > Appearance > Interface size sets how large the menus,
bars, icons and dockers are drawn, relative to the scaling the system
gives the monitor the window is on: a slider and a field from 75 % to
250 % in steps of 5 (100 % the default, Reset to Default goes back to
it), applied when the slider is let go.

- 100 % is the size of the other programs on that monitor, and it
  follows the monitor: moving the window to a monitor the system scales
  differently rescales it. 200 % is twice that size.
- The interface's own measures are 13 px text, 20 px icons and 24 px
  rows. On Windows 100 % is a zoom of 12 / 13 over the system's scaling,
  so the text comes out as large as the system's own (Segoe UI 9 pt,
  12 px at 100 % scaling, 15 px at 125 %); at 0.8 it was 10.4 px at
  100 %, too small (#229).
- On Linux 100 % is a zoom of 1 over the system's scaling: one interface
  point per pixel at the system's scaling (#1). Elsewhere (macOS) it is
  0.8. The sizes offered and their order are the same.
- The size is a zoom factor over the monitor's own scaling, so text stays
  sharp. Values out of 50 % to 400 % count as 100 %.
- The interface size never changes how large drawings are: zoom 100 %
  shows them at their real size (see Ruler calibration) at any size.
- Preferences written before sizes were relative (`ui_scale`, an absolute
  size) are ignored: the size starts at 100 % (`ui_size`).
- Ctrl+plus and Ctrl+minus zoom the drawing, never the interface.

## Ruler calibration

Options > Tools > Zoom, Pan > Calibrate Zoom... sets the screen resolution in
pixels per inch at the system's scaling (96 by default, 48 to 400,
`screen_ppi` in the preferences) and draws a 100 mm ruler and a 4 inch
ruler at that resolution: hold a real ruler against the screen and
change the value until the marks match. Zoom 100% then shows a drawing
at its real size (the zoom box percentages follow; the Pixels view shows
one pixel at the system's scaling per document pixel). Interface points
per inch are that resolution over the interface's zoom (104 at 100 %
on Windows, 96 on Linux, 120 elsewhere), so the interface size never changes real sizes. Reset puts
96 back.

## Document Options

General (Preferences: Fill open curves, off: an open curve's fill shows
only in its closed subpaths, on screen, in hit testing and in vector
exports; Auto-inflate bitmaps; view mode; page view single or multipage;
duplicate offset), Page Size (with the rendering resolution and the
bleed), Layout, Background, Guidelines (tabs Display, Horizontal,
Vertical, Guidelines, Presets), Grid, Rulers (see
`rulers-grid-guidelines.md`), Save (greyed). Save as Default keeps the
grid, ruler and guideline settings for new drawings.

## Checks

- `ui::options::tests::categories_follow_the_reference`
- `ui::options::tests::every_page_belongs_to_one_dialog`
- `ui::options::tests::cancel_puts_settings_and_document_back`
- `ui::options::tests::every_page_draws`
- `ui::tests::interface_size_is_relative_to_the_system_scaling`
- `prefs::tests` (print, export and trace preferences; the dialogs
  starting with them; imports centred in the view; the messages after
  opening; the Shelf asking before removing)
- `ui::options_global::tests::the_global_pages_draw`
- `drawing_scale::tests::zoom_100_follows_the_ruler_calibration`
- `tracedraw_render` `open_curves_are_not_filled_unless_the_document_says_so`
- `tracedraw_io` `open_curves_lose_their_fill_in_vector_exports`
- `app::tests` start-up, backups and clip frame centring (below)

## Theme

Customization > Appearance > Color > Theme: Light (the default), Medium
Light, Medium Dark, Dark or High Contrast (`theme.rs`, one palette each). Panels, bars,
menus, dialogs, lists, rulers, tabs and icons take the theme's colours;
the page, the drawing, the accent blue and the desktop colour (its own
setting) do not. The button with a moon (light themes) or a sun (dark
themes) at the right end of the status bar switches to Dark or to
Light. The choice is kept in the preferences and applies at once.

## Coloured tool icons

Customization > Appearance > Coloured tool icons (off by default): the
toolbox, its flyouts and the Hints panel draw each tool in its group's
colour (pick blue, shape editing purple, crop red, zoom teal, curves
green, drawing orange, shapes light blue, text grey, dimensions slate,
connectors sea green, effects magenta, transparency indigo, eyedroppers
ochre, fills orange, outlines navy), 35 % lighter on dark themes. The
High Contrast theme keeps one icon colour.

## Desktop accessibility settings

Read once at start-up (`system_prefs.rs`, with the `reg` tool on
Windows; elsewhere nothing is read and the defaults apply):

- Text size (Windows Settings > Accessibility > Text size, the
  `TextScaleFactor` value, 100 to 225 %): with "Grow with the desktop
  text size" (Appearance, on by default) the interface size is
  multiplied by it, up to 400 %. The option shows the size read.
- High contrast (the `Flags` value of the HighContrast key, bit 1):
  with "Use high contrast while the desktop has a high contrast theme
  on" (on by default) the High Contrast theme is used whatever theme is
  picked. Its colours are the desktop's (`Control Panel\Colors`:
  Window, WindowText, Hilight, HilightText, ButtonFace, ButtonText,
  GrayText, HotTrackingColor): panels in the button face colour, fields
  and menus in the window colour, text in the button text colour,
  disabled text in the grey text colour, hovered and picked rows in the
  highlight colour with the highlight text colour on them, links and
  focus frames in the hot-track colour. Picked by hand, or when a colour
  cannot be read, the classic black theme's colours are used.

## Where the preferences are kept

- `TRACEDRAW_CONFIG_DIR` when it is set.
- Otherwise a folder called `tracedraw-settings` next to the program,
  when there is one: a portable copy (on a USB stick, say) keeps its
  preferences there and leaves nothing on the computer. The Windows
  portable zip ships the folder (with a note inside saying what it is
  for; `packaging/windows/portable`), so it is portable as unzipped;
  deleting the folder makes that copy use the system's place
  (`settings::tests::the_portable_zip_ships_the_settings_folder`).
- Otherwise the system's place: `%APPDATA%\TraceDraw` on Windows,
  `~/Library/Application Support/TraceDraw` on macOS,
  `$XDG_CONFIG_HOME/tracedraw` (or `~/.config/tracedraw`) elsewhere.

Menus, lists and dialogs appear at once, without fading in.

