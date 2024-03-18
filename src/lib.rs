use std::convert::TryInto;

use accessibility_rs::engine::audit::auditor::Auditor;
use accessibility_rs::{audit, engine, AuditConfig, Conformance};

use serde::Serializer;
use serde::{de::Deserialize, ser::Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;
use web_sys::js_sys::{Array, Reflect};
use web_sys::{console, StyleSheet};
// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn check_a11y(elementHtml: String, styles: String) -> Vec<JsValue> {
    let html = format!(r#"<html lang="en"><head><title>Test</title></head><body>{}</body></html>"#, elementHtml);

    let mut config = AuditConfig::new(&html, styles.as_str(), true, "en");
    config.bounding_box = true;
    config.conformance = Conformance::WCAGAAA;
    log_1(&JsValue::from_str(&config.html.to_string()));

    let audit = audit(config);
    let mut result_vec: Vec<JsValue> = vec![];
    audit.into_iter().for_each(|issue| {
        let serialized = serde_wasm_bindgen::to_value(&issue).unwrap();
        result_vec.push(serialized)
    });

    return result_vec;
    
}
