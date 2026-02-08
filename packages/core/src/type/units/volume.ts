import type { Brand } from '../utils/brand';
import { err, ok, type Result } from '../utils/result';

export type Decibels = Brand<number, 'Decibels'>;
export type Volume = Brand<number, 'Volume'>;

export class InvalidDecibelsError extends Error {
    constructor() {
        super('InvalidDecibelsError');
    }
}

export class InvalidVolumeError extends Error {
    constructor() {
        super('InvalidVolumeError');
    }
}

export const Decibels = {
    create(value: number): Result<Decibels, InvalidDecibelsError> {
        if (!Number.isFinite(value)) {
            return err(new InvalidDecibelsError());
        }
        return ok(value as Decibels);
    },
};

export const Volume = {
    create(value: number): Result<Volume, InvalidVolumeError> {
        if (value < 0 || value > 100) {
            return err(new InvalidVolumeError());
        }
        return ok(value as Volume);
    },
};
