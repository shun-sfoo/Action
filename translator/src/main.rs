use std::env;

use model::{baidu::Baidu, caiyun::Caiyun, youdao::Youdao};
use query::Query;

mod model;
mod query;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    let content = env::args().into_iter().nth(1).unwrap();
    let youdao = Youdao::new(&content);
    let caiyun = Caiyun::new(&content);
    let baidu = Baidu::new(&content);

    let r1 = tokio::spawn(async move { youdao.query().await });
    let r2 = tokio::spawn(async move { caiyun.query().await });
    let r3 = tokio::spawn(async move { baidu.query().await });

    r1.await.unwrap();
    r2.await.unwrap();
    r3.await.unwrap();
}
