# ChangeLog

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
