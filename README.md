# newt-rs

_Rust bindings for the Newt console UI library._

* [ChangeLog](https://github.com/xelkarin/newt-rs/blob/v0.5.0/ChangeLog.md)

## Usage

Include _newt_ as a dependency of in your `Cargo.toml` file as follows.

```
[dependencies]
newt = "0.5"
```

The library API is currently subject to change and there are likely to be
breaking changes between minor versions.

### _asm_ feature

Some library functions such as ``windows::win_menu()``,
``windows::win_entries()``, and ``Grid`` related functions require the nightly
build of the Rust compiler and the _newt_ _asm_ feature to be enabled. These
are currently only available on the _x86_ and <i>x86_64</i> architectures.

Enable the feature as follows in your `Cargo.toml` file.

```
[features]
default = ["newt/asm"]
```

## LICENSE

Copyright (C) 2018,2019  Robert Gill

This library is free software; you can redistribute it and/or
modify it under the terms of the GNU Library General Public
License version 2 as published by the Free Software Foundation.

This library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
Library General Public License for more details.

You should have received a copy of the GNU Library General Public
License along with this library; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
