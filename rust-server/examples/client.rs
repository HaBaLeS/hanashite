extern crate rust_hanashite;

use std::sync::Arc;

use bytes::{Buf, BytesMut};
use futures::SinkExt;
use tokio::net::{TcpStream, UdpSocket};
use tokio::stream::StreamExt;
use tokio::task::JoinHandle;
use tokio::time::Duration;
use tokio::time::sleep;
use tokio_util::codec::{Decoder, Encoder, Framed};
use uuid::Uuid;

use rust_hanashite::clienthandler::MessageParser;
use rust_hanashite::protos::hanmessage::*;
use rust_hanashite::protos::hanmessage::mod_HanMessage::OneOfmsg;
use rust_hanashite::protos::updmessage::*;
use rust_hanashite::udphandler::UdpMessageParser;

#[tokio::main]
async fn main() {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    tokio::spawn(listener());
    sleep(Duration::from_secs(1)).await;
    for port in 9879..9980 {
        handles.push(tokio::spawn(connection(port)));
    }
    futures::future::join_all(handles).await;
}

async fn listener() {
    let udp_socket = Arc::new(UdpSocket::bind("0.0.0.0:9878").await.unwrap());
    udp_socket.connect("127.0.0.1:9876").await.unwrap();
    let stream = TcpStream::connect("127.0.0.1:9876").await.unwrap();
    let mut codec = UdpMessageParser {};
    let mut framed = Framed::new(stream, MessageParser {});
    framed.send(HanMessage {
        message_id: Vec::from(&Uuid::new_v4().as_bytes()[..]),
        msg: OneOfmsg::auth(Auth {
            username: "testme".to_string()
        }),
    }).await.expect("Send Failed");
    let connection_id: Uuid = if let Some(Ok(msg)) = framed.next().await {
        if let OneOfmsg::auth_result(result) = msg.msg {
            Uuid::from_slice(result.connection_id.as_slice()).unwrap()
        } else {
            return;
        }
    } else {
        return;
    };
    framed.send(HanMessage {
        message_id: Vec::from(&Uuid::new_v4().as_bytes()[..]),
        msg: OneOfmsg::chan_join(ChannelJoin {
            name: "testchannel".to_string(),
            channel_id: vec![0, 0],
        }),
    }).await.expect("Send Failed");
    let channel_id: Uuid = if let Some(Ok(msg)) = framed.next().await {
        if let OneOfmsg::chan_join_result(result) = msg.msg {
            Uuid::from_slice(result.channel_id.as_slice()).unwrap()
        } else {
            return;
        }
    } else {
        return;
    };
    let mut buf = BytesMut::new();
    let udp_message = HanUdpMessage {
        user_id: Vec::from(&connection_id.as_bytes()[..]),
        msg: mod_HanUdpMessage::OneOfmsg::ping_packet(PingPacket {}),
    };
    codec.encode(udp_message, &mut buf).expect("Encoder broken");
    udp_socket.send(buf.bytes()).await.expect("Udp Failed");
    let mut buf: Vec<u8> = vec![0 as u8; 8152];
    loop {
        buf.resize(8152, 0);
        let result1 = tokio::select! {
            cnt = udp_socket.recv(buf.as_mut_slice()) => cnt.unwrap(),
            _ = sleep(Duration::from_secs(5)) => return
        };
        buf.resize(result1, 0);
        println!("{:?}", codec.decode(&mut BytesMut::from(buf.as_slice())));
    }
}

async fn connection(port: u16) {
    let udp_addr = format!("0.0.0.0:{}", port);
    let udp_socket = Arc::new(UdpSocket::bind(udp_addr).await.unwrap());
    udp_socket.connect("127.0.0.1:9876").await.unwrap();
    let stream = TcpStream::connect("127.0.0.1:9876").await.unwrap();
    let mut codec = UdpMessageParser {};
    let mut framed = Framed::new(stream, MessageParser {});
    sleep(Duration::from_secs(1)).await;
    framed.send(HanMessage {
        message_id: Vec::from(&Uuid::new_v4().as_bytes()[..]),
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
        message_id: Vec::from(&Uuid::new_v4().as_bytes()[..]),
        msg: OneOfmsg::chan_join(ChannelJoin {
            name: "testchannel".to_string(),
            channel_id: vec![0, 0],
        }),
    }).await.expect("Send Failed");
    let channel_id: Uuid = if let Some(Ok(msg)) = framed.next().await {
        println!("Received {:?}", msg);
        if let OneOfmsg::chan_join_result(result) = msg.msg {
            Uuid::from_slice(result.channel_id.as_slice()).unwrap()
        } else {
            return;
        }
    } else {
        return;
    };
    let mut buf = BytesMut::new();
    let udp_message = HanUdpMessage {
        user_id: Vec::from(&connection_id.as_bytes()[..]),
        msg: mod_HanUdpMessage::OneOfmsg::ping_packet(PingPacket {}),
    };
    codec.encode(udp_message, &mut buf).expect("Encoder broken");
    udp_socket.send(buf.bytes()).await.expect("Udp Failed");
    for i in 1..20 {
        let mut buf = BytesMut::new();
        codec.encode(HanUdpMessage {
            user_id: Vec::from(&connection_id.as_bytes()[..]),
            msg: mod_HanUdpMessage::OneOfmsg::audio_frame(AudioPacket {
                channel_id: Vec::from(&channel_id.as_bytes()[..]),
                data: vec![1, 2, 3],
                sequence_id: i,
            }),
        }, &mut buf).expect("Encoder broken");
        udp_socket.send(buf.bytes()).await.expect("Udp Failed");
    }
    sleep(Duration::from_secs(10)).await;
}