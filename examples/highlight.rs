// Copyright (c) 2015-2016 Georg Brandl.  Licensed under the Apache License,
// Version 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at
// your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::env;
use std::fs::File;
use std::io::Read;

extern crate rugments;
use rugments::lexer::HtmlLexer;

fn main() {
    let mut buf = Vec::new();
    let filename = env::args().nth(1).unwrap();
    File::open(filename).unwrap().read_to_end(&mut buf).unwrap();
    let bufstr = String::from_utf8(buf).unwrap();
    for tok in HtmlLexer::new(&bufstr) {
        println!("{:?}", tok);
    }
}
