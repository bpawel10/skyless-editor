use super::AttributesType;
use rkyv::{Archive, Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Default, Clone, Archive, Deserialize, Serialize)]
pub struct Entity {
    pub attributes: AttributesType,
}

impl Entity {
    pub fn new() -> Self {
        Self::default()
    }
}

// TODO: move it to macros crate and define using #[proc_macro] instead
#[macro_export]
macro_rules! entity {
    ($($attribute:expr),*) => {
        {
            let mut attributes = HashMap::new();
            $(
                attributes.insert(
                    $attribute.as_name().to_string(),
                    Box::new($attribute) as Box<dyn Attribute>,
                );
            )*
            Entity { attributes }
        }
    };
}
