use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use crypto::{digest::Digest, sha2::Sha256};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Youdao {
    #[serde(skip)]
    pub url: String,
    #[serde(skip)]
    pub headers: HashMap<String, String>,
    from: String,
    to: String,
    curtime: String,
    #[serde(rename(serialize = "appKey"))]
    app_key: String,
    q: String,
    salt: String,
    sign: String,
    #[serde(rename(serialize = "signType"))]
    sign_type: String,
    #[serde(rename(serialize = "vocabId"))]
    vocab_id: String,
}

impl Youdao {
    pub fn new(content: &str) -> Self {
        let curtime = SystemTime::now();
        let duration = curtime
            .duration_since(UNIX_EPOCH)
            .expect("timestap covert duration error");
        let duration = duration.as_secs();
        let curtime = duration.to_string();
        let salt = Uuid::new_v4().to_string();

        let len = content.len();

        let input = if len > 20 {
            let head = (&content[..10]).to_string();
            let tail = (&content[len - 10..]).to_string();
            head + &len.to_string() + &tail
        } else {
            (&content).to_string()
        };

        let app_key = dotenv!("YOUDAO_APP_ID");
        let app_secert = dotenv!("YOUDAO_APP_SECRET");

        let sign_str = (&app_key).to_string() + &input + &salt + &curtime + &app_secert;

        let mut hasher = Sha256::new();
        hasher.input_str(&sign_str);
        let sign = hasher.result_str();

        let mut headers = HashMap::new();

        headers.insert(
            "content_type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        Self {
            url: "https://openapi.youdao.com/api".into(),
            headers,
            q: content.into(),
            from: "auto".into(),
            to: "auto".into(),
            app_key: app_key.into(),
            salt,
            sign,
            sign_type: "v3".into(),
            curtime,
            vocab_id: "1".into(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct YoudaoText {
    pub translation: Vec<String>,
}

impl std::fmt::Display for YoudaoText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = format!("Youdao: {}", self.translation[0]);
        f.write_str(&result)
    }
}
