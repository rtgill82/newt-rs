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
                    p = p.add(std::mem::size_of::<c_void>());
                    count += 1;
                }
            }
        }
        vec.into_boxed_slice()
    }};
}
