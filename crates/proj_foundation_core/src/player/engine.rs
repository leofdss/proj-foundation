use crate::core::entity::AudioRef;

pub trait PlayerEngine {
    fn start(&self, playlist: &[AudioRef]);
    fn play(&self);
    fn pause(&self);
    fn stop(&self);
}
