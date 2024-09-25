#![allow(unused_variables)]
#![allow(unused_unsafe)]

pub mod ffi;
pub mod error;

use std::rc::{ Rc, Weak };
use std::cell::RefCell;
use std::ffi::OsString;
use std::ptr::null_mut;

use error::Error;

pub struct Stylesheet {
    raw: *mut ffi::css_stylesheet,

    pub dom: Weak<StyleCollection>,
    
}

pub type StyleParameters = ffi::css_stylesheet_params;

pub struct StyleCollection {
    _ctx: *mut ffi::css_select_ctx,

    params: *const ffi::css_stylesheet_params,
    pub sheets: RefCell<Vec<Stylesheet>>,
}

// =========== IMPLs ===========

impl Stylesheet {
    fn new(dom: Rc<StyleCollection>, source: String) -> Result<Self, Error> {
        unsafe {
            let mut raw_stylesheet: *mut ffi::css_stylesheet = null_mut();

            let result = ffi::css_stylesheet_create(dom.params, &mut raw_stylesheet);
            if result != ffi::css_error_CSS_OK {
                return Err(Error::Native(result.into()));
            }

            Ok(Self{
                dom: Rc::downgrade(&dom),
                raw: raw_stylesheet
            })
        }
    }
}

impl std::ops::Drop for Stylesheet {
    fn drop(&mut self) {
        unsafe {
            ffi::css_stylesheet_destroy(self.raw);
        }
    }
}

impl StyleCollection {
    pub fn new() -> Result<Rc<StyleCollection>, Error> {
        let mut select_ctx: *mut ffi::css_select_ctx = null_mut();

        unsafe {
            let result = ffi::css_select_ctx_create(&mut select_ctx);
            if result != ffi::css_error_CSS_OK {
                return Err(Error::Native(result.into()));
            }
        }

        Ok(StyleCollection{
            _ctx: select_ctx,
            params: null_mut(),
            sheets: RefCell::new(Vec::new())
        }.into())
    }
    pub fn read_from_file(self: &Rc<StyleCollection>, filepath: OsString) -> Result<(), error::Error> {
        todo!()
    }

    pub fn read_from_source(self: &Rc<StyleCollection>, source: String) -> Result<(), error::Error> {
        let sheet_result = Stylesheet::new(self.clone(), source);

        match sheet_result {
            Err(err) => Err(err),
            Ok(sheet) => {
                self.clone().sheets.borrow_mut().push(sheet);
                Ok(())
            }
        }
    }

    pub fn compute_style_for(
        &self,
        element: impl ffi::Element,
        media: ffi::MediaType,
        inline_style: &Stylesheet,

    ) -> ffi::ComputedStyle {
        todo!()
    }
}

impl std::ops::Drop for StyleCollection {
    fn drop(&mut self) {
        unsafe {
            ffi::css_select_ctx_destroy(self._ctx);
        }
    }
}
