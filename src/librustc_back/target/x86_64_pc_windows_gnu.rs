// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use target::Target;

pub fn target() -> Target {
    let mut base = super::windows_base::opts();
    base.cpu = "x86-64".to_string();
    // On Win64 unwinding is handled by the OS, so we can link libgcc statically.
    base.pre_link_args.push("-static-libgcc".to_string());
    base.pre_link_args.push("-m64".to_string());

    Target {
        llvm_target: "x86_64-pc-windows-gnu".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        arch: "x86_64".to_string(),
        target_os: "windows".to_string(),
        target_env: "gnu".to_string(),
        options: base,
    }
}
