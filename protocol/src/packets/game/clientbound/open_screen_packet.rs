use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct OpenScreenPacket {
    pub containerId: i32,
    pub screenType: i32,
    pub title: ChatComponent,
}

impl CodablePacket for OpenScreenPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.containerId);
        buf.set_mc_var_int(self.screenType);
        buf.set_mc_chat_component(self.title);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let containerId = buf.get_mc_var_int()?;
        let screenType = buf.get_mc_var_int()?;
        let title = buf.get_mc_chat_component()?;
        return Ok(OpenScreenPacket {
            containerId,
            screenType,
            title,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(OpenScreenPacket {
            containerId: 56433,
            screenType: 345,
            title: ChatComponent::from("test title".to_string()),
        })
    }
}
