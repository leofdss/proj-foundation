use super::PlayerEngineError;
use crate::common::entity::AudioRef;
use crate::common::units::Milliseconds;

pub enum PlayerCommand {
    LoadPlaylist(Vec<AudioRef>),
    Play,
    Pause,
    Stop,
    UpdatePosition(Milliseconds),

    EngineFailed(PlayerEngineError),
}
