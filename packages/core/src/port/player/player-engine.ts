import type { Track, TrackId } from '@shared/objs/track';
import type { Volume } from '@shared/units/volume';
import type { Result } from '@shared/utils/result';

export type IPlayerEngine = {
    play(track: Track[]): Promise<Result<void, Error>>;
    pause(): Promise<Result<void, Error>>;
    volume(trackId: TrackId, volume: Volume): Promise<Result<void, Error>>;
};
