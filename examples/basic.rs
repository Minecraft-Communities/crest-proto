#![allow(unused_imports)]

use std::collections::{ HashMap, BTreeMap };

use crest_proto as crest;

#[derive(Clone, PartialEq)]
pub struct MyNode {
    pub text_content: Option<String>,

    pub prefix: Option<String>,
    pub namespace: Option<String>,
    pub namespaces: Option<BTreeMap<String, String>>,

    pub name: String,
    pub attributes: HashMap<String, String>,

    pub children: Vec<Box<Self>>,
    pub parent: Option<Box<Self>>,
}

impl crest::ffi::Element for MyNode {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn classes(&self) -> Vec<&str> {
        let classes_result = self.attributes.get("class");
        let classes_raw = classes_result.map_or("", String::as_str);
        classes_raw.split_whitespace().collect()
    }

    fn id(&self) -> Option<String> {
        self.attributes.get("id").cloned()
    }

    fn attributes(&self) -> HashMap<String, String> {
        self.attributes.clone()
    }

    fn parent(&self) -> Option<Box<Self>> {
        self.parent.clone()
    }

    fn siblings(&self) -> (Vec<Box<Self>>, Vec<Box<Self>>) {
        match &self.parent {
            Some(parent) => {
                let mut siblings_l: Vec<Box<Self>> = Vec::new();
                let mut siblings_r: Vec<Box<Self>> = Vec::new();
                let mut found_self = false;

                for sibling in parent.children.iter() {
                    if &**sibling == self {
                        found_self = true;
                        continue;
                    }

                    if !found_self {
                        siblings_l.push(sibling.clone());
                    }
                    else {
                        siblings_r.push(sibling.clone());
                    }
                }

                (siblings_l, siblings_r)
            },
            None => (Vec::new(), Vec::new())
        }
    }

    fn pseudoclasses(&self) -> Vec<&str> {
        todo!()
    }
}

fn main() {

    todo!()
}
