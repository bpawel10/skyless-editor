use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Progress {
    #[wasm_bindgen(readonly)]
    pub progress: f32,
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub label: Option<String>,
}
