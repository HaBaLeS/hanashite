#[cfg(test)]
mod tests {
    use crate::protos::hanmessage::{Auth, HanMessage};
    use crate::protos::hanmessage::mod_HanMessage::OneOfmsg;
    use uuid::Uuid;
    use tokio::net::TcpStream;
    use crate::tokio::io::AsyncWriteExt;
    use quick_protobuf::{MessageWrite, BytesWriter, Writer};

    #[test]
    fn testme() {
        test_msg();
    }


    #[tokio::main]
    async fn test_msg() {
        let mut stream = TcpStream::connect("127.0.0.1:9876").await.unwrap();
        for i in 1..100 {
            let mut msg = HanMessage {
                uuid: Vec::from(&Uuid::new_v4().as_bytes()[..]),
                msg: OneOfmsg::auth(Auth {
                    username: "testme".to_string()
                }),
            };
            let size = msg.get_size();
            let mut buffer: Vec<u8> = Vec::new();
            buffer.resize(size + 8, 0);
            let mut writer = Writer::new(BytesWriter::new(buffer.as_mut()));
            writer.write_fixed32(0x00008A71).expect("write magic");
            writer.write_fixed32(size as u32).expect("write size");
            msg.write_message(&mut writer);
            stream.write(&buffer[..]).await;
        }
    }
}