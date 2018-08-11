#[macro_export]
macro_rules! newt_component {
    ($type:tt, $($gen:tt),+) => {
        newt_component_base!($type<$($gen),+>, <$($gen),+>);
        newt_component_partial_eq!($type<$($gen),+>, <Rhs: Component, $($gen),+>);
        newt_component_deref!($type<$($gen),+>, <$($gen),+>);
    };

    ($type:tt,) => {
        newt_component_base!($type);
        newt_component_partial_eq!($type, <Rhs: Component>);
        newt_component_deref!($type);
    };

    ($type:tt, < $($gen:tt),+ >) => {
        newt_component!($type, $($gen),+);
    };

    ($type:tt $($tail:tt)*) => {
        newt_component!($type, $($tail)*);
    };
}

macro_rules! newt_component_base {
    ($type:ty, $($gen:tt)*) => {
        impl $($gen)* Component for $type {
            fn co(&self) -> NewtComponentPtr {
                self.co
            }

            fn takes_focus(&self, value: bool) {
                #[link(name="newt")]
                extern "C" {
                    fn newtComponentTakesFocus(co: NewtComponentPtr,
                                               val: c_int);
                }

                unsafe { newtComponentTakesFocus(self.co, value as c_int); }
            }
        }
    };

    ($type:ty) => {
        newt_component_base!($type,);
    };
}

macro_rules! newt_component_partial_eq {
    ($type:ty, $($gen:tt)*) => {
        impl $($gen)* std::cmp::PartialEq<Rhs> for $type {
            fn eq(&self, other: &Rhs) -> bool {
                self.co == other.co()
            }
        }
    };

    ($type:ty) => {
        newt_component_partial_eq!($type,);
    };
}

macro_rules! newt_component_deref {
    ($type:ty, $($gen:tt)*) => {
        impl $($gen)* std::ops::Deref for $type {
            type Target = NewtComponentPtr;
            fn deref(&self) -> &Self::Target {
                &self.co
            }
        }
    };

    ($type:ty) => {
        newt_component_deref!($type,);
    };
}
