use super::AudioRefDto;
use crate::common::units::Volume;

#[derive(Clone, Default)]
pub enum PlayerStatusDto {
    #[default]
    Init,
    Ready,
    Playing,
    Stopped,
}

#[derive(Clone)]
pub struct AudioTrackDto {
    pub audio: AudioRefDto,
    pub volume: Volume,
}

#[derive(Clone, Default)]
pub struct PlayerStateDto {
    pub status: PlayerStatusDto,
    pub tracks: Vec<AudioTrackDto>,
}
