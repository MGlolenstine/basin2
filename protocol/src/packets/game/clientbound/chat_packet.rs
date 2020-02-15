
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct ChatPacket {
    pub message: ChatComponent,
    pub chat_type: ChatType,
}

impl CodablePacket for ChatPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_chat_component(self.message);
        buf.set_mc_u8(self.chat_type as u8);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let message = buf.get_mc_chat_component()?;
        let chat_type: ChatType = buf.get_mc_enum_u8()?;
        return Ok(ChatPacket { message, chat_type });
    }
}
