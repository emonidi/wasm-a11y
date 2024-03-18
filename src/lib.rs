use accessibility_rs::{audit, AuditConfig, Conformance};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn check_a11y(element_html: String, styles: String) -> Vec<JsValue> {
    let html = format!(r#"<html lang="en"><head><title>Test</title></head><body>{}</body></html>"#, element_html);

    let mut config = AuditConfig::new(&html, styles.as_str(), true, "en");
    config.bounding_box = true;
    config.conformance = Conformance::WCAGAAA;
    
    let audit = audit(config);
    let mut result_vec: Vec<JsValue> = vec![];
    audit.into_iter().for_each(|issue| {
        let serialized = serde_wasm_bindgen::to_value(&issue).unwrap();
        result_vec.push(serialized)
    });

    return result_vec;
    
}
