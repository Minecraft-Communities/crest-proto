#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/css_ffi_bindings.rs"));

use std::ffi::OsString;
use std::sync::Arc;
use std::fs;

pub struct StyleSelector {

}

pub type StyleRule = (String, Option<String>);

#[derive(Clone)]
pub struct Stylesheet {

}

pub struct StylesheetCollection(Vec<Stylesheet>);

impl StylesheetCollection {
    fn collect(self) -> Arc<[Stylesheet]> { self.0[..].into() }

    fn add_sheet_from_file(&mut self, filepath: OsString) -> Result<(), std::error::Error> {
        let source = fs::read_to_string(filepath).expect(format!("Failed to read CSS file: '{}'", filepath.to_str().unwrap()));

        todo!()
    }

    fn add_sheet_from_string(&mut self, source: String) -> Result<(), std::error::Error> {
        todo!()
    }
}
