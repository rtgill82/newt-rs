# newt-rs

_Rust bindings for the Newt console UI library._

[ChangeLog](https://github.com/rtgill82/newt-rs/blob/v0.6.11/ChangeLog.md) |
[API Docs](https://docs.rs/newt/0.6.11/newt/) |
[crates.io](https://crates.io/crates/newt)

This crate provides bindings to Red Hat, Inc.'s [Newt][newt] console UI
library. Newt is a small and simple to use UI library providing widgets and
basic stacked window management for console applications.

[newt]: https://pagure.io/newt

## Usage

Include _newt_ as a dependency of in your `Cargo.toml` file as follows.

```
[dependencies]
newt = "0.6"
```

The library API is currently subject to change and there are likely to be
breaking changes between minor versions. Requires at least Rust 1.59 for inline
assembly functions on the supported architectures.

## Features

The following features are available as build options:
[`static`][static_feature], [`asm`][asm_feature], [`f16`][f16_feature].

[static_feature]: #static_feature
[asm_feature]: #asm_feature
[f16_feature]: #f16_feature

### `static` feature

Enabling the `static` feature will force the `newt-sys` dependency to be
statically built against its included libraries rather than dynamically against
local system libraries when the required libraries are available on the system.

[newt_sys]: https://crates.io/crates/newt-sys

### <a name="asm_feature"></a> `asm` feature

Some library functions such as ``windows::win_entries()``,
``windows::win_menu()``, and functions related to the ``Grid`` struct require
the _asm_ feature to be enabled.

These are currently available on the _x86_, <i>x86_64</i>, _arm_, _aarch64_,
_riscv32_, and _riscv64_ architectures.

### <a name="f16_feature"></a> `f16` feature

Enable an implementation of the `Data` trait for the `f16` primitive, allowing
it to be passed as data to `Listbox`es and `CheckboxTree`s. This primitive is
unstable and only available in nightly builds of the compiler. Adding the
`-Zcrate-attr=feature(f16)` option to the `RUSTFLAGS` environment variable
is required to build.

## LICENSE

Copyright (C) 2018-2020,2025  Robert Gill <<rtgill82@gmail.com>>

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
