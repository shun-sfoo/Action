use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use crypto::{digest::Digest, md5::Md5};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Baidu {
    #[serde(skip)]
    pub url: String,
    #[serde(skip)]
    pub headers: HashMap<String, String>,
    appid: String,
    q: String,
    from: String,
    to: String,
    salt: String,
    sign: String,
}

impl Baidu {
    pub fn new(content: &str) -> Self {
        let appid = dotenv!("BAIDU_APP_ID");
        let app_secret = dotenv!("BAIDU_APP_SECRET");

        let curtime = SystemTime::now();
        let duration = curtime
            .duration_since(UNIX_EPOCH)
            .expect("timestap covert duration error");

        let duration = duration.as_secs();
        let salt = duration.to_string();

        let sign_str = (&appid).to_string() + &content + &salt + &app_secret;
        let mut hasher = Md5::new();
        hasher.input_str(&sign_str);
        let sign = hasher.result_str();

        let mut headers = HashMap::new();
        headers.insert(
            "content_type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        Self {
            url: "http://api.fanyi.baidu.com/api/trans/vip/translate".to_string(),
            q: (&content).to_string(),
            from: "en".to_string(),
            to: "zh".to_string(),
            headers,
            salt,
            appid: appid.into(),
            sign,
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct BaiduText {
    from: String,
    to: String,
    trans_result: Vec<TR>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct TR {
    src: String,
    dst: String,
}

impl std::fmt::Display for BaiduText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = format!("Baidu: {}", &self.trans_result[0].dst);
        f.write_str(&result)
    }
}
