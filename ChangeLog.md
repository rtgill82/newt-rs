# ChangeLog

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
