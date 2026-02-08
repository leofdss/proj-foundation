import type { Track } from '@shared/objs/track';
import { ok, type Result } from '@shared/utils/result';
import { play } from 'src/domain/player/player-commands';
import {
    type PlayingState,
    type StoppedState,
} from 'src/domain/player/player-state';
import type { IPlayerEngine } from 'src/port/player/player-engine';

export class Player {
    constructor(private _engine: IPlayerEngine) {}

    async play(state: StoppedState): Promise<Result<PlayingState, Error>> {
        const tracks: Track[] = Object.values(state.trackStates).map(
            (e) => e.track
        );

        const result = await this._engine.play(tracks);

        if (!result.ok) {
            return result;
        }

        return ok(play(state));
    }
}
