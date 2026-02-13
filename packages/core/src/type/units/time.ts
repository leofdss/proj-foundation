import type { Brand } from '../utils/brand';
import { err, ok, type Result } from '../utils/result';

export type Milliseconds = Brand<number, 'Milliseconds'>;

export class InvalidMillisecondsError extends Error {
    constructor() {
        super('InvalidMillisecondsError');
    }
}

export const Milliseconds = {
    create(value: number): Result<Milliseconds, InvalidMillisecondsError> {
        if (!Number.isFinite(value) || value < 0) {
            return err(new InvalidMillisecondsError());
        }
        return ok(value as Milliseconds);
    },
};
