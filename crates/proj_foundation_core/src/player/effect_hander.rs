use super::PlayerEffect;
use super::PlayerEngine;

pub struct PlayerEffectHandler<E: PlayerEngine> {
    engine: E,
}

impl<E: PlayerEngine> PlayerEffectHandler<E> {
    pub fn new(engine: E) -> Self {
        PlayerEffectHandler { engine }
    }

    pub fn handle(&self, effects: Vec<PlayerEffect>) {
        for effect in effects {
            match effect {
                PlayerEffect::StartPlayback(list) => self.engine.start(&list),
                PlayerEffect::Play => self.engine.play(),
                PlayerEffect::Pause => self.engine.pause(),
                PlayerEffect::Stop => self.engine.stop(),
            }
        }
    }
}
