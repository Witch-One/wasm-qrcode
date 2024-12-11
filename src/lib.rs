mod utils;

extern crate base64;
extern crate wasm_bindgen;
use bardecoder;
use bardecoder::prepare::BlockedMean;
use base64::Engine as _;

use image;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(base64_image: &str) -> String {
    let decoded_data = match base64::engine::general_purpose::STANDARD.decode(base64_image) {
        Ok(data) => data,
        Err(e) => {
            return format!("Failed to decode base64 image:{:?}", e);
        }
    };

    let img = match image::load_from_memory(&decoded_data) {
        Ok(img) => img,
        Err(e) => {
            return format!("Failed to load image:{:?}", e);
        }
    };

    let mut db = bardecoder::default_builder();

    db.prepare(Box::new(BlockedMean::new(7, 9)));

    let decoder = db.build();
    let results = decoder.decode(&img);
    let mut res = String::new();
    for result in results {
        let s = format!("{}", result.unwrap());
        res += s.as_str();
    }
    return format!("From 57:{:?} results found", res);
}
