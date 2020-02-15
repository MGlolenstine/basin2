
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct SetEntityMotionPacket {
    pub id: i32,
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
}

impl CodablePacket for SetEntityMotionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_i16(self.xa);
        buf.set_mc_i16(self.ya);
        buf.set_mc_i16(self.za);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let id = buf.get_mc_var_int()?;
        let xa = buf.get_mc_i16()?;
        let ya = buf.get_mc_i16()?;
        let za = buf.get_mc_i16()?;
        return Ok(SetEntityMotionPacket { id, xa, ya, za });
    }
}
