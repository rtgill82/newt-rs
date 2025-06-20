# ChangeLog

## 0.6.11

* Implement DestroyCallback.

* Add `RadiobuttonSet`.

* Add `set` and `set_colors` functions to `VerticalScrollbar`.

* Add `select_items` function to `Listbox`.

* Make the `newt::grid::Parent` trait public.

* Implement better error handling when converting `char` to `c_char`

* CheckboxTree: only accept positive integers as indexes.

* Add aliases for and deprecate some `Grid` related constants.

* Documentation updates

## 0.6.10

* Documentation updates

## 0.6.9

* Support `aarch64`, `riscv32`, and `riscv64` architectures.

* Assembly clean ups and fixes.

* Bug fixes and compilation warning fixes.

## 0.6.8

* Bug fixes

## 0.6.7

* Update assembly code for the new inline assembler.

## 0.6.6

* Documentation updates

## 0.6.5

* Check for non-ASCII `chars` when passing `chars` as `Data`

## 0.6.4

* Use `llvm_asm!` macro to allow building with recent nightlies.

## 0.6.3

* Add `take_component()` method to Form.

## 0.6.2

* Remove dependency on the `dropck_eyepatch` nightly feature.

## 0.6.1

* Update license version in Cargo.toml, create new release to update crates.io.

## 0.6.0

* Prevent use after free bugs that were possible in previous versions.

* Functions changing widget settings no longer declared as `mut`.

* Change license to LGPL-2.1.

## 0.5.5

* Implement Clone, Debug, and Default traits for the Colors struct.

## 0.5.4

* Moved components to the `newt::widget` namespace.
  Usage of `newt::components::*` is deprecated.

* Renamed `newt::components::ComponentFuncs` trait to `newt::widgets::WidgetFns`.
  Usage of `ComponentFuncs` is deprecated.

* Renamed `newt::grid::GridTrait` to `newt::grid::GridFns`.
  Usage of `GridTrait` is deprectated.

* Documentation updates.

## 0.5.3

* Implement `VerticalScrollbar` initialization, although I've yet to figure out
  how they are used...

* Document the `static` feature.

## 0.5.2

* Fix inline assembly _invalid symbol redefinition_ error when making release
  builds.

* Build releases with _'opt-level = 1'_. Higher optimizations cause segfaults
  in functions making use of inline assembly. This doesn't rule out that there
  might be a problem with my code, but I can't find anything.

* Reorganize assembly code.

* Documentation updates.

## 0.5.1

* Documentation updates.

* Add ``prelude`` module containing ``Component``s and constants.

## 0.5.0

* Implement Grids.

## 0.4.0

* Implement ``win_menu()`` and ``win_entries()`` functions. These require the
  _asm_ feature to be enabled.

* ``Form.watch_fd()`` accepts an ``FDFlags`` ``enum`` instead of raw integers.

* ``reflow_text()`` returns it's results as a tuple.

* Some functions now return ``Result`` instead of ``i32`` allowing for Rust
  style error handling.

* Convenience windows were moved to the ``windows`` module.

## 0.3.0

* Implement component callbacks, help callbacks, suspend callbacks, and entry
  filters.

* Implement ``Form.watch_fd()``.

* Implement ``get_position()`` and ``get_size()`` for ``Component``.

## 0.2.2

* Implement the simple message windows ``win_message()``, ``win_choice()``,
  and ``win_ternary()``.

* Implement ``get_cursor_position()`` and ``set_cursor_position()`` for the
  ``Entry`` component.

* Add the feature _static_ allowing users to force the crate to be built
  against static versions of the bundled libraries. Usually dynamic linking
  against the system libraries will be used if they are available.

## 0.2.1

* Depend on ``newt-sys`` crate to provide library bindings.

## 0.2.0

* The ``get_current()`` function for ``Listbox`` and ``CheckboxTree`` now
  returns ``Option``, rather than ``panic!``ing when there are no entries.

* Implemented internal trait ``Data`` that's accepted as an argument for
  certain ``Listbox`` and ``CheckboxTree`` functions. Added implementations of
  the ``Data`` trait to integer primitives.

* Added ``newt::components::component::Data`` tuple struct that implements the
  ``Data`` trait. This can be used to wrap references to complex data types.

* ``CheckboxTree::add_item()`` _indexes_ parameter is now an ``Option``.

## 0.1.1

* ``get_current()`` as implemented for ``CheckboxTree`` and ``Listbox`` raises
  ``panic!`` when called while those components have no entries, preventing it
  from returning an invalid memory reference.

## 0.1.0

* Minimum functionality of basic components.
