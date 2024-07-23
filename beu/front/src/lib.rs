#[allow(unused_imports)]
use app::*;

use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    leptos_dom::HydrationCtx::stop_hydrating();
}
