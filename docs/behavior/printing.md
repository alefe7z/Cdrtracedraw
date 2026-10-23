# Printing

File > Print (Ctrl+P) builds a print-ready PDF of the sheets and sends
it to the printer chosen in the dialog, or opens it in the system
viewer, where the printer is chosen. `export::print` builds it; the sheets come from
`imposition.rs`.

## Printers

The Printer list holds the system's printers, the default one first,
then "PDF viewer (system)". Each printer comes with its status, location
and comment, shown under the list (`printers.rs`).

- Linux and macOS: `lpstat -l -p -d` lists the CUPS printers when the
  dialog opens (`parse_lpstat`, which also reads the Description and
  Location lines), and a print on one of them hands the PDF to `lp -d`
  with the number of copies and their collation (`lp_args`) and the
  drawing's name as the job title; the status bar says "Sent to printer
  name", or what `lp` complained about.
- Windows: PowerShell lists `Win32_Printer` as CSV
  (`parse_windows_printers`: name, default, status, offline, location,
  comment). A print hands the PDF to the PDF program's "Print to" verb
  for that printer; with no program that can, the PDF opens in the
  viewer.
- The gear beside the list opens the system's settings of the printer:
  the driver's preferences on Windows (`printui.dll,PrintUIEntry /e`),
  the printer settings elsewhere.

The dialog starts on the printer printed to last when the system still
has it, else on the default printer, else on the PDF viewer
(`starting_printer`); the printer is remembered with the other print
choices (Options > Global > Printing).

## Print dialog

