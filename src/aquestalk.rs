use libc::{c_char, c_int, c_uchar, c_void};
use std::{error, ffi::CString, fmt, slice};

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[link(name = "AquesTalk", kind = "dylib")]
extern "C" {
    fn AquesTalk_Synthe_Utf8(
        koe: *const c_char,
        iSpeed: c_int,
        size: *const c_int,
    ) -> *const c_uchar;

    fn AquesTalk_FreeWave(wav: *const c_uchar) -> c_void;
}

#[derive(Debug)]
pub enum AquesTalkError {
    Error(i32),
}

impl fmt::Display for AquesTalkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AquesTalkError::Error(e) => write!(f, "AquesTalkError: {}", e),
        }
    }
}

impl std::error::Error for AquesTalkError {}

pub fn synthe(text: &str, speed: i32) -> Result<Vec<u8>> {
    let size: i32 = 0;
    let content: CString = CString::new(text).unwrap();
    et wav: *const u8 =
        unsafe { AquesTalk_Synthe_Utf8(content.as_ptr(), speed, &size as *const i32) };
    if wav.is_null() {
        return Err(Box::new(AquesTalkError::Error(size)));
    }
    unsafe {
        AquesTalk_FreeWave(wav);
    };
    return unsafe { Ok(slice::from_raw_parts(wav, size.try_into().unwrap()).to_vec()) };
}
