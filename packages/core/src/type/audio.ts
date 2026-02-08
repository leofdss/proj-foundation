import type { Milliseconds } from './units/time';
import type { Brand } from './utils/brand';
import { err, ok, type Result } from './utils/result';

export type AudioId = Brand<string, 'AudioId'>;

export class InvalidAudioIdError extends Error {
    constructor() {
        super('InvalidAudioIdError');
    }
}

export const AudioId = {
    create(value: string): Result<AudioId, InvalidAudioIdError> {
        if (value.length === 0) {
            return err(new InvalidAudioIdError());
        }
        return ok(value as AudioId);
    },
};

export class Audio {
    constructor(
        public readonly id: AudioId,
        public readonly title: string,
        public readonly duration: Milliseconds
    ) {}
}
