use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct AwardStatsPacketItem {
    stat_type: i32,
    stat: i32,
    value: i32,
}

#[derive(PartialEq, Clone, Debug)]
pub struct AwardStatsPacket {
    pub stats: Vec<AwardStatsPacketItem>,
}

impl CodablePacket for AwardStatsPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.stats.len() as i32);
        for item in self.stats {
            buf.set_mc_var_int(item.stat_type);
            buf.set_mc_var_int(item.stat);
            buf.set_mc_var_int(item.value);
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let mut stats: Vec<AwardStatsPacketItem> = vec![];
        let count = buf.get_mc_var_int()?;
        for _ in 0..count {
            stats.push(AwardStatsPacketItem {
                stat_type: buf.get_mc_var_int()?,
                stat: buf.get_mc_var_int()?,
                value: buf.get_mc_var_int()?,
            })
        }
        return Ok(AwardStatsPacket { stats });
    }
}
