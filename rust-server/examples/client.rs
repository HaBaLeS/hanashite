extern crate rust_hanashite;

use rust_hanashite::protos::hanmessage::{Auth, HanMessage};
use rust_hanashite::protos::hanmessage::mod_HanMessage::OneOfmsg;
use rust_hanashite::clienthandler::MessageParser;
use uuid::Uuid;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio_util::codec::Encoder;
use bytes::BytesMut;
use tokio::time::sleep;
use tokio::time::Duration;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let mut handles:Vec<JoinHandle<()>> =  Vec::new();
    for _ in 1..20 {
        handles.push(tokio::spawn(connection()));
    }
    futures::future::join_all(handles).await;
}

async fn connection() {
    let mut stream = TcpStream::connect("127.0.0.1:9876").await.unwrap();
    sleep(Duration::from_secs(1)).await;
    for _ in 1..2 {
        let msg = HanMessage {
            uuid: Vec::from(&Uuid::new_v4().as_bytes()[..]),
            msg: OneOfmsg::auth(Auth {
                username: "testme".to_string()
            }),
        };
        let mut parser = MessageParser {};
        let mut output = BytesMut::new();
        parser.encode(msg, &mut output).expect("Encoding Failed");
        stream.write(&output[..]).await.expect("network failed");
        sleep(Duration::from_secs(5)).await;
    }
}