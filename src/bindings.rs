use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Serialize, Deserialize)]
pub struct HighlightOptions {
    pub language: String,
    #[serde(rename(serialize = "ignoreIllegals"))]
    pub ignore_illegals: bool,
}

#[wasm_bindgen(module = "https://unpkg.com/@highlightjs/cdn-assets@11.6.0/es/highlight.min.js")]
extern "C" {
    #[wasm_bindgen(js_name = "default")]
    pub type Hljs;

    #[wasm_bindgen]
    pub type HighlightResult;

    #[wasm_bindgen(static_method_of = Hljs, js_name = "highlightAll", js_class = "default")]
    pub fn highlight_all();

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &HighlightResult) -> String;

    #[wasm_bindgen(static_method_of = Hljs, js_class = "default")]
    pub fn highlight(code: &str, options: &JsValue) -> HighlightResult;
}
