// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Regression tests for RUSTSEC-2026-0197 / rustgd/cgmath#565: same-index
//! calls into safe swap APIs must not create aliased `&mut` references.
//! Also covers UNSAFE-001 (docs/unsafe-audit.md): the fixed-size-array
//! reference conversions' transmutes. Run under Miri (see
//! docs/unsafe-audit.md) to catch aliasing violations, not just the
//! (unaffected) output values.
//!
//! Cargo only auto-discovers direct children of `tests/` as test binaries,
//! so these use `#[path]` to pull content from `tests/soundness/` while
//! keeping a single discovered target -- an explicit `[[test]]` entry in
//! Cargo.toml would disable autotests discovery for every other file in
//! `tests/`, silently dropping the upstream test suite from `cargo test`.

extern crate cgmath;

#[path = "soundness/array_conversions.rs"]
mod array_conversions;
#[path = "soundness/swap_columns.rs"]
mod swap_columns;
#[path = "soundness/swap_elements.rs"]
mod swap_elements;
