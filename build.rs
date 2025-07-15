//
// Copyright (C) 2025 Robert Gill <rtgill82@gmail.com>
//
// This file is a part of newt-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 2.1 as published by the Free Software Foundation.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::env;

use rustc_version::Channel;
use rustc_version::version_meta;

const F16_FEATURE_ATTR: &str = "-Zcrate-attr=feature(f16)";

pub fn main() {
    println!("cargo::rustc-check-cfg=cfg(nightly)");

    if env::var("CARGO_FEATURE_F16").is_ok() {
        match version_meta().unwrap().channel {
            Channel::Nightly => {
                match env::var("CARGO_ENCODED_RUSTFLAGS") {
                    Ok(mut rustflags) => check_rustflags(&mut rustflags),
                    Err(_)            => rustflag_error()
                }
                println!("cargo::rustc-cfg=nightly");
            },

            _ => nightly_error()
        }
    }
}

fn nightly_error() -> ! {
    panic!("ERROR: `f16` feature currently requires the nightly build of the rust compiler.")
}

fn rustflag_error() -> ! {
    panic!("ERROR: `f16` feature requires the '{}' option to be set in the RUSTFLAGS environment variable.", F16_FEATURE_ATTR)
}

fn check_rustflags(rustflags: &mut String) {
    remove_whitespace(rustflags);
    if !rustflags.contains(F16_FEATURE_ATTR)
    {
        rustflag_error();
    }
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !(c.is_whitespace() || c == '\u{1f}'));
}
