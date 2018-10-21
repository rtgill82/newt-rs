#[doc(hidden)]
#[macro_export]
macro_rules! newt_component {
    ($type:ident, {constr: [$($constr:tt)+], params: [$($params:tt)+]}) => {
        newt_component_base!($type<$($params)+>, <$($constr)+>);
        newt_component_partial_eq_trait!($type<$($params)+>, <$($constr)+ Rhs: ::components::Component>);
        newt_component_partial_eq!($type<$($params)+>, <$($constr)+>);
        newt_component_drop!($type<$($params)+>, <$($constr)+>);
    };

    ($type:tt, $($tail:tt)+) => {
        parse_generics!(newt_component!($type), $($tail)+);
    };

    ($type:tt,) => {
        newt_component_base!($type);
        newt_component_partial_eq_trait!($type, <Rhs: ::components::Component>);
        newt_component_partial_eq!($type);
        newt_component_drop!($type);
    };

    ($type:tt $($tail:tt)*) => {
        newt_component!($type, $($tail)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! parse_generics {
    ($cb:ident, $type:ident, [], [], $tt:tt>) => {
        parse_generics!($cb, $type, [$tt,], [$tt,]);
    };

    ($cb:ident, $type:ident, [], [], $t1:tt: $t2:tt>) => {
        parse_generics!($cb, $type, [$t1: $t2,], [$t1,]);
    };

    ($cb:ident, $type:ident, [$($constr:tt)+], [$($params:tt)+], $tt:tt>) => {
        parse_generics!($cb, $type, [$($constr)+ $tt], [$($params)+ $tt]);
    };

    ($cb:ident, $type:ident, [$($constr:tt)+], [$($params:tt)+], $t1:tt: $t2:tt>) => {
        parse_generics!($cb, $type, [$($constr)+ $t1: $t2], [$($params)+ $t1]);
    };

    ($cb:ident, $type:ident, [$($constr:tt)+], [$($params:tt)+]) => {
        $cb ! { $type, {constr: [$($constr)+], params: [$($params)+]} }
    };

    ($cb:ident ! ($type:ident), <$($tts:tt)+) => {
        parse_generics!($cb, $type, [], [], $($tts)+);
    };

    ($cb:ident, $type:ident, [], [], $tt:tt, $($tail:tt)+) => {
        parse_generics!($cb, $type, [$tt,], [$tt,], $($tail)+);
    };

    ($cb:ident, $type:ident, [$($constr:tt)+], [$($params:tt)+], $t1:tt: $t2:tt, $($tail:tt)+) => {
        parse_generics!($cb, $type, [$($constr)+ $t1: $t2,], [$($params)+ $t1,], $($tail:tt)+);
    };

    ($cb:ident, $type:ident, [$($constr:tt)+], [$($params:tt)+], $tt:tt, $($tail:tt)+) => {
        parse_generics!($cb, $type, [$($constr)+ $tt,], [$($params)+ $tt,], $($tail)+);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! c_ptr_array_to_boxed_slice {
    ($ptr:tt [ $type:tt ], $numitems:tt) => {{
        let mut vec: Vec<$type> = Vec::new();
        if $numitems > 0 {
            let mut count = 0;
            let mut p = $ptr;
            unsafe {
                while count < $numitems {
                    vec.push($type::newt_from_ptr(*p));
                    p = p.offset(std::mem::size_of::<c_void>() as isize);
                    count += 1;
                }
            }
        }
        vec.into_boxed_slice()
    }};
}

macro_rules! newt_component_base {
    ($type:ty, $($gen:tt)*) => {
        impl $($gen)* ::components::Component for $type {
            fn co(&self) -> newtComponent {
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

macro_rules! newt_component_drop {
    (Form) => { };

    (VerticalScrollbar) => { };

    ($type:ty, $($gen:tt)*) => {
        impl $($gen)* std::ops::Drop for $type {
            fn drop(&mut self) {
                if !self.attached_to_form() {
                    unsafe {
                        ::newt_sys::newtComponentDestroy(self.co());
                    }
                }
            }
        }
    };

    ($type:ty) => {
        newt_component_drop!($type,);
    };
}

macro_rules! newt_component_partial_eq_trait {
    ($type:ty, $($gen:tt)*) => {
        impl $($gen)* std::cmp::PartialEq<Rhs> for $type {
            fn eq(&self, other: &Rhs) -> bool {
                if self.co == std::ptr::null_mut() {
                    return false
                }
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
        impl $($gen)* std::cmp::PartialEq<Box<dyn (::components::Component)>> for $type {
            fn eq(&self, other: &Box<dyn (::components::Component)>) -> bool {
                if self.co == std::ptr::null_mut() {
                    return false
                }
                self.co == other.co()
            }
        }

        impl $($gen)* std::cmp::PartialEq<::components::form::ExitReason> for $type {
            fn eq(&self, other: &::components::form::ExitReason) -> bool {
                other == self
            }
        }
    };

    ($type:ty) => {
        newt_component_partial_eq!($type,);
    };
}
