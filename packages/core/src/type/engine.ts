import type { PlayerState } from '@state/player';

export type PlayerEngineAdapter = {
    sync: (state: PlayerState) => Promise<void>;
};
