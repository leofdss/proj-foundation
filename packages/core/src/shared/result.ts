// Rust like

export class Ok<T> {
    public readonly ok = true;
    constructor(public readonly value: T) {}
}

export class Err<T> {
    public readonly ok = false;
    constructor(public readonly error: T) {}
}

export type Result<T, E> = Ok<T> | Err<E>;

export const ok = <T>(value: T): Result<T, never> => new Ok(value);

export const err = <E>(error: E): Result<never, E> => new Err(error);
