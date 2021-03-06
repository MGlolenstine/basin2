use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SelectAdvancementsTabPacket {
    pub tab: Option<ResourceLocation>,
}

impl CodablePacket for SelectAdvancementsTabPacket {
    fn encode(self, buf: &mut BytesMut) {
        match self.tab {
            Some(tab) => {
                buf.set_mc_bool(true);
                buf.set_mc_string(tab);
            }
            None => {
                buf.set_mc_bool(false);
            }
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let tab = if buf.get_mc_bool()? {
            Some(buf.get_mc_string(32767)?)
        } else {
            None
        };
        return Ok(SelectAdvancementsTabPacket { tab });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SelectAdvancementsTabPacket {
            tab: Some("a tab".to_string()),
        })
    }
}
