use crate::core::entity::AudioRef;

pub enum PlayerEffect {
    StartPlayback(Vec<AudioRef>),
    Play,
    Pause,
    Stop,
}
