mod utils;

use bevy::prelude::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
    utils::log("Initializing glc-rs");
}

#[wasm_bindgen]
pub struct Glc {
    app: App,
}

impl Glc {
    pub fn new(canvas_id: &str) {
        utils::log(&format!("Creating new GLC app with canvas id: {canvas_id}"));
    }
}