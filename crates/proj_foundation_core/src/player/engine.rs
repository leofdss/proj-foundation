use crate::core::entity::AudioRef;

pub enum PlayerEngineError {
    DeviceLost,
    DecodeFailure,
}

pub trait PlayerEngine {
    fn start(&self, playlist: &[AudioRef]) -> Result<(), PlayerEngineError>;
    fn play(&self) -> Result<(), PlayerEngineError>;
    fn pause(&self) -> Result<(), PlayerEngineError>;
    fn stop(&self) -> Result<(), PlayerEngineError>;
}
