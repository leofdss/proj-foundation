import type { Brand } from '../utils/brand';
import { err, ok, type Result } from '../utils/result';

export type Hertz = Brand<number, 'Hertz'>;

export class InvalidHertzError extends Error {
    constructor() {
        super('InvalidHertzError');
    }
}

export const Hertz = {
    create(value: number): Result<Hertz, InvalidHertzError> {
        if (!Number.isFinite(value) || value <= 0) {
            return err(new InvalidHertzError());
        }
        return ok(value as Hertz);
    },
};
