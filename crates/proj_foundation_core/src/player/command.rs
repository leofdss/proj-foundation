use crate::core::entity::AudioRef;
use crate::core::units::Milliseconds;

pub enum PlayerCommand {
    LoadPlaylist(Vec<AudioRef>),
    Play,
    Pause,
    Stop,
    UpdatePosition(Milliseconds),
}