One window of a fixed size (`ui/print_dialog.rs`) with seven tabs:
General, Color, Separations, Layout, Prepress, PostScript and the
preflight's issues ("No Issues", "1 Issue" or "n Issues"). The tab in
front is blue. Along the bottom: help (?), Print Preview, the side
preview button (the first sheet beside the tabs), then Print (blue, the
default button), Cancel and Apply (keeps the settings as Options >
Global > Printing's). Labels underline their access letter. The dialog
starts with those preferences, and with the drawing's own bleed as the
bleed limit when it has one.

- General: Destination (Printer and its gear, Orientation, Use PPD,
  Status, Location, Comment; Print to file writes a PostScript file, with
  Single file, pages or plates to separate files). Print range: Current
  document, Documents (with more than one open), Current page,
  Selection, Pages (a list such as `1-3, 5, 8-`) and Even & Odd, Even or
  Odd pages. Copies: the number, the picture of the sets and Collate.
  Print as bitmap at a resolution. Print style: Custom while the
  settings match no saved style, Default print settings and the saved
  styles; Save As names a new style (kept in the preferences).
- Color: Composite or separations; the document's or the proof's color
  settings, the conversions, the output colors, Convert spot colors to
  CMYK (spot inks print as their CMYK mix at their tint), the profile,
  Preserve CMYK numbers and pure black, the rendering intent, and a note.
- Separations: document overprints, screen frequency, In-RIP trapping,
  Print text in black (fills, outlines and character colours of every
  text), Always overprint black (100 % black fills and outlines
  overprint), Print separations in color, Advanced (the inks' screens),
  Show empty plates, and the plates with their ink and a box each: a
  plate unticked is left out of the separations
  (`export::separations_doc`).
- Layout: As in document, Fit to page (with printer's marks) or
  Reposition images to a corner or the centre; the settings for a page
  (position, size, scale and tiles); tiled pages with marks and overlap;
  the bleed limit (how far beyond the page objects print, 4 mm by
  default); the imposition layout and Edit, which opens the preview with
  the imposition tool.
- Prepress: Invert and Mirror (the whole sheet, marks included; Invert
  prints a negative, the sheet's empty paper black too,
  `export::tests::black_text_prints_black_and_a_negative_blackens_the_sheet`);
  registration marks and their style; Print file information (with its
  text, the drawing's name when left empty), Print page numbers and
  Position within page; color calibration bar, densitometer scales and
  their densities; crop/fold marks (exterior only); bitmap downsampling;
  marks to objects.
- PostScript: the language level written in a PostScript file's header
  (2 or 3), DSC, bitmap compression and JPEG quality, OPI and DCS links,
  Type 1 fonts, PDF marks, flatness, fountain steps and fountain fills.
- Issues: what the preflight finds; a picked issue shows its hint and
  "Don't check for this issue in the future"; the gear opens the
  preflight settings.

Options kept but not yet changing the output: Use PPD, Print to file's
separate files, the color conversions, profile and intent, overprints,
screens, trapping, Reposition, tiling, the registration mark style,
Position within page, Exterior only, downsampling, marks to objects, DSC,
compression, OPI, DCS, fonts, PDF marks, flatness and fountain steps.

## Print Preview

File > Print Preview, or Print Preview in the Print dialog (which hands
over its settings), fills the window with the sheets exactly as Print
sends them: the imposition, the marks, Mirror and Invert, built by the
same function (`export::print_sheets`). Closing it (Esc, the last
button of the bar, File > Close Print Preview) goes back to the Print
dialog when it came from there.

- Menu bar: File (Print, Save and Delete Print Style, Print Options,
  Close), View (Full Screen Preview, Enable Color, To Page, To Width, To
  Height, Actual Size), Settings (Print Options, composite or
  separations, Invert, Mirror) and Help.
- First bar: the print style, save (+) and delete (-) a style, Print
  Options (the Print dialog's tabs with OK, Cancel and Apply), Print
  (Ctrl+P), the zoom (To Page, To Width, To Height, 10 to 800 %), full
  screen (Esc leaves it), Enable Color (plates in their ink's colour),
  Invert, Mirror and Close.
- Second bar: the imposition layout, then what the tool in use sets:
  with Pick or Zoom the objects' position and size, the scale and the
  units; with the imposition layout tool the columns, rows, gutter and
  sheet; with the marks placement tool the printer's marks. The tile
  button turns tiled pages on.
- Left: Pick, Imposition Layout, Marks Placement and Zoom (click zooms
  in, right click out; Ctrl and the wheel zoom too).
- The sheet on the desktop with the printable area dashed, 3.5 mm in.
- Bottom: first, previous, "n of m", next and last, then a tab per sheet,
  or with separations per plate that prints ("Page 1 - Cyan"; empty
  plates only with Show empty plates, unticked plates left out); Page Up
  and Page Down go through them. A plate shows the ink's coverage,
  in the ink's colour with Enable Color, else in grey (black = full ink);
  registration colour shows on every plate.
- Status bar: the selection, the printer, and "CMYK Seps" or
  "Composite".

## Imposition

- Pages keep their aspect ratio; each cell is as large as the largest
  page on its sheet and a smaller page sits in its middle.
- A booklet's page count is rounded up to a multiple of four with blank
  pages at the back. The sheet sides pair the pages as a folded booklet
  needs: for eight pages 8 and 1, 2 and 7, 6 and 3, 4 and 5 (left and
  right). Its two pages meet at the spine with no gap and no bleed between
  them.
- Every page is clipped to its trim box plus the bleed, so nothing spills
  onto its neighbours or the margin. Hidden and non-printing layers are
  left out; master layers print with every page they show on; a page
  background prints under its page.

## Marks

With any mark on, a 12 mm margin (plus the bleed) surrounds the pages.

- Crop marks: 5 mm lines in registration colour, 0.1 mm wide, starting
  1.5 mm beyond the bleed, at every cut line of the grid, outside the
  pages only. A booklet also gets a dashed fold mark at the spine.
- Registration marks: a target (circle and cross) centred on each side
  of the pages.
- Color bars: patches of 4 mm of cyan, magenta, yellow, black, red,
  green, blue (two-ink overprints) and 50 % tints of the four inks, left
  of the top target.
- Densitometer scale: black from 0 to 100 % in steps of ten, right of the
  top target.
- Page numbers: each page's number, 6 pt, under the page.
- Page information: the drawing's name (or the text given), the pages on the sheet, "sheet n
  of n" and the date and time, in 6 pt under the pages.

## Checks

- `printers::tests::the_printer_list_is_read_from_lpstat`, `printers::tests::the_dialog_starts_on_the_remembered_printer_else_the_default`, `printers::tests::lp_is_told_the_printer_copies_and_title`
- `export::tests::page_lists_and_ranges_pick_pages`
- `export::tests::print_colour_options_change_the_sheets_objects`
- `export::tests::copies_repeat_sheets_collated_or_not_and_ps_gets_its_level`

- `ui::print_preview::tests::plates_split_the_sheet_by_ink`
- `ui::print_preview::tests::the_settings_key_ignores_copies_and_preflight`

- `imposition::tests::booklets_pair_the_pages_for_folding`
- `imposition::tests::n_up_fills_sheets_in_reading_order_and_scales_to_fit`
- `imposition::tests::fitted_sheets_grow_with_marks_bleed_and_gutters`
- `imposition::tests::imposing_places_and_clips_every_page`
- `export::tests::the_print_dialog_settings_make_the_layout`
- `export::tests::colours_are_mapped_inside_groups_frames_and_text`: the
  separations and Invert reach into groups, clip frames and character
  fills.
