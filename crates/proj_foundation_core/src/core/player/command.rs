use super::PlayerEvent;
use crate::common::entity::AudioRef;

pub enum PlayerCommand {
    LoadPlaylist(Vec<AudioRef>),
    Play,
    Stop,
    EngineEvent(PlayerEvent),
}
