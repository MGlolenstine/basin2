use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct PlayerActionPacket {
    pub pos: BlockPos,
    pub direction: Direction,
    pub action: PlayerActionPacketAction,
}

impl CodablePacket for PlayerActionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.action as i32);
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_u8(self.direction as u8);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let action: PlayerActionPacketAction = buf.get_mc_enum()?;
        let pos = buf.get_mc_block_pos()?;
        let direction: Direction = buf.get_mc_enum_u8()?;
        return Ok(PlayerActionPacket {
            pos,
            direction,
            action,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(PlayerActionPacket {
            action: PlayerActionPacketAction::StartDestroyBlock,
            pos: BlockPos { x: 1, y: 2, z: 3 },
            direction: Direction::North,
        })
    }
}
