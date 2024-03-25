import { readFile } from "node:fs/promises";

(() => {
  return Promise.resolve("rs_summary.wasm")
    .then(readFile)
    .then(WebAssembly.instantiate)
    .then((pair) => {
      const {
        module,
        instance,
      } = pair || {};
      const {
        memory,

        sum_in_resize8f,
        sum_in_ptr8f,

        sum_std8f,
        sum_simd8f,
      } = instance?.exports || {};

      const isz = 16777216;

      const icap = sum_in_resize8f(isz);
      const iview = new Float64Array(memory?.buffer, sum_in_ptr8f(), isz);
      for (let i = 0; i < isz; i++) {
        iview[i] = i + 0.5;
      }

      const lpcnt = 16;

      const funcs = {
        sum_wasm: sum_std8f,
        sum_simd: sum_simd8f,
        sum_node: () => iview.reduce((state, next) => state + next, 0.0),
      };

      return Object.keys(funcs).map((key) => {
        const f = funcs[key];
        const started = Date.now();
        let rslt = 0.0;
        for (let i = 0; i < lpcnt; i++) rslt = f();
        const elapsed = Date.now() - started;
        return { name: key, elapsed, rslt };
      });
    })
    .then(console.info)
    .catch(console.warn);
})();
