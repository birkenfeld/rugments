// Copyright (c) 2015-2016 Georg Brandl.  Licensed under the Apache License,
// Version 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at
// your option. This file may not be copied, modified, or distributed except
// according to those terms.

//! A port of the Pygments highlighter library to Rust.

#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod lexer;

#[cfg(test)]
#[path = "../test/mod.rs"]
mod test;
