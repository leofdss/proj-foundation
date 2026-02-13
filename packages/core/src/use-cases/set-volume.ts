import { updateVolume } from '@domain/player';
import {
    PlayerStateType,
    type PausedState,
    type PlayingState,
} from '@state/player';
import type { TrackId } from '@state/track';
import type { PlayerEngineAdapter } from '@type/engine';
import type { Volume } from '@type/units/volume';

export async function setVolume(
    engine: PlayerEngineAdapter,
    state: PausedState,
    update: {
        trackId: TrackId;
        volume: Volume;
    }
): Promise<PausedState>;
export async function setVolume(
    engine: PlayerEngineAdapter,
    state: PlayingState,
    update: {
        trackId: TrackId;
        volume: Volume;
    }
): Promise<PlayingState>;
export async function setVolume(
    engine: PlayerEngineAdapter,
    state: PausedState | PlayingState,
    update: {
        trackId: TrackId;
        volume: Volume;
    }
): Promise<PausedState | PlayingState> {
    switch (state.kind) {
        case PlayerStateType.Playing: {
            const newState = updateVolume(state, update);
            await engine.sync(newState);
            return newState;
        }
        case PlayerStateType.Paused: {
            const newState = updateVolume(state, update);
            await engine.sync(newState);
            return newState;
        }
    }
}
