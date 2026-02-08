import { InvalidMillisecondsError, Milliseconds } from '../units/time';
import type { Brand } from '../utils/brand';
import { err, ok, type Result } from '../utils/result';

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
    readonly title: string;
    readonly duration: Milliseconds;
};

export type CreateTrackInput = {
    id: string;
    title: string;
    duration: number;
};

export function createTrack({
    id,
    title,
    duration,
}: CreateTrackInput): Result<
    Track,
    InvalidTrackIdError | InvalidMillisecondsError
> {
    const trackIdResult = TrackId.create(id);
    if (!trackIdResult.ok) {
        return trackIdResult;
    }

    const durationResult = Milliseconds.create(duration);
    if (!durationResult.ok) {
        return durationResult;
    }

    return ok({
        id: trackIdResult.value,
        title,
        duration: durationResult.value,
    });
}
