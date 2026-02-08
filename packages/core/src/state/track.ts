import type { Audio } from '@type/audio';
import type { InvalidMillisecondsError } from '@type/units/time';
import { Volume } from '@type/units/volume';
import type { Brand } from '@type/utils/brand';
import { err, ok, type Result } from '@type/utils/result';

export type TrackId = Brand<string, 'TrackId'>;

export class InvalidTrackIdError extends Error {
    constructor() {
        super('InvalidTrackIdError');
    }
}

export const TrackId = {
    create(value: string): Result<TrackId, InvalidTrackIdError> {
        if (value.length === 0) {
            return err(new InvalidTrackIdError());
        }
        return ok(value as TrackId);
    },
};

export type Track = {
    readonly id: TrackId;
    readonly audio: Audio;
    readonly volume: Volume;
};

export type CreateTrackInput = {
    id: string;
    audio: Audio;
    volume: Volume;
};

export function createTrack({
    id,
    audio,
    volume,
}: CreateTrackInput): Result<
    Track,
    InvalidTrackIdError | InvalidMillisecondsError
> {
    const trackIdResult = TrackId.create(id);
    if (trackIdResult.ok === false) {
        return trackIdResult;
    }

    const volumeResult = Volume.create(volume);
    if (volumeResult.ok === false) {
        return volumeResult;
    }

    return ok({
        id: trackIdResult.value,
        audio,
        volume: volumeResult.value,
    });
}
