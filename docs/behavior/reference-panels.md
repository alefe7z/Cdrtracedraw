# Reference panel integrations

All five panels can be opened, closed, docked at either side or floated.
Their ids are persisted with the existing docker layout preferences.

## Live Guides

Two collapsible sections expose the same Alignment Guides and Dynamic
Guides controls as Options. Changes affect the existing snapping/guide
engine immediately and use the existing settings persistence. Edges,
centers, page targets, margins, equal spacing/sizing, angle presets,
ticks, tips, colors and guide styles keep their existing formulas.

## Fit Objects to Path

Select the objects and then the destination path. Apply distributes the
objects' bounding-box centers by cumulative path length. Flattening
uses the existing 0.05 mm tolerance. Start and end percentages constrain
the interval, Reverse changes placement order, Rotate follows the local
tangent. With n > 1, t = i/(n-1); a single object uses t = 0.5. The final
arc distance is length * (start + t*(end-start)). Invalid/zero lengths
are ignored. Percentages are clamped to 0..100 and end >= start.

One Engine::run_batch transforms the objects and supports undo. The
path is unchanged. Defaults preserve the old menu behavior (0..100,
no rotation/reversal). These preferences persist across sessions.

## Color Proof

Proof Colors and Simulate Overprints connect directly to the existing
renderer and invalidate its cache when changed. The panel displays the
drawing's profile names and rendering intent; Color Management opens
the existing dialog to assign/convert profiles. Color accuracy remains
that of the existing color pipeline; this integration does not claim
matching another application's proprietary proofing algorithms.

## Assets

An offline library of local artwork file references, separate from the
Shelf's embedded snippets. Add Files chooses several supported files,
deduplicates paths, and keeps their order. Search filters file names;
Insert or double-click invokes the existing importer in the active
drawing (with its undo behavior). Remove removes the library reference,
not the source file. Paths persist in settings. Moving/deleting a source
file requires adding its new path; this is not a cloud asset service.

## Views

Save View stores a name, persistent page id, zoom and page-space center.
Saving the same name on the same page replaces it. Restoring uses the
current canvas center, so it works when the window size changes, and
keeps the zoom within the view's limits (`zoom-and-pan.md`, Range). Views for pages outside the
active drawing are hidden. Update replaces the current page's named
view; Remove removes it from preferences. Names and views persist in
settings, not inside exported third-party drawings. Invalid/nonfinite
view data are rejected. Resetting preferences preserves the asset list
and named views, like recent-file history.
