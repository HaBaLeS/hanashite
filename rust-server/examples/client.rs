extern crate rust_hanashite;
use rust_hanashite::protos::hanmessage::{Auth, HanMessage, StreamHeader};
use rust_hanashite::protos::hanmessage::mod_HanMessage::OneOfmsg;
use uuid::Uuid;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use quick_protobuf::{MessageWrite, BytesWriter, Writer};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9876").await.unwrap();
    for _ in 1..100 {
        let msg = HanMessage {
            uuid: Vec::from(&Uuid::new_v4().as_bytes()[..]),
            msg: OneOfmsg::auth(Auth {
                username: "testme".to_string()
            }),
        };
        let size = msg.get_size();
        let header = StreamHeader {
            magic: 0x00008A71,
            length: size as u32,
        };
        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(size + header.get_size(), 0);
        let mut writer = Writer::new(BytesWriter::new(buffer.as_mut()));
        header.write_message(&mut writer).expect("Writing header failed");
        msg.write_message(&mut writer).expect("Write message failed");
        stream.write(&buffer[..]).await.expect("network failed");
    }
}