use super::{Offset, Texture, Textures};
use rkyv::{Archive, Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub struct Item {
    pub id: u16,
    #[wasm_bindgen(getter_with_clone)]
    pub name: Option<String>,
    // minimap:
    #[wasm_bindgen(skip)]
    pub ground: bool,
    #[wasm_bindgen(skip)]
    pub stackable: bool,
    #[wasm_bindgen(skip)]
    pub splash: bool,
    #[wasm_bindgen(skip)]
    pub fluid_container: bool,
    #[wasm_bindgen(skip)]
    pub draw_offset: Offset,
    #[wasm_bindgen(skip)]
    pub height_offset: Offset,
    #[wasm_bindgen(skip)]
    pub textures: Textures,
}

#[wasm_bindgen]
impl Item {
    #[wasm_bindgen(getter)]
    pub fn texture(&self) -> Texture {
        self.textures.get_default().clone()
    }

    #[wasm_bindgen(getter, js_name = allTextures)]
    pub fn all_textures(&self) -> Vec<JsValue> {
        self.textures
            .get_all()
            .into_iter()
            .map(JsValue::from)
            .collect()
    }
}
