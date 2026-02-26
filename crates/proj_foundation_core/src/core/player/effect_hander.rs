use super::PlayerCommand;
use super::PlayerEffect;
use super::PlayerEngine;
use super::PlayerEngineError;
use super::PlayerState;
use crate::common::entity::AudioRef;

pub struct PlayerEffectHandler<E: PlayerEngine> {
    engine: E,
}

impl<E: PlayerEngine> PlayerEffectHandler<E> {
    pub fn new(engine: E) -> Self {
        PlayerEffectHandler { engine }
    }

    pub fn handle(&self, effects: Vec<PlayerEffect>, state: &PlayerState) -> Vec<PlayerCommand> {
        let mut new_commands = Vec::new();

        for effect in effects {
            match effect {
                PlayerEffect::StartPlayback => {
                    let list: Vec<AudioRef> =
                        state.tracks.clone().into_iter().map(|e| e.audio).collect();
                    if let Err(_) = self.engine.start(&list) {
                        new_commands.push(PlayerCommand::EngineFailed(
                            PlayerEngineError::StartPlaybackError,
                        ));
                    }
                }
                PlayerEffect::Play => {
                    if let Err(_) = self.engine.play() {
                        new_commands
                            .push(PlayerCommand::EngineFailed(PlayerEngineError::PlayError));
                    }
                }
                PlayerEffect::Pause => {
                    if let Err(_) = self.engine.pause() {
                        new_commands
                            .push(PlayerCommand::EngineFailed(PlayerEngineError::PauseError));
                    }
                }
                PlayerEffect::Stop => {
                    if let Err(_) = self.engine.stop() {
                        new_commands
                            .push(PlayerCommand::EngineFailed(PlayerEngineError::StopError));
                    }
                }
            }
        }

        new_commands
    }
}
