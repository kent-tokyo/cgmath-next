# Security Policy

## Scope

This policy covers `cgmath-next` itself: memory-safety issues (including
safe-Rust-reachable undefined behavior — "soundness" issues), and any other
security-relevant defect in this crate's code.

**Soundness issues are treated as security issues.** A safe API that can
be driven into undefined behavior (e.g. the RUSTSEC-2026-0197 same-index
`swap_columns` bug this project was created to fix) is exactly the kind of
report this policy is for, not just classic memory-corruption-from-unsafe
bugs.

## Supported versions

`cgmath-next` has not yet published a stable release. Until a `0.18.1`
stable release exists, only the latest commit on the default branch is
supported. Once `0.18.1` ships, this section will be updated with the
supported version range (expected: the latest `0.18.x` release).

## Reporting a vulnerability

**Please use [GitHub Security Advisories](https://github.com/kent-tokyo/cgmath-next/security/advisories/new)
for this repository** rather than a public issue. A dedicated security
contact email has not been established yet — do not send reports to any
email address not listed on this page, since none is currently listed.

**Do not post proof-of-concept exploit code or Miri/ASan repro cases in a
public GitHub issue, PR, or discussion.** Undefined-behavior repro cases
are exactly the kind of thing that belongs in a private advisory, not a
public issue tracker, until a fix is available.

## What to include in a report

* A minimal reproduction (ideally runnable under `cargo miri test` if it's
  a soundness issue)
* The affected version/commit
* Whether the issue is reachable from 100% safe Rust, or requires existing
  unsafe code in the caller
* Any suggested fix, if you have one

## Disclosure policy

* We aim to acknowledge new reports within a reasonable timeframe and will
  work with the reporter on a disclosure timeline once a fix is available.
* A fix will be released as a new patch version before or alongside public
  disclosure wherever feasible.
* Credit is given to reporters in the release notes, unless the reporter
  requests otherwise.
* If a report turns out to affect upstream `cgmath` itself (i.e. the bug
  predates this fork and is still present in `cgmath` 0.18.0 or later on
  `rustgd/cgmath`), we will discuss with the reporter whether and how to
  also notify upstream and RustSec — this is not assumed automatically,
  since upstream `cgmath` is unmaintained and the appropriate channel may
  be RustSec directly rather than an unmaintained repository's issue
  tracker.

## Relationship to RustSec

`cgmath-next` was created partly in response to
[RUSTSEC-2026-0196](https://rustsec.org/advisories/RUSTSEC-2026-0196.html)
(upstream `cgmath` unmaintained) and
[RUSTSEC-2026-0197](https://rustsec.org/advisories/RUSTSEC-2026-0197.html)
(the `swap_columns` soundness issue, fixed in this fork — see
`docs/unsafe-audit.md` for the full scope of what was fixed, which is
broader than the advisory's literal wording). We intend to keep RustSec
informed of `cgmath-next`-specific advisories once this project has a
public release and its own advisory-db presence.
