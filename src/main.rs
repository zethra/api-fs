#![allow(dead_code)]
use actix::prelude::*;
use apifs::ApiFS;
use failure::Error;
use futures::{future, Future, Stream};
// use std::future::Future;
use bytebuffer::ByteBuffer;
use reqwest::r#async::{Client, Decoder};
use std::io::{self, Cursor};
use std::mem;

struct Msg(bool);
impl Message for Msg {
    type Result = Result<(), ()>;
}
struct AA;
impl Actor for AA {
    type Context = Context<AA>;
}
impl Handler<Msg> for AA {
    type Result = Box<Future<Item = (), Error = ()>>;
    fn handle(&mut self, msg: Msg, _ctx: &mut Context<Self>) -> Self::Result {
        Box::from(
            Client::new()
                .get("http://0.0.0.0:8000/big.txt")
                .send()
                .and_then(|mut res| {
                    println!("{}", res.status());
                    let body = mem::replace(res.body_mut(), Decoder::empty());
                    body.concat2()
                })
                .map_err(|err| println!("request error: {}", err))
                // .fold(ByteBuffer::new(), |acc, chunk| {
                // })
                .map(|body| {
                    // Whole response is one chunk
                    let mut body = Cursor::new(body);
                    println!("\nCHUNK");
                    let _ = io::copy(&mut body, &mut io::stdout()).map_err(|err| {
                        println!("stdout error: {}", err);
                    });
                }),
        )
        // Box::from(future::ok("Test".to_owned()))
    }
}

fn main() {
    env_logger::init();

    let sys = System::new("example");

    // Start MyActor in current thread
    let addr = AA.start();

    let result = addr.send(Msg(true));

    Arbiter::spawn(
        result
            .map(|res| match res {
                // Ok(result) => println!("Got result: {}", result),
                Ok(result) => {}
                Err(err) => println!("Got error: {:?}", err),
            })
            .map_err(|e| {
                println!("Actor is probably died: {}", e);
            }),
    );

    // let result = addr.send(Msg(false));

    // Arbiter::spawn(
    //     result
    //         .map(|res| match res {
    //             // Ok(result) => println!("Got result: {}", result),
    //             Ok(result) => {},
    //             Err(err) => println!("Got error: {:?}", err),
    //         })
    //         .map_err(|e| {
    //             println!("Actor is probably died: {}", e);
    //         }),
    // );

    sys.run();

    // let api_fs = ApiFS::init("./api-test/api.yml").unwrap();
    // println!("{:#?}", api_fs);
}
