# Provenance

## Baseline source

`cgmath-next` is a fork of the **published** `cgmath` 0.18.0 crate, not upstream's
current `master` branch.

| Field | Value |
|---|---|
| Crate | `cgmath` |
| Version | `0.18.0` |
| Download URL | `https://crates.io/api/v1/crates/cgmath/0.18.0/download` |
| SHA-256 (tarball) | `1a98d30140e3296250832bbaaff83b27dcd6fa3cc70fb6f1f3e5c9c0023b5317` |
| SHA-256 source | crates.io sparse index, `https://index.crates.io/cg/ma/cgmath`, `cksum` field for `vers = "0.18.0"` |
| Upstream commit SHA (from `.cargo_vcs_info.json`) | `637c566cc2141203d8d99c03e7ab770796c44f5f` |
| Upstream repository | `https://github.com/rustgd/cgmath` |
| Fetch date | 2026-08-16 |
| License | Apache-2.0 (preserved verbatim, see `LICENSE`) |
| crates.io publish date | 2021-01-03T01:02:55Z |

The tarball's SHA-256 was independently computed with `shasum -a 256` and matches
the `cksum` recorded in the crates.io sparse index exactly. Both values are
recorded above.

The imported `Cargo.toml` in this repository is the package's `Cargo.toml.orig`
(the author-written manifest as committed upstream), not the registry-normalized
`Cargo.toml` that crates.io generates on publish. This matters for `[lib] name`,
comments, and dependency requirement syntax — see `docs/compatibility.md` for why
this was necessary for the rename fixtures.

No files were modified during import. The first commit
(`chore: import cgmath 0.18.0 release source`) contains exactly the 39 files
present in the published tarball, minus `.cargo_vcs_info.json` (cargo-generated
build metadata, not source) and `Cargo.toml` (replaced by `Cargo.toml.orig`,
content diff recorded below for transparency).

## Cargo.toml normalization diff (registry Cargo.toml vs. Cargo.toml.orig)

The registry-generated `Cargo.toml` differs from `Cargo.toml.orig` only in ways
cargo's publish step always applies: reordering, `[dependencies.X]` table syntax
instead of inline tables, and an auto-generated header comment. No semantic
difference. Notably, `Cargo.toml.orig` already contains an explicit
`[lib]\nname = "cgmath"` — this is not something we are adding, it was already
present upstream.

## License and copyright

* `LICENSE` copied verbatim (Apache-2.0).
* Per-file copyright headers (`Copyright 2013-2014 The CGMath Developers...`)
  preserved unmodified in every source file.
* No `NOTICE` file exists in the upstream 0.18.0 tarball or the upstream
  repository at commit `637c566c`, so none is added here.

## Relationship to upstream `master`

See `docs/upstream-master-diff.md` for the full commit-by-commit classification
of `rustgd/cgmath` commits between tag `v0.18.0` and `master` (24 commits, as of
2026-08-16). None of them address RUSTSEC-2026-0197 (`swap_columns` UB) — the
`unsafe { ptr::swap(&mut self[a], &mut self[b]) }` pattern is present unchanged
on master. The soundness fix in this repository is original work, not a
backport.
