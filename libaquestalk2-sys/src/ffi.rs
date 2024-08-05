use libc::{c_char, c_int, c_uchar, c_void};

#[link(name = "AquesTalk2Eva", kind = "dylib")]
extern "C" {
    pub fn AquesTalk2_Synthe_Utf8(
        koe: *const c_char,
        iSpeed: c_int,
        size: *const c_int,
        phontDat: *const c_void,
    ) -> *const c_uchar;

    pub fn AquesTalk2_FreeWave(wav: *const c_uchar) -> c_void;
}
