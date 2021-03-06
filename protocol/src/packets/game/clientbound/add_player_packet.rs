use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;
use uuid::Uuid;

#[derive(PartialEq, Clone, Debug)]
pub struct AddPlayerPacket {
    pub entityId: i32,
    pub playerId: Uuid,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yRot: u8,
    pub xRot: u8,
}

impl CodablePacket for AddPlayerPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_uuid(self.playerId);
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
        buf.set_mc_u8(self.yRot);
        buf.set_mc_u8(self.xRot);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entityId = buf.get_mc_var_int()?;
        let playerId = buf.get_mc_uuid()?;
        let x = buf.get_mc_f64()?;
        let y = buf.get_mc_f64()?;
        let z = buf.get_mc_f64()?;
        let yRot = buf.get_mc_u8()?;
        let xRot = buf.get_mc_u8()?;
        return Ok(AddPlayerPacket {
            entityId,
            playerId,
            x,
            y,
            z,
            yRot,
            xRot,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(AddPlayerPacket {
            entityId: 54321,
            playerId: Uuid::new_v4(),
            x: 123.0,
            y: 64.0,
            z: -157.0,
            xRot: 20,
            yRot: 30,
        })
    }
}
