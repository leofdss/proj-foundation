import {
    PlayerStateType,
    type PausedState,
    type PlayingState,
    type StoppedState,
} from '@state/player';
import type { TrackId } from '@state/track';
import type { Milliseconds } from '@type/units/time';
import type { Volume } from '@type/units/volume';

export function start(state: StoppedState): PlayingState {
    return {
        ...state,
        state: PlayerStateType.Playing,
    };
}

export function pause(state: PlayingState): PausedState {
    return {
        ...state,
        state: PlayerStateType.Paused,
    };
}

export function play(state: PausedState): PlayingState {
    return {
        ...state,
        state: PlayerStateType.Playing,
    };
}

export function stop(state: PlayingState | PausedState): StoppedState {
    return {
        ...state,
        state: PlayerStateType.Stopped,
    };
}

export function updatedPosition(
    state: PausedState,
    volume: Milliseconds
): PausedState;
export function updatedPosition(
    state: PlayingState,
    volume: Milliseconds
): PlayingState;
export function updatedPosition(
    state: PausedState | PlayingState,
    position: Milliseconds
): PausedState | PlayingState {
    return {
        ...state,
        position,
    };
}

export function updateVolume(
    state: PausedState,
    trackId: TrackId,
    volume: Volume
): PausedState;
export function updateVolume(
    state: PlayingState,
    trackId: TrackId,
    volume: Volume
): PlayingState;
export function updateVolume(
    state: PausedState | PlayingState,
    trackId: TrackId,
    volume: Volume
): PausedState | PlayingState {
    const track = state.tracks[trackId];
    if (track) {
        return {
            ...state,
            tracks: {
                ...state.tracks,
                [trackId]: {
                    ...track,
                    volume,
                },
            },
        };
    }
    return state;
}
