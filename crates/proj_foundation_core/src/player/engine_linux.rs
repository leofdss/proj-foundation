use super::PlayerEngine;
use crate::core::entity::AudioRef;

pub struct PlayerEngineLinux {}

impl PlayerEngine for PlayerEngineLinux {
    fn start(&self, _playlist: &[AudioRef]) {
        todo!()
    }

    fn play(&self) {
        todo!()
    }

    fn pause(&self) {
        todo!()
    }

    fn stop(&self) {
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
