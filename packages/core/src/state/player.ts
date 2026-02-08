import type { Milliseconds } from '@type/units/time';
import type { Track, TrackId } from './track';
import type { State } from '@type/utils/state';

type PlayerStateBase = {
    readonly tracks: Readonly<Record<TrackId, Track>>;
    readonly position: Milliseconds;
};

export enum PlayerStateType {
    Stopped = 'Stopped',
    Playing = 'Playing',
    Paused = 'Paused',
}

export type StoppedState = State<PlayerStateBase, PlayerStateType.Stopped>;
export type PlayingState = State<PlayerStateBase, PlayerStateType.Playing>;
export type PausedState = State<PlayerStateBase, PlayerStateType.Paused>;

export type PlayerState = StoppedState | PlayingState | PausedState;

export function createPlayerState(tracks: Track[]): StoppedState {
    const trackStates: Record<TrackId, Track> = {};
    for (const track of tracks) {
        trackStates[track.id] = track;
    }
    return {
        tracks: trackStates,
        position: 0 as Milliseconds,
        state: PlayerStateType.Stopped,
    };
}
