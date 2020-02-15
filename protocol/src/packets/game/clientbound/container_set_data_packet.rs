
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct ContainerSetDataPacket {
    pub containerId: u8,
    pub id: i16,
    pub value: i16,
}

impl CodablePacket for ContainerSetDataPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
        buf.set_mc_i16(self.id);
        buf.set_mc_i16(self.value);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let containerId = buf.get_mc_u8()?;
        let id = buf.get_mc_i16()?;
        let value = buf.get_mc_i16()?;
        return Ok(ContainerSetDataPacket { containerId, id, value });
    }
}
