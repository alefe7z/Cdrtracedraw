# Docker catalog

The plus button under a side column's vertical tabs opens a compact panel
catalog. The owner's 2026-10-23 screenshot and 17.55-second reference
video supply the row ordering and interaction reference. All pictograms
are original vector drawings from the project's painters; no reference
image, video or proprietary program asset is bundled.

## Layout and interaction

- Rows are 17 interface points high, with a 12-point checkbox, a separate
  16-point icon and a 12-point label. The minimum width is 300 points;
  longer translations widen the menu. Cyan rules separate the groups.
- Checkboxes reflect whether a panel is open anywhere, including a
  collapsed column or floating window. A checked row closes that panel.
  An unchecked row opens it at the front of the column whose plus was used.
- The catalog remains open while several rows are toggled. Click outside
  or press Escape to dismiss the popup. On smaller screens the catalog
  scrolls, and the tab column scrolls separately to keep plus reachable.
- The same pictograms appear on vertical tabs. The active tab has a blue
  highlight in normal themes; high contrast uses the configured palette.
- Comments, Effects and Macros remain available from the separate Window menu.
- Portuguese labels follow the supplied catalog. Picture Mosaic and Vector
  Mosaic appear there as PhotoCocktail and Pointillizer; these entries open
  the existing mosaic panels and do not change their algorithms.

## Reference completion pass

The five entries previously disabled are now real DockerTab variants:
Live Guides, Fit Objects to Path, Color Proof, Assets and Views. See
`reference-panels.md` for their behavior and remaining compatibility limits.
The catalog now has 43 active rows, in the reference order, with no extra
More panels row. The three application-specific panels (Comments, Effects,
Macros) remain accessible from Window > Panels.

The dark menu background (38,38,38), label color (202,206,212), separator
(1,125,188), 300-point minimum width, checkbox position and row spacing
come from the supplied screenshot. Panel-specific original vector artwork
replaces the earlier generic tool-icon substitutions. The plus follows the
tab stack; its popup's anchor stays fixed while panels are toggled. Closing
the last panel on the secondary side leaves the open catalog usable.

## Validation

Coverage, popup interaction, asset deduplication, saved-view persistence,
invalid-view rejection, path fitting with rotation and undo, and artwork
coverage have Rust regressions. `reference-validation.yml` runs formatting,
tests, build, library lint and native smoke/monkey checks, and uploads a
render of the real dark pt-BR catalog. A compiled/rendered comparison must
pass before visual equivalence is claimed. Static checks alone are not
native validation.
