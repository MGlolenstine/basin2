use crate::result::*;
use bytes::BytesMut;

pub trait CodablePacket {
    fn encode(self, buf: &mut BytesMut);
    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized;
}

pub trait PacketContainer {
    fn encode(self, buf: &mut BytesMut);
    fn decode(id: i32, buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized;
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::fmt::Debug;

    pub fn cycle<T: CodablePacket + Clone + Debug + PartialEq>(initial: T) -> Result<()> {
        let mut buf = BytesMut::new();
        initial.clone().encode(&mut buf);
        let decoded = T::decode(&mut buf)?;
        assert_eq!(initial, decoded);
        Ok(())
    }
}