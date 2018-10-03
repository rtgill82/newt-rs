#[macro_export]
macro_rules! newt_component {
    ($type:tt, $($gen:tt),+) => {
        newt_component_base!($type<$($gen),+>, <$($gen),+>);
        newt_component_partial_eq_trait!($type<$($gen),+>, <Rhs: Component, $($gen),+>);
        newt_component_partial_eq!($type<$($gen),+>, <$($gen),+>);
    };

    ($type:tt,) => {
        newt_component_base!($type);
        newt_component_partial_eq_trait!($type, <Rhs: Component>);
        newt_component_partial_eq!($type);
    };

    ($type:tt, < $($gen:tt),+ >) => {
        newt_component!($type, $($gen),+);
    };

    ($type:tt $($tail:tt)*) => {
        newt_component!($type, $($tail)*);
    };
}

#[macro_export]
macro_rules! c_ptr_array_to_boxed_slice {
    ($ptr:tt [ $type:tt ], $numitems:tt) => {{
        let mut vec: Vec<&$type> = Vec::new();
        if $numitems > 0 {
            let mut count = 0;
            let mut p = $ptr;
            unsafe {
                while count < $numitems {
                    vec.push(&*(p as *const $type));
                    p = p.offset(1);
                    count += 1;
                }
            }
        }
        vec.into_boxed_slice()
    }};
}

macro_rules! newt_component_base {
    ($type:ty, $($gen:tt)*) => {
        impl $($gen)* Component for $type {
            fn co(&self) -> c_component {
                self.co
            }

            fn attach_to_form(&mut self) {
                self.attached_to_form = true;
            }

            fn attached_to_form(&self) -> bool {
                self.attached_to_form
            }
        }
    };

    ($type:ty) => {
        newt_component_base!($type,);
    };
}

macro_rules! newt_component_partial_eq_trait {
    ($type:ty, $($gen:tt)*) => {
        impl $($gen)* std::cmp::PartialEq<Rhs> for $type {
            fn eq(&self, other: &Rhs) -> bool {
                self.co == other.co()
            }
        }
    };

    ($type:ty) => {
        newt_component_partial_eq_trait!($type,);
    };
}

macro_rules! newt_component_partial_eq {
    ($type:ty, $($gen:tt)*) => {
        impl $($gen)* std::cmp::PartialEq<Box<Component>> for $type {
            fn eq(&self, other: &Box<Component>) -> bool {
                self.co == other.co()
            }
        }

        impl $($gen)* std::cmp::PartialEq<ExitReason> for $type {
            fn eq(&self, other: &ExitReason) -> bool {
                other == self
            }
        }
    };

    ($type:ty) => {
        newt_component_partial_eq!($type,);
    };
}
