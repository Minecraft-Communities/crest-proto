
use derive_more::From;

use std::ffi::CStr;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct NativeError(pub super::ffi::css_error);

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Native(NativeError),

    #[from]
    Generic(String),
}

impl core::fmt::Display for NativeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let err_string: *const ::std::os::raw::c_char = super::ffi::css_error_to_string(self.0);
            
            let result: String = if err_string.is_null() {
                String::new()
            }
            else {
                CStr::from_ptr(err_string).to_string_lossy().into_owned()
            };
            write!(f, "{result}")
        }
    }
}

impl From<super::ffi::css_error> for NativeError {
    fn from(value: super::ffi::css_error) -> Self {
        NativeError(value)
    }
}

impl std::ops::Deref for NativeError {
    type Target = super::ffi::css_error;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.0
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

