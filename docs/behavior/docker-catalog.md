# Docker catalog

The plus button under a side column's vertical tabs opens a compact panel
catalog. The owner's 2026-10-23 screenshot and 17.55-second reference
video supply the row ordering and interaction reference. All pictograms
are original vector drawings from the project's painters; no reference
image, video or proprietary program asset is bundled.

## Layout and interaction

- Rows are 17 interface points high, with an 11-point checkbox, a separate
  16-point icon and a 12-point label. The minimum width is 296 points;
  longer translations widen the menu. Cyan rules separate the groups.
- Checkboxes reflect whether a panel is open anywhere, including a
  collapsed column or floating window. A checked row closes that panel.
  An unchecked row opens it at the front of the column whose plus was used.
- The catalog remains open while several rows are toggled. Click outside
  or press Escape to dismiss the popup. On smaller screens the catalog
  scrolls, and the tab column scrolls separately to keep plus reachable.
- The same pictograms appear on vertical tabs. The active tab has a blue
  highlight in normal themes; high contrast uses the configured palette.
- Comments, Effects and Macros remain available under More panels, after
  the reference groups. The separate Window menu is unchanged.
- Portuguese labels follow the supplied catalog. Picture Mosaic and Vector
  Mosaic appear there as PhotoCocktail and Pointillizer; these entries open
  the existing mosaic panels and do not change their algorithms.

## Existing limitations

Live Guides, Fit Objects to Path, Color Proof, Assets and Views do not
have dedicated docker implementations in this project. They are shown
explicitly disabled, with a translated explanation. No checkbox is
allowed to pretend to open a nonexistent panel. This change does not
implement those five panels or promise their reference application's
behavior. Every one of the 41 existing panels is still reachable.

## Validation

`ui::dockers::catalog::tests` checks complete, unique panel coverage and
opening/closing at the requested side, including collapsed and floating
panels. Native compilation, formatting, workspace tests, clippy and
visual smoke/monkey checks remain pending in the authoring environment,
which has no Rust/Cargo or display server. Do not mark native visual
acceptance complete until a built application has been inspected.
