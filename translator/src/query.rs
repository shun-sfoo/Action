use async_trait::async_trait;

use crate::model::{
    baidu::{Baidu, BaiduText},
    caiyun::{Caiyun, CaiyunText},
    youdao::{Youdao, YoudaoText},
};

#[async_trait]
pub trait Query {
    async fn query(&self);
}

#[async_trait]
impl Query for Youdao {
    async fn query(&self) {
        let headers = (&self.headers).try_into().expect("youdao valid headers");
        let resp = reqwest::Client::new()
            .post(&self.url)
            .headers(headers)
            .form(&self)
            .send()
            .await
            .expect("youdao post request error");

        let text: YoudaoText =
            serde_json::from_str(&resp.text().await.expect("get youdao search error")).unwrap();

        println!("{}", text);
    }
}

#[async_trait]
impl Query for Baidu {
    async fn query(&self) {
        let headers = (&self.headers).try_into().expect("baidu valid headers");
        let resp = reqwest::Client::new()
            .post(&self.url)
            .headers(headers)
            .form(&self)
            .send()
            .await
            .expect("baidu post request error");

        let text: BaiduText =
            serde_json::from_str(&resp.text().await.expect("get youdao search error")).unwrap();

        println!("{}", text);
    }
}

#[async_trait]
impl Query for Caiyun {
    async fn query(&self) {
        let headers = (&self.headers).try_into().expect("caiyun valid headers");

        let body = serde_json::to_string(self).unwrap();

        let resp = reqwest::Client::new()
            .post(&self.url)
            .headers(headers)
            .body(body)
            .send()
            .await
            .expect("caiyun request url error");

        let text: CaiyunText =
            serde_json::from_str(&resp.text().await.expect("get youdao search error")).unwrap();

        println!("{}", text);
    }
}
