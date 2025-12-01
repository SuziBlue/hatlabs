
use std::io::Cursor;
use gloo::console::log;
use prost::Message;
use onnx_ir::OnnxGraph;

/// Parses an ONNX model from in-memory bytes (for use in WASM).
pub fn parse_onnx_bytes(bytes: &[u8]) -> OnnxGraph {
    log::info!("Parsing ONNX model from memory buffer");

    // Parse ONNX protobuf from bytes
    let mut cursor = Cursor::new(bytes);
    let onnx_model: ModelProto = Message::parse_from_reader(&mut cursor)
        .expect("Unable to parse ONNX model from bytes");

    // Check opset versions
    if !verify_opsets(&onnx_model.opset_import, MIN_OPSET_VERSION) {
        panic!(
            "Unsupported ONNX opset version. Requires opset >= {MIN_OPSET_VERSION}. \
             Use ONNX shape inference tools to upgrade your model."
        );
    }

    debug_assert!(
        onnx_model.graph.node.is_top_sorted(),
        "Nodes are not topologically sorted"
    );

    log::debug!("Parsed ONNX model with {} nodes", onnx_model.graph.node.len());

    for opset in &onnx_model.opset_import {
        log::debug!(
            "Opset domain: {:?}, version: {:?}",
            if opset.domain.is_empty() {
                "<default>"
            } else {
                &opset.domain
            },
            opset.version
        );
    }

    let builder = OnnxGraphBuilder::default();
    let graph = builder.build(&onnx_model);

    log::info!("Finished parsing ONNX model from memory buffer");

    graph
}
