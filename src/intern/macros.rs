//
// Copyright (C) 2019  Robert Gill <locke@sdf.lonestar.org>
//
// This file is a part of newt-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 2.1 as published by the Free Software Foundation.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

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
