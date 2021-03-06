use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;
use log::*;

#[derive(PartialEq, Clone, Debug)]
pub struct InteractPacket {
    pub entityId: i32,
    pub action: InteractPacketAction,
    pub location: Option<(f32, f32, f32)>,
    pub hand: Option<InteractionHand>,
}

impl CodablePacket for InteractPacket {
    fn encode(self, buf: &mut BytesMut) {
        use InteractPacketAction::*;
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_var_int(self.action as i32);
        match (self.action, self.location, self.hand) {
            (InteractAt, Some((x, y, z)), Some(hand)) => {
                buf.set_mc_f32(x);
                buf.set_mc_f32(y);
                buf.set_mc_f32(z);
                buf.set_mc_var_int(hand as i32);
            }
            (InteractAt, None, _) => {
                error!("invalid interact packet with InteractAt action and no location or hand!");
                panic!();
            }
            (Interact, _, Some(hand)) => {
                buf.set_mc_var_int(hand as i32);
            }
            _ => (),
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        use InteractPacketAction::*;
        let entityId = buf.get_mc_var_int()?;
        let action: InteractPacketAction = buf.get_mc_enum()?;
        let location = match action {
            InteractAt => Some((buf.get_mc_f32()?, buf.get_mc_f32()?, buf.get_mc_f32()?)),
            _ => None,
        };
        let hand = match action {
            InteractAt => Some(buf.get_mc_enum()?),
            Interact => Some(buf.get_mc_enum()?),
            _ => None,
        };
        return Ok(InteractPacket {
            entityId,
            action,
            location,
            hand,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle_interact_at() -> Result<()> {
        cycle(InteractPacket {
            entityId: 12345,
            action: InteractPacketAction::InteractAt,
            location: Some((1.0, 2.0, 3.0)),
            hand: Some(InteractionHand::MainHand),
        })
    }

    #[test]
    fn test_cycle_interact() -> Result<()> {
        cycle(InteractPacket {
            entityId: 12345,
            action: InteractPacketAction::Interact,
            location: None,
            hand: Some(InteractionHand::MainHand),
        })
    }

    #[test]
    fn test_cycle_attack() -> Result<()> {
        cycle(InteractPacket {
            entityId: 12345,
            action: InteractPacketAction::Attack,
            location: None,
            hand: None,
        })
    }
}
