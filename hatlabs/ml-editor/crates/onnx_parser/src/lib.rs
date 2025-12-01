
mod onnx {
    include!(concat!(env!("OUT_DIR"), "/onnx.rs"));
}

use onnx::ModelProto;
use wasm_bindgen::prelude::*;
use web_sys::console;
use std::panic;

#[wasm_bindgen(start)]
pub fn start() {
    panic::set_hook(Box::new(|info| {
        console::error_1(&format!("Panic occurred: {info}").into());
    }));
}

/// Parses an ONNX model from binary input.
#[wasm_bindgen]
pub fn parse_onnx(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let model: ModelProto = prost::Message::decode(bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse ONNX: {}", e)))?;

    // You can convert to your own IR here.
    let summary = format!(
        "Parsed ONNX model: producer = {:?}, IR version = {:?}",
        model.producer_name,
        model.ir_version
    );

    Ok(JsValue::from_str(&summary))
}
