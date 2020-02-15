
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct ContainerSetSlotPacket {
    pub containerId: u8,
    pub slot: i16,
    pub itemStack: ItemStack,
}

impl CodablePacket for ContainerSetSlotPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
        buf.set_mc_i16(self.slot);
        buf.set_mc_item_stack(self.itemStack);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let containerId = buf.get_mc_u8()?;
        let slot = buf.get_mc_i16()?;
        let itemStack = buf.get_mc_item_stack()?;
        return Ok(ContainerSetSlotPacket { containerId, slot, itemStack });
    }
}
