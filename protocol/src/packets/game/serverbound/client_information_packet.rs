use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ClientInformationPacket {
    pub language: String,
    pub viewDistance: u8,
    pub chatVisibility: ChatVisibility,
    pub chatColors: bool,
    pub modelCustomisation: u8,
    pub mainHand: HumanoidArm,
}

impl CodablePacket for ClientInformationPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.language);
        buf.set_mc_u8(self.viewDistance);
        buf.set_mc_var_int(self.chatVisibility as i32);
        buf.set_mc_bool(self.chatColors);
        buf.set_mc_u8(self.modelCustomisation);
        buf.set_mc_var_int(self.mainHand as i32);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let language = buf.get_mc_string(16)?;
        let viewDistance = buf.get_mc_u8()?;
        let chatVisibility: ChatVisibility = buf.get_mc_enum()?;
        let chatColors = buf.get_mc_bool()?;
        let modelCustomisation = buf.get_mc_u8()?;
        let mainHand: HumanoidArm = buf.get_mc_enum()?;
        return Ok(ClientInformationPacket {
            language,
            viewDistance,
            chatVisibility,
            chatColors,
            modelCustomisation,
            mainHand,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ClientInformationPacket {
            language: "a language".to_string(),
            viewDistance: 12,
            chatVisibility: ChatVisibility::Full,
            chatColors: true,
            modelCustomisation: 4,
            mainHand: HumanoidArm::Right,
        })
    }
}
