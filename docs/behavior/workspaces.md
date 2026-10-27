# Workspaces

A workspace is how the window is set up: which bars show and where they
sit, the toolbox, keyboard shortcuts, the colour palette options and the
interface's look. Window > Workspace picks a built-in one (Default, Lite,
Classic, Illustration, Page Layout, Touch); the user's own setup can be
kept in a file and brought back.

## Touch

For touch screens: the interface (bars, buttons, panels, menus and text)
is drawn `TOUCH_SCALE` = 1.4 times the interface size chosen (at most
400 %), and drawings keep their real size at zoom 100 % (the points per
inch shrink by as much). The standard toolbar, property bar and toolbox
show; rulers, panels and the status bar do not. On the drawing two
fingers pan and a pinch zooms about its centre, with any tool; the
gesture drops what the first finger had begun, so it neither drags an
object nor draws a marquee. Picking another workspace brings the
interface back to the size chosen.
Checks: `ui::tests::the_touch_workspace_is_larger_and_keeps_real_sizes`,
`pick_drag_tests::two_fingers_pan_without_dragging_anything`.

## Kept across restarts

Everything below is part of the preferences (`settings.json` in the
configuration folder) and comes back when CD Draw starts again: every page of Options, where
each bar sits (`bars`), which bars show (`shown_bars`: standard toolbar,
property bar, toolbox, status bar, panels, rulers, Text, Zoom and
Transform toolbars), the tool each toolbox group shows, hidden flyouts,
shortcuts, palette options, interface size and desktop colour, the
dockers (`dockers`: the dockers in the tab column, in order, and the one
in front), the units, nudge and duplicate distances, snapping (`snap`:
what it snaps to, alignment and dynamic guides, snapping off), the tool
settings new objects take (`tools`: polygon and star points, sharpness,
rectangle corners, ellipse arcs, spiral revolutions, table and graph
paper rows and columns, blend and contour steps, contour offset, eraser
width and shape, text font, size and hyphenation), what the window
shows (`view`: grid, guidelines, page border, bleed, printable area,
pixel and baseline grids, objects that will not print, proof colours,
overprints, the view mode) and the dockers' own choices (`panels`: the
Color docker's model and views, Align To, Use Outline, Distribute To).

The preferences are saved whenever they change, looked at once a second
(so a change lasts even when CD Draw does not close normally), on
Options' OK, and
when CD Draw closes; not while Options is open, whose Cancel puts them
back. Values out of range in a file edited by hand are brought into
range, and names CD Draw does not know fall back to the defaults.
Preferences written before the bars shown were kept have no
`shown_bars`: the built-in workspace's bars show, as before; without
`dockers` the column holds Hints, Properties and Objects; without
`tools`, `view` or `panels` they start as on a first start. Picking a
built-in workspace sets its own bars, as it always did.
Checks: `app::tests::tools_view_snapping_and_dockers_survive_a_restart`,
`app::tests::preferences_are_saved_when_they_change`.

## Dockers

The tab column holds the open dockers, as tabs; a click on a tab brings
its docker to the front, a click on the tab in front collapses the
docker area to the column. The docker's close button (x) and its tab's
"Close Docker" take the docker out of the column and bring the next one
forward; with none left the docker area closes. Collapse (the two
triangles) only folds the area. A docker opened from a menu or its
shortcut joins the column; Help > Hints opens Hints and, ticked, closes
it. The column is kept when CD Draw closes, so a docker closed stays
closed the next time.
Checks: `app::tests::a_closed_docker_stays_closed_the_next_time`.

## Properties docker

Laid out after the owner's reference video (`ui/properties_docker.rs`).

- A row of section buttons stays above what scrolls: the outline pen,
  the fill bucket, the transparency checker, then the selected object's
  own (QR code first, Rectangle, "fx", Character, Summary "i"). A button
  opens its section, scrolls smoothly to it and stays lit. At the right:
  make the selection's outline and fill the default for new objects (a
  tray with an arrow), and Show only the section picked (a page with a
  crossed eye).
- Each section has a "^ Title" header (bold, no bar); a click closes it
  ("v") or opens it. Closed sections are kept in the preferences
  (`properties_closed`). F11 and the status bar's fill and outline go to
  their section.
