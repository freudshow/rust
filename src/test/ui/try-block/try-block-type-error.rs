// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: --edition 2018

#![feature(try_blocks)]

fn foo() -> Option<()> { Some(()) }

fn main() {
    let _: Option<f32> = try {
        foo()?;
        42
        //~^ ERROR type mismatch
    };

    let _: Option<i32> = try {
        foo()?;
    };
    //~^ ERROR type mismatch
}
