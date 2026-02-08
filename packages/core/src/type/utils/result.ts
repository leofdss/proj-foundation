// Rust like

export type Ok<T> = {
    readonly ok: true;
    readonly value: T;
};

export type Err<T> = {
    readonly ok: false;
    readonly error: T;
};

export type Result<T, E> = Ok<T> | Err<E>;

export const ok = <T>(value: T): Result<T, never> => ({
    ok: true,
    value,
});

export const err = <E>(error: E): Result<never, E> => ({ ok: false, error });
