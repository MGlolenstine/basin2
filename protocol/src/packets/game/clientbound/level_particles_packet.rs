
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct LevelParticlesPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub xDist: f32,
    pub yDist: f32,
    pub zDist: f32,
    pub maxSpeed: f32,
    pub count: i32,
    pub overrideLimiter: bool,
    pub particle: ParticleOptions,
}

impl CodablePacket for LevelParticlesPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i32(self.particle.id() as i32);
        buf.set_mc_bool(self.overrideLimiter);
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
        buf.set_mc_f32(self.xDist);
        buf.set_mc_f32(self.yDist);
        buf.set_mc_f32(self.zDist);
        buf.set_mc_f32(self.maxSpeed);
        buf.set_mc_i32(self.count);
        self.particle.serialize(buf);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let particle_id: Particle = buf.get_mc_enum_i32()?;
        let overrideLimiter = buf.get_mc_bool()?;
        let x = buf.get_mc_f64()?;
        let y = buf.get_mc_f64()?;
        let z = buf.get_mc_f64()?;
        let xDist = buf.get_mc_f32()?;
        let yDist = buf.get_mc_f32()?;
        let zDist = buf.get_mc_f32()?;
        let maxSpeed = buf.get_mc_f32()?;
        let count = buf.get_mc_i32()?;
        let particle = ParticleOptions::parse(particle_id, buf)?;
        return Ok(LevelParticlesPacket { x, y, z, xDist, yDist, zDist, maxSpeed, count, overrideLimiter, particle });
    }
}
