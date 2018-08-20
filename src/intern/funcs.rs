use std::ffi::CString;

pub fn char_slice_to_cstring(slice: &[char]) -> CString {
    let mut vec: Vec<u8> = Vec::new();
    for c in slice.iter() {
        let mut b = [0; 1];
        let ch = c.encode_utf8(&mut b);
        vec.push(ch.as_bytes()[0]);
    }

    let string = String::from_utf8_lossy(vec.as_slice());
    let cstr = CString::new(string.into_owned()).unwrap();
    return cstr;
}
