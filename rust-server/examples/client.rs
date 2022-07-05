extern crate rust_hanashite;

use std::sync::Arc;

use bytes::{BytesMut};
use futures::SinkExt;
use futures::StreamExt;
use tokio::net::{TcpStream, UdpSocket};
use tokio::task::JoinHandle;
use tokio::time::Duration;
use tokio::time::sleep;
use tokio_util::codec::{Encoder, Framed};
use uuid::Uuid;

use rust_hanashite::clienthandler::MessageParser;
use rust_hanashite::protos::hanmessage::*;
use rust_hanashite::protos::hanmessage::han_message::Msg;
use rust_hanashite::protos::udpmessage::*;
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
    framed.send(Box::new(HanMessage {
        message_id: Vec::from(&Uuid::new_v4().as_bytes()[..]),
        msg: Some(Msg::Auth(Auth {
            username: "testme".to_string()
        })) ,
    })).await.expect("Send Failed");
    let connection_id: Uuid = if let Some(Ok(msg)) = framed.next().await {
        if let Some(Msg::AuthResult(result)) = msg.msg {
            Uuid::from_slice(result.connection_id.as_slice()).unwrap()
        } else {
            return;
        }
    } else {
        return;
    };
    framed.send(Box::new(HanMessage {
        message_id: Vec::from(&Uuid::new_v4().as_bytes()[..]),
        msg: Some(Msg::ChanJoin(ChannelJoin {
            name: "testchannel".to_string(),
        })),
    })).await.expect("Send Failed");
    let _channel_id: Uuid = if let Some(Ok(msg)) = framed.next().await {
        if let Some(Msg::ChanJoinResult(result)) = msg.msg {
            Uuid::from_slice(result.channel_id.as_slice()).unwrap()
        } else {
            return;
        }
    } else {
        return;
    };
    let mut buf = BytesMut::new();
    let udp_message = HanUdpMessage {
        connection_id: Vec::from(&connection_id.as_bytes()[..]),
        msg: Some(han_udp_message::Msg::PingPacket(PingPacket {})),
    };
    codec.encode(udp_message, &mut buf).expect("Encoder broken");
    udp_socket.send(&buf[..]).await.expect("Udp Failed");
    let mut buf = BytesMut::with_capacity(8152);
    loop {
        buf.resize(8152, 0);
        match tokio::select! {
            res = udp_socket.recv_from(buf.as_mut()) => res,
            _ = sleep(Duration::from_secs(5)) => return
            } {
            Err(error) => {
                println!("Error with UDP socket: {}", &error);
            }
            Ok((size, _addr)) => {
                buf.resize(size, 0);
                println!("{:?}",rust_hanashite::udphandler::parse_msg(&buf).unwrap());
            }
        };
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
    framed.send(Box::new(HanMessage {
        message_id: Vec::from(&Uuid::new_v4().as_bytes()[..]),
        msg: Some(Msg::Auth(Auth {
            username: "testme".to_string()
        })),
    })).await.expect("Send Failed");
    let connection_id: Uuid = if let Some(Ok(msg)) = framed.next().await {
        println!("Received {:?}", msg);
        if let Some(Msg::AuthResult(result)) = msg.msg {
            Uuid::from_slice(result.connection_id.as_slice()).unwrap()
        } else {
            return;
        }
    } else {
        return;
    };
    framed.send(Box::new(HanMessage {
        message_id: Vec::from(&Uuid::new_v4().as_bytes()[..]),
        msg: Some(Msg::ChanJoin(ChannelJoin {
            name: "testchannel".to_string(),
        })),
    })).await.expect("Send Failed");
    let channel_id: Uuid = if let Some(Ok(msg)) = framed.next().await {
        println!("Received {:?}", msg);
        if let Some(Msg::ChanJoinResult(result)) = msg.msg {
            Uuid::from_slice(result.channel_id.as_slice()).unwrap()
        } else {
            return;
        }
    } else {
        return;
    };
    let mut buf = BytesMut::new();
    let udp_message = HanUdpMessage {
        connection_id: Vec::from(&connection_id.as_bytes()[..]),
        msg: Some(han_udp_message::Msg::PingPacket(PingPacket {})),
    };
    codec.encode(udp_message, &mut buf).expect("Encoder broken");
    udp_socket.send(&buf[..]).await.expect("Udp Failed");
    println!("Send UDP");
    for i in 1..20 {
        let mut buf = BytesMut::new();
        codec.encode(HanUdpMessage {
            connection_id: Vec::from(&connection_id.as_bytes()[..]),
            msg: Some(han_udp_message::Msg::AudioFrame(AudioPacket {
                channel_id: Vec::from(&channel_id.as_bytes()[..]),
                data: vec![1, 2, 3],
                sequence_id: i,
            })),
        }, &mut buf).expect("Encoder broken");
        udp_socket.send(&buf[..]).await.expect("Udp Failed");
        println!("Send UDP");
    }
    sleep(Duration::from_secs(10)).await;
}