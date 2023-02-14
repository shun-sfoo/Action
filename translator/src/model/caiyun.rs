use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Caiyun {
    #[serde(skip)]
    pub url: String,
    #[serde(skip)]
    pub headers: HashMap<String, String>,
    source: String,
    trans_type: String,
    request_id: String,
}

impl Caiyun {
    pub fn new(content: &str) -> Self {
        let token = dotenv!("CAIYUN_TOKEN");
        let token = format!("token {}", token);
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        headers.insert("x-authorization".to_string(), token);

        Self {
            url: "http://api.interpreter.caiyunai.com/v1/translator".into(),
            headers,
            source: content.into(),
            trans_type: "en2zh".into(),
            request_id: "demo".into(),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CaiyunText {
    rc: i32,
    target: String,
    confidence: f64,
    // src_tgt unkonw
    #[serde(skip)]
    isdict: i32,
}

impl std::fmt::Display for CaiyunText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = format!("Caiyun: {}", &self.target);
        f.write_str(&result)
    }
}
