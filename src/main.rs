#![allow(dead_code)]
use apifs::ApiFS;

fn main() {
    env_logger::init();

    let api_fs = ApiFS::init("./api-test/api.yml").unwrap();
    println!("{:#?}", api_fs);
}
