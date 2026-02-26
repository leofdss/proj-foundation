use super::AudioTrack;
use super::PlayerCommand;
use super::PlayerEffect;
use super::PlayerState;
use super::PlayerStatus;
use crate::core::units::Milliseconds;
use crate::core::units::Volume;

fn no_transition(state: &PlayerState) -> (PlayerState, Vec<PlayerEffect>) {
    (state.clone(), vec![])
}

fn transition(
    state: &PlayerState,
    new_status: PlayerStatus,
    effects: Vec<PlayerEffect>,
) -> (PlayerState, Vec<PlayerEffect>) {
    (
        PlayerState {
            status: new_status,
            ..state.clone()
        },
        effects,
    )
}

pub fn player_use_case(
    state: &PlayerState,
    command: PlayerCommand,
) -> (PlayerState, Vec<PlayerEffect>) {
    match (&state.status, command) {
        //
        // LOAD PLAYLIST
        //
        (_, PlayerCommand::LoadPlaylist(list)) => {
            let playback = list.clone();

            (
                PlayerState {
                    status: PlayerStatus::Stopped,
                    tracks: list
                        .into_iter()
                        .map(|audio| AudioTrack {
                            audio,
                            volume: Volume(1.0),
                        })
                        .collect(),
                },
                vec![PlayerEffect::StartPlayback(playback)],
            )
        }

        //
        // STOPPED
        //
        (PlayerStatus::Stopped, PlayerCommand::Play) => transition(
            state,
            PlayerStatus::Playing {
                position: Milliseconds(0),
            },
            vec![PlayerEffect::Play],
        ),

        (
            PlayerStatus::Stopped,
            PlayerCommand::Pause | PlayerCommand::Stop | PlayerCommand::UpdatePosition(_),
        ) => no_transition(state),

        //
        // PLAYING
        //
        (PlayerStatus::Playing { position }, PlayerCommand::Pause) => transition(
            state,
            PlayerStatus::Paused {
                position: position.clone(),
            },
            vec![PlayerEffect::Pause],
        ),

        (PlayerStatus::Playing { .. }, PlayerCommand::Stop) => {
            transition(state, PlayerStatus::Stopped, vec![PlayerEffect::Stop])
        }

        (PlayerStatus::Playing { .. }, PlayerCommand::Play) => no_transition(state),

        (PlayerStatus::Playing { .. }, PlayerCommand::UpdatePosition(pos)) => {
            transition(state, PlayerStatus::Playing { position: pos }, vec![])
        }

        //
        // PAUSED
        //
        (PlayerStatus::Paused { position }, PlayerCommand::Play) => transition(
            state,
            PlayerStatus::Playing {
                position: position.clone(),
            },
            vec![PlayerEffect::Play],
        ),

        (PlayerStatus::Paused { .. }, PlayerCommand::Stop) => {
            transition(state, PlayerStatus::Stopped, vec![PlayerEffect::Stop])
        }

        (PlayerStatus::Paused { .. }, PlayerCommand::Pause) => no_transition(state),

        (PlayerStatus::Paused { .. }, PlayerCommand::UpdatePosition(pos)) => {
            transition(state, PlayerStatus::Paused { position: pos }, vec![])
        }
    }
}
