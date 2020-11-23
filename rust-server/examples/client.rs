extern crate rust_hanashite;

use rust_hanashite::protos::hanmessage::*;
use rust_hanashite::protos::updmessage::*;
use rust_hanashite::protos::hanmessage::mod_HanMessage::OneOfmsg;
use rust_hanashite::clienthandler::MessageParser;
use rust_hanashite::udphandler::UdpMessageParser;
use uuid::Uuid;
use tokio::net::{TcpStream, UdpSocket};
use tokio_util::codec::{Framed, Encoder};
use tokio::time::sleep;
use tokio::time::Duration;
use tokio::task::JoinHandle;
use futures::SinkExt;
use tokio::stream::StreamExt;
use std::sync::Arc;
use bytes::{BytesMut, Buf};

#[tokio::main]
async fn main() {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let udp_socket = Arc::new(UdpSocket::bind("127.0.0.1:9877").await.unwrap());
    udp_socket.connect("127.0.0.1:9876").await.unwrap();
    for _ in 1..20 {
        handles.push(tokio::spawn(connection(udp_socket.clone())));
    }
    futures::future::join_all(handles).await;
}

async fn connection(udp_socket: Arc<UdpSocket>) {
    let stream = TcpStream::connect("127.0.0.1:9876").await.unwrap();
    let mut codec = UdpMessageParser {};
    let mut framed = Framed::new(stream, MessageParser {});
    sleep(Duration::from_secs(1)).await;
    for _ in 1..2 {
        framed.send(HanMessage {
            uuid: Vec::from(&Uuid::new_v4().as_bytes()[..]),
            msg: OneOfmsg::auth(Auth {
                username: "testme".to_string()
            }),
        }).await.expect("Send Failed");
        let connection_id: Uuid = if let Some(Ok(msg)) = framed.next().await {
            println!("Received {:?}", msg);
            if let OneOfmsg::auth_result(result) = msg.msg {
                Uuid::from_slice(result.connection_id.as_slice()).unwrap()
            } else {
                return;
            }
        } else {
            return;
        };
        framed.send(HanMessage {
            uuid: Vec::from(&Uuid::new_v4().as_bytes()[..]),
            msg: OneOfmsg::chan_join(ChannelJoin {
                name: "testchannel".to_string(),
                uuid: vec![0, 0],
            }),
        }).await.expect("Send Failed");
        if let Some(msg) = framed.next().await {
            println!("Received {:?}", msg);
        }
        let mut buf = BytesMut::new();
        codec.encode(HanUdpMessage {
            user_id: Vec::from(&connection_id.as_bytes()[..]),
            audio_frame: None

        }, &mut buf).expect("Encoder broken");
        udp_socket.send(buf.bytes()).await.expect("Udp Failed");
        sleep(Duration::from_secs(10)).await;

    }
}