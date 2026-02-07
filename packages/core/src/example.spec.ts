import { describe, it, expect } from 'vitest';
import { sum, SumError } from './example';
import { err, Err, ok } from './shared/result';

describe('sum', () => {
    it('should sum two numbers', () => {
        expect(sum(2, 3)).toEqual(ok(5));
    });
    it('should sum two numbers - Error', () => {
        expect(sum(2, undefined as unknown as number)).toEqual(
            err(new SumError())
        );
    });
});
