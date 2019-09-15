# newt-rs

_Rust bindings for the Newt console UI library._

[ChangeLog](https://github.com/xelkarin/newt-rs/blob/v0.6.0/ChangeLog.md) |
[API Docs](https://docs.rs/newt/0.6.0/newt/) |
[crates.io](https://crates.io/crates/newt)

This crate provides bindings to Red Hat, Inc.'s [Newt][newt] console UI
library. Newt is a small and simple to use UI library providing widgets and
basic stacked window management for console applications.

The majority, if not all of Newt's functionality has been implemented.
Although some [features][asm_feature] currently require the nightly build of the
Rust compiler.

[newt]: https://pagure.io/newt
[asm_feature]: #asm_feature

## Usage

Include _newt_ as a dependency of in your `Cargo.toml` file as follows.

```
[dependencies]
newt = "0.6"
```

The library API is currently subject to change and there are likely to be
breaking changes between minor versions.

## Features

### `static` feature

Enabling the `static` feature will force the [`newt-sys`][newt_sys] dependency
to be built against its included libraries statically rather than dynamically
against local system libraries if they're available.

It can be enabled in your `Cargo.toml` file as follows.

```
[dependencies]
newt = { version = "0.6", features = ["static"] }
```

[newt_sys]: https://crates.io/crates/newt-sys

### <a name="asm_feature"></a> `asm` feature

Some library functions such as ``windows::win_menu()``,
``windows::win_entries()``, and ``Grid`` related functions require the nightly
build of the Rust compiler and the _newt_ _asm_ feature to be enabled. These
are currently only available on the _x86_ and <i>x86_64</i> architectures.

Enable the feature as follows in your `Cargo.toml` file.

```
[dependencies]
newt = { version = "0.6", features = ["asm"] }
```

## LICENSE

Copyright (C) 2018,2019  Robert Gill <locke@sdf.lonestar.org>

This library is free software; you can redistribute it and/or
modify it under the terms of the GNU Lesser General Public
License version 2.1 as published by the Free Software Foundation.

This library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public
License along with this library; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
