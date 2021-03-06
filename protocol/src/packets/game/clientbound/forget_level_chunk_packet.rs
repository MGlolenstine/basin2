use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ForgetLevelChunkPacket {
    pub x: i32,
    pub z: i32,
}

impl CodablePacket for ForgetLevelChunkPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i32(self.x);
        buf.set_mc_i32(self.z);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let x = buf.get_mc_i32()?;
        let z = buf.get_mc_i32()?;
        return Ok(ForgetLevelChunkPacket { x, z });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ForgetLevelChunkPacket { x: 128, z: 12 })
    }
}
