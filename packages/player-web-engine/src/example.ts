import { err, ok, type Result } from './shared/result';

class SumError extends Error {}

function sum(a: number, b: number): Result<number, SumError> {
    try {
        const result = a + b;
        if (isNaN(result)) {
            return err(new SumError());
        }
        return ok(result);
    } catch {
        return err(new SumError());
    }
}

export { sum, SumError };
