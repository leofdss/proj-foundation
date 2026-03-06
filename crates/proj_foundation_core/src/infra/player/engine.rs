use crate::common::entity::AudioRef;

pub trait PlayerEngine {
    fn load_playlist(&self, playlist: &[AudioRef]);
    fn start(&self);
    fn stop(&self);
}
