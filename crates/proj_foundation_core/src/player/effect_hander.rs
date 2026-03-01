use super::PlayerCommand;
use super::PlayerEffect;
use super::PlayerEngine;
use super::PlayerError;

pub struct PlayerEffectHandler<E: PlayerEngine> {
    engine: E,
}

impl<E: PlayerEngine> PlayerEffectHandler<E> {
    pub fn new(engine: E) -> Self {
        PlayerEffectHandler { engine }
    }

    pub fn handle<F>(&self, effects: Vec<PlayerEffect>, mut dispatch: F)
    where
        F: FnMut(PlayerCommand),
    {
        for effect in effects {
            match effect {
                PlayerEffect::StartPlayback(list) => {
                    if let Err(_) = self.engine.start(&list) {
                        dispatch(PlayerCommand::EngineFailed(PlayerError::StartFailed));
                    }
                }
                PlayerEffect::Play => {
                    if let Err(_) = self.engine.play() {
                        dispatch(PlayerCommand::EngineFailed(PlayerError::PlayFailed));
                    }
                }
                PlayerEffect::Pause => {
                    if let Err(_) = self.engine.pause() {
                        dispatch(PlayerCommand::EngineFailed(PlayerError::PauseFailed));
                    }
                }
                PlayerEffect::Stop => {
                    if let Err(_) = self.engine.stop() {
                        dispatch(PlayerCommand::EngineFailed(PlayerError::StopFailed));
                    }
                }
            }
        }
    }
}
