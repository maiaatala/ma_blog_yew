use wasm_bindgen::JsCast;
use web_sys::HtmlMetaElement;

pub fn api_url() -> String {
    let document = web_sys::window().unwrap().document().unwrap();
    let meta = document
        .query_selector(r#"meta[data-trunk-env="API_URL"]"#)
        .unwrap()
        .unwrap();
    let element: HtmlMetaElement = meta.dyn_into().unwrap();
    element.content()
}

pub fn test_var() -> String {
    let document = web_sys::window().unwrap().document().unwrap();
    let meta = document
        .query_selector(r#"meta[data-trunk-env="TEST_VAR"]"#)
        .unwrap()
        .unwrap();
    let element: HtmlMetaElement = meta.dyn_into().unwrap();
    element.content()
}

