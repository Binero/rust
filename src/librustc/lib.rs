// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The "main crate" of the Rust compiler. This crate contains common
//! type definitions that are used by the other crates in the rustc
//! "family". Some prominent examples (note that each of these modules
//! has their own README with further details).
//!
//! - **HIR.** The "high-level (H) intermediate representation (IR)" is
//!   defined in the `hir` module.
//! - **MIR.** The "mid-level (M) intermediate representation (IR)" is
//!   defined in the `mir` module. This module contains only the
//!   *definition* of the MIR; the passes that transform and operate
//!   on MIR are found in `librustc_mir` crate.
//! - **Types.** The internal representation of types used in rustc is
//!   defined in the `ty` module. This includes the **type context**
//!   (or `tcx`), which is the central context during most of
//!   compilation, containing the interners and other things.
//! - **Traits.** Trait resolution is implemented in the `traits` module.
//! - **Type inference.** The type inference code can be found in the `infer` module;
//!   this code handles low-level equality and subtyping operations. The
//!   type check pass in the compiler is found in the `librustc_typeck` crate.
//!
//! For a deeper explanation of how the compiler works and is
//! organized, see the README.md file in this directory.
//!
//! # Note
//!
//! This API is completely unstable and subject to change.

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/nightly/")]
#![deny(warnings)]

#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(conservative_impl_trait)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(i128_type)]
#![feature(inclusive_range_syntax)]
#![cfg_attr(windows, feature(libc))]
#![feature(macro_vis_matcher)]
#![feature(never_type)]
#![feature(nonzero)]
#![feature(quote)]
#![feature(rustc_diagnostic_macros)]
#![feature(slice_patterns)]
#![feature(specialization)]
#![feature(unboxed_closures)]
#![feature(trace_macros)]
#![feature(test)]
#![feature(const_atomic_bool_new)]

#![recursion_limit="512"]

extern crate arena;
#[macro_use] extern crate bitflags;
extern crate core;
extern crate fmt_macros;
extern crate getopts;
extern crate graphviz;
#[cfg(windows)]
extern crate libc;
extern crate owning_ref;
extern crate rustc_back;
#[macro_use] extern crate rustc_data_structures;
extern crate serialize;
extern crate rustc_const_math;
extern crate rustc_errors as errors;
#[macro_use] extern crate log;
#[macro_use] extern crate syntax;
extern crate syntax_pos;
extern crate jobserver;

extern crate serialize as rustc_serialize; // used by deriving

// Note that librustc doesn't actually depend on these crates, see the note in
// `Cargo.toml` for this crate about why these are here.
#[allow(unused_extern_crates)]
extern crate flate2;
#[allow(unused_extern_crates)]
extern crate test;

#[macro_use]
mod macros;

// NB: This module needs to be declared first so diagnostics are
// registered before they are used.
pub mod diagnostics;

pub mod cfg;
pub mod dep_graph;
pub mod hir;
pub mod ich;
pub mod infer;
pub mod lint;

pub mod middle {
    pub mod allocator;
    pub mod borrowck;
    pub mod expr_use_visitor;
    pub mod const_val;
    pub mod cstore;
    pub mod dataflow;
    pub mod dead;
    pub mod dependency_format;
    pub mod entry;
    pub mod exported_symbols;
    pub mod free_region;
    pub mod intrinsicck;
    pub mod lang_items;
    pub mod liveness;
    pub mod mem_categorization;
    pub mod privacy;
    pub mod reachable;
    pub mod region;
    pub mod recursion_limit;
    pub mod resolve_lifetime;
    pub mod stability;
    pub mod trans;
    pub mod weak_lang_items;
}

pub mod mir;
pub mod session;
pub mod traits;
pub mod ty;

pub mod util {
    pub mod common;
    pub mod ppaux;
    pub mod nodemap;
    pub mod fs;
}

// A private module so that macro-expanded idents like
// `::rustc::lint::Lint` will also work in `rustc` itself.
//
// `libstd` uses the same trick.
#[doc(hidden)]
mod rustc {
    pub use lint;
}

// FIXME(#27438): right now the unit tests of librustc don't refer to any actual
//                functions generated in librustc_data_structures (all
//                references are through generic functions), but statics are
//                referenced from time to time. Due to this bug we won't
//                actually correctly link in the statics unless we also
//                reference a function, so be sure to reference a dummy
//                function.
#[test]
fn noop() {
    rustc_data_structures::__noop_fix_for_27438();
}


// Build the diagnostics array at the end so that the metadata includes error use sites.
__build_diagnostic_array! { librustc, DIAGNOSTICS }
