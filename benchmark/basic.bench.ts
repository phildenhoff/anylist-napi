import { bench, describe } from "vitest";

import { plus100 } from "../index.js";

function add(a: number) {
  return a + 100;
}

describe("Performance benchmarks", () => {
  bench("Native a + 100", () => {
    plus100(10);
  });

  bench("JavaScript a + 100", () => {
    add(10);
  });
});
