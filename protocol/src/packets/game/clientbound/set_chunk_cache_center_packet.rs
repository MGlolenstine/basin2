use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetChunkCacheCenterPacket {
    pub x: i32,
    pub z: i32,
}

impl CodablePacket for SetChunkCacheCenterPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.x);
        buf.set_mc_var_int(self.z);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let x = buf.get_mc_var_int()?;
        let z = buf.get_mc_var_int()?;
        return Ok(SetChunkCacheCenterPacket { x, z });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetChunkCacheCenterPacket { x: 45367, z: -34 })
    }
}
