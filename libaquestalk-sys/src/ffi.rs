use libc::{c_char, c_int, c_uchar, c_void};

#[link(name = "AquesTalk", kind = "dylib")]
extern "C" {
    pub fn AquesTalk_Synthe_Utf8(
        koe: *const c_char,
        iSpeed: c_int,
        size: *const c_int,
    ) -> *const c_uchar;

    pub fn AquesTalk_FreeWave(wav: *const c_uchar) -> c_void;
}
