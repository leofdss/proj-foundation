import { defineConfig } from 'vitest/config';

export default defineConfig({
    test: {
        environment: 'node',
        globals: true,
        setupFiles: [],
        coverage: {
            provider: 'v8',
            reporter: ['text', 'html'],
        },
    },
});