- Measured on the owner's reference (dark theme): the docker's
  background grey 38, its title bold; the section buttons 24 points, 8
  apart, 3 points from the docker's edge, the outline, fill and
  transparency ones drawn from the owner's SVG files
  (`assets/icons/properties/`, `ui/prop_icons.rs`), the one in use on a
  blue (#1C4670) rounded square. Section titles bold 12 points, text 12
  points in grey #CACED4. Bodies start 6 points from the edge, rows 23
  points tall and 9 apart; lists filled grey 60 with round corners and no
  frame, a box typed in (the width) on the docker's grey with a grey line;
  lists end in a thin "v"; lines between groups grey.
- Outline: the colour (a list's frame with the swatch; no outline shows
  a crossed white swatch), the width box with its list (None, Hairline,
  0.1 to 10 mm), shown as "0,2 mm" (the language's decimal sign, the
  unit after it, no spin arrows), and the unit by name, the line style list drawn as lines
  with "..." for Edit Line Style, three dash fittings (as they come,
  fitted to corners and ends, set in Edit Line Style; greyed for a solid
  line). A small arrow (closed at first) hides or shows the lower rows, their labels
  right-aligned: Miter limit, Corners, Line caps and Position as
  pictures. Then Arrowheads (start and end lists, "..." to swap or
  remove, Share attributes sets both), Variable Outline (a ruler with
  the width nodes, the picked node's position, sides and total; greyed
  without nodes; the flame picks the Variable Outline tool) and
  Calligraphy (stretch in percent, angle, the nib drawn, reset; Behind
  fill, Scale with object; Overprint outline greyed, it is set when
  printing). Further outlines follow. With nothing selected the section
  stops after the arrow: arrowheads, variable outline and calligraphy
  belong to objects.
- Fill: the kinds as pictures (a crossed box, a framed square, a
  gradient, a grid, rings, a mesh) and their settings; with no fill, "The
  default has no fill." (nothing selected) or "The object has no fill.".
  Further fills follow.
- Transparency: the merge mode across the section, the kinds as
  pictures (none with a red dot, uniform, fountain, pattern, texture),
  then the amount or the fountain's angle and what it applies to.
- With nothing selected the Outline, Fill and Transparency sections set
  what new objects (and the Transparency tool) start with.

The docker keeps the width it has (340 points by default, or what the
edge was dragged to): selecting an object never widens it, and its
contents are laid out within it and clipped to it.
Checks: `ui::properties_docker::tests`,
`ui::dockers::tests::properties_sections_open_close_and_jump`,
`ui::dockers::tests::properties_sections_fit_the_default_docker_width`;
every kind's sections draw in
`ui::mod::tests::window_draws_with_every_docker_tool_and_selection`.

## Export and import

Options > Workspaces > Workspace > Export and import, or Window >
Workspace > Export Workspace... and Import Workspace...

| Part | What it holds |
|---|---|
| Keyboard shortcuts | the user's shortcuts for menu commands and tools (`Settings::shortcuts`) |
| Toolbars, toolbox and status bar | the built-in workspace it starts from, where the bars sit and whether they are locked, which bars show, the tool each toolbox group shows, hidden flyouts, the Outline flyout, what the status bar's first field shows |
| Color palette options | No Color well, right mouse button, document palette, the palette shown, colours joining the document palette |
| Appearance | interface size, desktop colour, page border |

- The options page has a check box per part (remembered in
  `workspace_parts`); Export writes the parts checked, Import applies the
  parts checked that the file has. The Window menu items use every part.
- An import from the options is part of the options: Cancel puts the
  previous setup back, OK keeps it. From the Window menu it is kept and
  saved at once.
- The status bar names the file and the parts applied, or says the file
  is not a workspace file or has none of the parts chosen.

## The file

JSON with the extension `.tdws`; parts not exported are left out.

```json
{
  "kind": "tracedraw-workspace",
  "version": 1,
  "base": "default",
  "shortcuts": [["menu.file.export", "Ctrl+Shift+X"], ["pick", "Q"]],
  "bars": {
    "layout": { "toolbox": "right", "palette": "bottom", "locked": true },
    "shown": { "status_bar": false, "zoom_toolbar": true },
    "toolbox": ["pick", "shape"],
    "toolbox_hidden": ["eraser"],
    "show_outline_flyout": false,
    "status_info": "CursorCoordinates"
  },
  "palette": { "show_no_color": true, "right_click_outline": true },
  "appearance": { "ui_size": 1.0, "desktop_rgb": [255, 255, 255], "page_border": true }
}
```

A file is never trusted:

- More than 4 MB, not JSON, a part of the wrong shape, another `kind`
  or version 0: refused, nothing changes. Later versions are read as far
  as their parts are understood; unknown fields are ignored.
- Shortcuts with an empty or overlong (over 64 characters) id or keys
  are dropped; the last keys given for an id win; at most 4096 are kept.
- Floating bar positions that are not finite or lie beyond 100000 points
  fall back to the default layout's; toolbox lists keep at most 256
  entries, and an overlong tool id counts as none.
- An interface size outside 50 % to 400 % becomes 100 %.

## Checks

- Given a workspace with custom shortcuts, bars on the right, the status
  bar hidden, palette options and an interface size of 125 %, when it is
  exported and imported into a fresh editor, then every part is the same.
- Given a file exported with shortcuts and palette only, when it is
  imported with only shortcuts checked, then the bars and palette stay as
  they were.
- Given garbage, an empty file, another kind, version 0 or a part of the
  wrong shape, when it is imported, then nothing changes and the status
  bar says it is not a workspace file.
- Given the status bar hidden and the Transform toolbar shown, when
  CD Draw starts again, then they are still hidden and shown.
