use std::ffi::CString;

pub fn char_slice_to_cstring(slice: &[char]) -> CString {
    let mut vec: Vec<u8> = Vec::new();
    for c in slice.iter() { vec.push(*c as u8); }
    let string = String::from_utf8_lossy(vec.as_slice());
    CString::new(string.into_owned()).unwrap()
}
