import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "node",

    // Single-threaded for NAPI compatibility (AVA's workerThreads: false)
    maxWorkers: 1,

    include: ["test/**/*.{test,spec}.ts"],
    exclude: ["test/deno.test.ts"],
    testTimeout: 120000, // 2m (matching AVA)
    globals: false,

    benchmark: {
      include: ["benchmark/**/*.bench.ts"],
    },
  },
});
