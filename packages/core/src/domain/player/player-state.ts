import type { Track } from '@shared/objs/track';
import type { Milliseconds } from '@shared/units/time';
import type { Volume } from '@shared/units/volume';
import type { State } from '@shared/utils/state';

export type PlayerState = {
    readonly tracks: readonly Track[];
    readonly position: Milliseconds;
    readonly volume: Volume;
};

export enum PlayerStateType {
    Stopped = 'Stopped',
    Playing = 'Playing',
    Paused = 'Paused',
}

export type StoppedState = State<PlayerState, PlayerStateType.Stopped>;
export type PlayingState = State<PlayerState, PlayerStateType.Playing>;
export type PausedState = State<PlayerState, PlayerStateType.Paused>;

export function createPlayerState(tracks: Track[]): StoppedState {
    return {
        tracks,
        position: 0 as Milliseconds,
        volume: 0 as Volume,
        state: PlayerStateType.Stopped,
    };
}
