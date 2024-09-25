#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bitmask_enum::bitmask;

use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/css_ffi_bindings.rs"));

#[bitmask]
pub enum MediaType {
    Aural,
    Braille,
    Embossed,
    Handheld,
    Print,
    Projection,
    Screen,
    Speech,
    TTY,
    TV,

    All = Self::Aural.bits
        | Self::Braille.bits
        | Self::Embossed.bits
        | Self::Handheld.bits
        | Self::Print.bits
        | Self::Projection.bits
        | Self::Screen.bits
        | Self::Speech.bits
        | Self::TTY.bits
        | Self::TV.bits,
}

mod jumptable_function_defaults {
    use super::*;

    pub unsafe extern "C" fn node_name(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *mut css_qname,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_classes(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        classes: *mut *mut *mut lwc_string,
        n_classes: *mut u32,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_id(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        id: *mut *mut lwc_string,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn named_ancestor_node(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *const css_qname,
        ancestor: *mut *mut ::std::os::raw::c_void,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn named_parent_node(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *const css_qname,
        parent: *mut *mut ::std::os::raw::c_void,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn named_sibling_node(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *const css_qname,
        sibling: *mut *mut ::std::os::raw::c_void,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn named_generic_sibling_node(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *const css_qname,
        sibling: *mut *mut ::std::os::raw::c_void,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn parent_node(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        parent: *mut *mut ::std::os::raw::c_void,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn sibling_node(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        sibling: *mut *mut ::std::os::raw::c_void,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_has_name(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *const css_qname,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_has_class(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        name: *mut lwc_string,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_has_id(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        name: *mut lwc_string,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_has_attribute(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *const css_qname,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_has_attribute_equal(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *const css_qname,
        value: *mut lwc_string,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_has_attribute_dashmatch(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *const css_qname,
        value: *mut lwc_string,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_has_attribute_includes(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *const css_qname,
        value: *mut lwc_string,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_has_attribute_prefix(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *const css_qname,
        value: *mut lwc_string,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_has_attribute_suffix(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *const css_qname,
        value: *mut lwc_string,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_has_attribute_substring(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        qname: *const css_qname,
        value: *mut lwc_string,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_is_root(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_count_siblings(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        same_name: bool,
        after: bool,
        count: *mut i32,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_is_empty(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_is_link(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_is_visited(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_is_hover(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_is_active(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_is_focus(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_is_enabled(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_is_disabled(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_is_checked(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_is_target(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_is_lang(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        lang: *mut lwc_string,
        match_: *mut bool,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn node_presentational_hint(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        nhints: *mut u32,
        hints: *mut *mut css_hint,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn ua_default_for_property(
        pw: *mut ::std::os::raw::c_void,
        property: u32,
        hint: *mut css_hint,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn set_libcss_node_data(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        libcss_node_data: *mut ::std::os::raw::c_void,
    ) -> css_error { todo!() }

    pub unsafe extern "C" fn get_libcss_node_data(
        pw: *mut ::std::os::raw::c_void,
        node: *mut ::std::os::raw::c_void,
        libcss_node_data: *mut *mut ::std::os::raw::c_void,
    ) -> css_error { todo!() }
}

pub trait Element: Clone {
    // type NodeType;

    fn name(&self)          -> String;
    fn classes(&self)       -> Vec<&str>;
    fn id(&self)            -> Option<String>;
    fn attributes(&self)    -> HashMap<String, String>;

    fn parent(&self)        -> Option<Box<Self>>;
    fn siblings(&self)      -> (Vec<Box<Self>>, Vec<Box<Self>>);
    
    fn pseudoclasses(&self) -> Vec<&str>;
    
    fn next_sibling(&self) -> Option<Box<Self>> {
        let (_, siblings_r) = self.siblings();
        siblings_r.get(0).cloned()
    }

    fn named_ancestor(&self, name: &str) -> Option<Box<Self>> {
        let mut ancestor = self.parent();
        while let Some(ancestor_unwrapped) = ancestor {
            if ancestor_unwrapped.name() == name {
                return Some(ancestor_unwrapped);
            }
            ancestor = ancestor_unwrapped.parent();
        }
        None
    }

    fn named_parent(&self, name: &str) -> Option<Box<Self>> {
        let parent = self.parent();
        if parent.is_none() { return None; }

        let parent_unwrapped = parent.unwrap();
        if parent_unwrapped.name() == name {
            Some(parent_unwrapped)
        }
        else {
            None
        }
    }

    fn named_sibling(&self, name: &str)         -> Option<Box<Self>> { todo!() }

    fn named_generic_sibling(&self, name: &str) -> Option<Box<Self>> { todo!() }

    fn has_name(&self, name: &str)                              -> bool { todo!() }
    fn has_class(&self, class: &str)                            -> bool { todo!() }
    fn has_id(&self, id: &str)                                  -> bool { todo!() }
    
    fn has_attribute(&self, attr: &str)                         -> bool { todo!() }
    fn has_attribute_equal(&self, attr: &str, value: &str)      -> bool { todo!() }
    fn has_attribute_dashmatch(&self, attr: &str, value: &str)  -> bool { todo!() }
    fn has_attribute_includes(&self, attr: &str, value: &str)   -> bool { todo!() }
    fn has_attribute_prefix(&self, attr: &str, value: &str)     -> bool { todo!() }
    fn has_attribute_suffix(&self, attr: &str, value: &str)     -> bool { todo!() }
    fn has_attribute_substring(&self, attr: &str, value: &str)  -> bool { todo!() }

    fn has_pseudoclass(&self, pseudoclass: &str)                -> bool { self.pseudoclasses().to_vec().contains(&pseudoclass) }

    fn is_root(&self)           -> bool { self.parent().is_none() }
    fn count_siblings(&self)    -> usize { self.siblings().0.len() + self.siblings().1.len() }
    fn is_empty(&self)          -> bool { self.has_pseudoclass("empty") }
    
    fn is_link(&self)           -> bool { self.has_pseudoclass("link") }
    fn is_visited(&self)        -> bool { self.has_pseudoclass("visited") }
    fn is_hover(&self)          -> bool { self.has_pseudoclass("hover") }
    fn is_active(&self)         -> bool { self.has_pseudoclass("active") }
    fn is_focus(&self)          -> bool { self.has_pseudoclass("focus") }

    fn is_disabled(&self)       -> bool { self.has_pseudoclass("disabled") }
    fn is_enabled(&self)        -> bool { self.has_pseudoclass("enabled") }
    fn is_checked(&self)        -> bool { self.has_pseudoclass("checked") }

    fn is_target(&self)             -> bool { self.has_pseudoclass("target") }
    fn is_lang(&self, lang: &str)   -> bool { self.has_attribute_equal("lang", lang) }

    fn jumptable(&self) -> css_select_handler {
        css_select_handler{
            handler_version: css_select_handler_version_CSS_SELECT_HANDLER_VERSION_1,

            node_name:                      Some(jumptable_function_defaults::node_name),
            node_classes:                   Some(jumptable_function_defaults::node_classes),
            node_id:                        Some(jumptable_function_defaults::node_id),

            named_ancestor_node:            Some(jumptable_function_defaults::named_ancestor_node),
            named_parent_node:              Some(jumptable_function_defaults::named_parent_node),
            named_sibling_node:             Some(jumptable_function_defaults::named_sibling_node),
            named_generic_sibling_node:     Some(jumptable_function_defaults::named_generic_sibling_node),

            parent_node:                    Some(jumptable_function_defaults::parent_node),
            sibling_node:                   Some(jumptable_function_defaults::sibling_node),

            node_has_name:                  Some(jumptable_function_defaults::node_has_name),
            node_has_class:                 Some(jumptable_function_defaults::node_has_class),
            node_has_id:                    Some(jumptable_function_defaults::node_has_id),
            node_has_attribute:             Some(jumptable_function_defaults::node_has_attribute),
            node_has_attribute_equal:       Some(jumptable_function_defaults::node_has_attribute_equal),
            node_has_attribute_dashmatch:   Some(jumptable_function_defaults::node_has_attribute_dashmatch),
            node_has_attribute_includes:    Some(jumptable_function_defaults::node_has_attribute_includes),
            node_has_attribute_prefix:      Some(jumptable_function_defaults::node_has_attribute_prefix),
            node_has_attribute_suffix:      Some(jumptable_function_defaults::node_has_attribute_suffix),
            node_has_attribute_substring:   Some(jumptable_function_defaults::node_has_attribute_substring),

            node_is_root:                   Some(jumptable_function_defaults::node_is_root),
            node_count_siblings:            Some(jumptable_function_defaults::node_count_siblings),

            node_is_empty:                  Some(jumptable_function_defaults::node_is_empty),
            node_is_link:                   Some(jumptable_function_defaults::node_is_link),
            node_is_visited:                Some(jumptable_function_defaults::node_is_visited),
            node_is_hover:                  Some(jumptable_function_defaults::node_is_hover),
            node_is_active:                 Some(jumptable_function_defaults::node_is_active),
            node_is_focus:                  Some(jumptable_function_defaults::node_is_focus),
            node_is_enabled:                Some(jumptable_function_defaults::node_is_enabled),
            node_is_disabled:               Some(jumptable_function_defaults::node_is_disabled),
            node_is_checked:                Some(jumptable_function_defaults::node_is_checked),
            node_is_target:                 Some(jumptable_function_defaults::node_is_target),
            node_is_lang:                   Some(jumptable_function_defaults::node_is_lang),

            node_presentational_hint:       Some(jumptable_function_defaults::node_presentational_hint),
            ua_default_for_property:        Some(jumptable_function_defaults::ua_default_for_property),
            set_libcss_node_data:           Some(jumptable_function_defaults::set_libcss_node_data),
            get_libcss_node_data:           Some(jumptable_function_defaults::get_libcss_node_data),
        }
    }
}

pub struct ComputedStyle {
    // native_style
}

impl ComputedStyle {
    fn apply(self) { todo!() }
}
