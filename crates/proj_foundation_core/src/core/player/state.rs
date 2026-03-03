use crate::common::entity::AudioRef;
use crate::common::units::Milliseconds;
use crate::common::units::Volume;

#[derive(Clone, Default)]
pub enum PlayerStatus {
    #[default]
    Stopped,
    Playing {
        position: Milliseconds,
    },
    Paused {
        position: Milliseconds,
    },
}

#[derive(Clone)]
pub struct AudioTrack {
    pub audio: AudioRef,
    pub volume: Volume,
}

#[derive(Clone, Default)]
pub struct PlayerState {
    pub status: PlayerStatus,
    pub tracks: Vec<AudioTrack>,
}
