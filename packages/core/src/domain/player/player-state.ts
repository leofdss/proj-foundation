import type { Track, TrackId } from '@shared/objs/track';
import type { Milliseconds } from '@shared/units/time';
import type { Volume } from '@shared/units/volume';
import type { State } from '@shared/utils/state';

export type TrackState = {
    readonly track: Track;
    readonly volume: Volume;
};

export type PlayerState = {
    readonly trackStates: Readonly<Record<TrackId, TrackState>>;
    readonly position: Milliseconds;
};

export enum PlayerStateType {
    Stopped = 'Stopped',
    Playing = 'Playing',
    Paused = 'Paused',
}

export type StoppedState = State<PlayerState, PlayerStateType.Stopped>;
export type PlayingState = State<PlayerState, PlayerStateType.Playing>;
export type PausedState = State<PlayerState, PlayerStateType.Paused>;

export function createTrackState(track: Track): TrackState {
    return {
        track,
        volume: 0 as Volume,
    };
}

export function createPlayerState(tracks: Track[]): StoppedState {
    const trackStates: Record<TrackId, TrackState> = {};
    for (const track of tracks) {
        trackStates[track.id] = createTrackState(track);
    }
    return {
        trackStates,
        position: 0 as Milliseconds,
        state: PlayerStateType.Stopped,
    };
}
