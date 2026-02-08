import {
    createPlayerState,
    type PlayerState,
} from 'src/domain/player/player-state';

export type IPlayerRepository = {
    get(): PlayerState;
    save(state: PlayerState): void;
};

export class PlayerRepository implements IPlayerRepository {
    private static _instance = new PlayerRepository(createPlayerState([]));

    private constructor(private _state: PlayerState) {}

    static getInstance(): PlayerRepository {
        return PlayerRepository._instance;
    }

    get(): PlayerState {
        return this._state;
    }

    save(state: PlayerState): void {
        this._state = state;
    }
}
