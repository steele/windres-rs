// Copyright (c) 2017-2018 FaultyRAM
// Copyright (c) 2020 James Steele
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

extern crate windres;

use windres::Build;

fn main() {
    Build::new().compile("windres-test.rc").unwrap();
}
