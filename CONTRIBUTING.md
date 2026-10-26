# Contributing to CD Draw

Thank you for helping. Read `AGENTS.md` first: it holds the engineering
rules (commands, no panics on input, layering, tests, units, language).
This file covers what every contribution must also respect.

## Contributions

- State the origin of third-party code, data or assets in the pull
  request and list them in `ATTRIBUTION.md` with their author, source and
  license.
- Record new format knowledge, its source and the date in
  `CLEANROOM.md`.
- What the project follows, uses or bundles is the owner's decision
  (`AGENTS.md`, rule 3).

## Sign-off (DCO)

Every commit must carry a sign-off line:

```text
Signed-off-by: Your Name <you@example.com>
```

`git commit -s` adds it. By signing off you certify the Developer
Certificate of Origin 1.1 (https://developercertificate.org/): that you
wrote the contribution or otherwise have the right to submit it under
the project's licenses, and that you understand the contribution and
your sign-off are public and kept with the project.

Pull requests with unsigned commits, or with material whose origin is
unclear, are not merged.

## Workflow

- Run `cargo fmt --all`, `cargo clippy --workspace --tests` and
  `cargo test --workspace` before pushing; the build must stay free of
  warnings.
- Every change comes with tests; format code gets round-trip and
  malformed-input tests.
- User-visible strings go through the translation files (`docs/i18n.md`).
- A new tool or effect gets a page in `docs/behavior/` and a row in
  `docs/features.md`.

## License

Contributions are licensed under MIT OR Apache-2.0, like the rest of the
project.
