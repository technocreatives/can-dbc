extern crate can_dbc;
extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use can_dbc::DBC;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn from_dbc(dbc_in: &str) -> JsValue {
    let dbc = DBC::from_slice(dbc_in.as_bytes()).expect("Failed to parse dbc file");
    JsValue::from_serde(&dbc).unwrap()
}
