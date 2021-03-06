use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ChangeDifficultyPacket {
    pub difficulty: Difficulty,
    pub locked: bool,
}

impl CodablePacket for ChangeDifficultyPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.difficulty as u8);
        buf.set_mc_bool(self.locked);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let difficulty: Difficulty = buf.get_mc_enum_u8()?;
        let locked = buf.get_mc_bool()?;
        return Ok(ChangeDifficultyPacket { difficulty, locked });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ChangeDifficultyPacket {
            difficulty: Difficulty::Easy,
            locked: true,
        })
    }
}
