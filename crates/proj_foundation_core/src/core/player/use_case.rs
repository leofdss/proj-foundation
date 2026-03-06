use super::AudioTrack;
use super::PlayerCommand;
use super::PlayerEffect;
use super::PlayerState;
use super::PlayerStatus;
use crate::common::units::Volume;
use crate::core::player::PlayerEvent;

pub fn player_use_case(
    state: &PlayerState,
    command: PlayerCommand,
) -> (PlayerState, Vec<PlayerEffect>) {
    match (&state.status, command) {
        (_, PlayerCommand::LoadPlaylist(list)) => (
            PlayerState {
                status: PlayerStatus::Init,
                tracks: list
                    .into_iter()
                    .map(|audio| AudioTrack {
                        audio,
                        volume: Volume(1.0),
                    })
                    .collect(),
            },
            vec![PlayerEffect::LoadPlaylist],
        ),

        (PlayerStatus::Ready | PlayerStatus::Stopped, PlayerCommand::Play) => (
            PlayerState {
                status: PlayerStatus::Playing,
                ..state.clone()
            },
            vec![PlayerEffect::StartPlayback],
        ),

        (PlayerStatus::Playing, PlayerCommand::Stop) => (
            PlayerState {
                status: PlayerStatus::Stopped,
                ..state.clone()
            },
            vec![PlayerEffect::StopPlayback],
        ),

        (PlayerStatus::Init, PlayerCommand::EngineEvent(PlayerEvent::Ready)) => (
            PlayerState {
                status: PlayerStatus::Ready,
                ..state.clone()
            },
            vec![],
        ),

        (PlayerStatus::Playing, PlayerCommand::EngineEvent(PlayerEvent::PlaybackFinished)) => (
            PlayerState {
                status: PlayerStatus::Stopped,
                ..state.clone()
            },
            vec![],
        ),

        (_, PlayerCommand::EngineEvent(PlayerEvent::EngineFailed)) => (
            PlayerState {
                status: PlayerStatus::Init,
                ..state.clone()
            },
            vec![],
        ),

        _ => (state.clone(), vec![]),
    }
}
