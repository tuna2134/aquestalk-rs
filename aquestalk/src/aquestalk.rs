// Aquestalk core
use std::{error, ffi::CString, fmt, slice};

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// This error about aquestalk. Please watch aquestalk official [documentaion](https://www.a-quest.com/archive/manual/prog_guide_linux.pdf).
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

#[cfg(feature = "aquestalk1")]
pub fn synthe_aquestalk1(text: &str, speed: i32) -> Result<Vec<u8>> {
    let size: i32 = 0;
    let content: CString = CString::new(text).unwrap();
    let wav: *const u8 =
        unsafe { libaquestalk_sys::ffi::AquesTalk_Synthe_Utf8(content.as_ptr(), speed, &size as *const i32) };
    if wav.is_null() {
        return Err(Box::new(AquesTalkError::Error(size)));
    }
    unsafe {
        libaquestalk_sys::ffi::AquesTalk_FreeWave(wav);
    };
    return unsafe { Ok(slice::from_raw_parts(wav, size.try_into().unwrap()).to_vec()) };
}
