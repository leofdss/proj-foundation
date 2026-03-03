use crate::{
    common::entity::AudioRef, core::player::PlayerEngine, core::player::PlayerEngineError,
};

pub struct PlayerEngineLinux {}

impl PlayerEngine for PlayerEngineLinux {
    fn start(&self, _playlist: &[AudioRef]) -> Result<(), PlayerEngineError> {
        todo!()
    }

    fn play(&self) -> Result<(), PlayerEngineError> {
        todo!()
    }

    fn pause(&self) -> Result<(), PlayerEngineError> {
        todo!()
    }

    fn stop(&self) -> Result<(), PlayerEngineError> {
        todo!()
    }
}

impl PlayerEngineLinux {
    pub fn new() -> Self {
        PlayerEngineLinux {}
    }
}
