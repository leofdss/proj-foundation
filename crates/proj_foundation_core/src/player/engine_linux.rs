use super::PlayerEngine;
use crate::core::entity::AudioRef;

pub struct PlayerEngineLinux {}

impl PlayerEngine for PlayerEngineLinux {
    fn start(&self, _playlist: &[AudioRef]) -> Result<(), super::PlayerEngineError> {
        todo!()
    }

    fn play(&self) -> Result<(), super::PlayerEngineError> {
        todo!()
    }

    fn pause(&self) -> Result<(), super::PlayerEngineError> {
        todo!()
    }

    fn stop(&self) -> Result<(), super::PlayerEngineError> {
        todo!()
    }
}

impl PlayerEngineLinux {
    pub fn new() -> Self {
        PlayerEngineLinux {}
    }

    pub fn run(&self) {
        println!("run");
    }
}
