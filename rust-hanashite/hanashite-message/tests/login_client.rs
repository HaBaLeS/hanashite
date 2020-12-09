#![allow(dead_code)]

use tokio;

#[tokio::main]
async fn main() {

}

#[allow(dead_code)]
mod util {
    use uuid::Uuid;
    use hanashite_message::protos::hanmessage::HanMessage;
    use hanashite_message::protos::to_data_opt;
    use hanashite_message::protos::hanmessage::han_message::Msg;

    pub fn build_message(uuid: &Option<Uuid>, msg: Msg) -> HanMessage {
        HanMessage {
            message_id: to_data_opt(uuid),
            msg: Some(msg),
        }
    }
}