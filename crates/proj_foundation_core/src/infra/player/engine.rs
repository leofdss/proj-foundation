use crate::common::entity::AudioRef;

pub enum PlayerEngineEnvId {
    Linux,
    Android,
}

pub trait PlayerEngine {
    fn get_env_id(&self) -> PlayerEngineEnvId;
    fn load_playlist(&self, playlist: &[AudioRef]);
    fn start(&self);
    fn stop(&self);
}
