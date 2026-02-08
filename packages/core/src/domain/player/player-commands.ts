import type { Track } from '@shared/objs/track';
import {
    PlayerStateType,
    type PausedState,
    type PlayingState,
    type StoppedState,
} from './player-state';
import { Milliseconds } from '@shared/units/time';
import type { Volume } from '@shared/units/volume';

export function play(state: StoppedState): PlayingState {
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

export function resume(state: PausedState): PlayingState {
    return {
        ...state,
        state: PlayerStateType.Playing,
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

export function updateVolume(state: PausedState, volume: Volume): PausedState;
export function updateVolume(state: PlayingState, volume: Volume): PlayingState;
export function updateVolume(
    state: PausedState | PlayingState,
    volume: Volume
): PausedState | PlayingState {
    return {
        ...state,
        volume,
    };
}
