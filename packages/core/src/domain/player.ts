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
        kind: PlayerStateType.Playing,
    };
}

export function pause(state: PlayingState): PausedState {
    return {
        ...state,
        kind: PlayerStateType.Paused,
    };
}

export function play(state: PausedState): PlayingState {
    return {
        ...state,
        kind: PlayerStateType.Playing,
    };
}

export function stop(state: PlayingState | PausedState): StoppedState {
    return {
        ...state,
        kind: PlayerStateType.Stopped,
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
    update: {
        trackId: TrackId;
        volume: Volume;
    }
): PausedState;
export function updateVolume(
    state: PlayingState,
    update: {
        trackId: TrackId;
        volume: Volume;
    }
): PlayingState;
export function updateVolume(
    state: PausedState | PlayingState,
    update: {
        trackId: TrackId;
        volume: Volume;
    }
): PausedState | PlayingState {
    const track = state.tracks[update.trackId];
    if (track) {
        return {
            ...state,
            tracks: {
                ...state.tracks,
                [update.trackId]: {
                    ...track,
                    volume: update.volume,
                },
            },
        };
    }
    return state;
}
