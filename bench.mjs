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
        sum_in_resize4f,

        sum_in_ptr8f,
        sum_in_ptr4f,

        sum_std8f,
        sum_std4f,

        sum_simd8f,
        sum_simd4f,
      } = instance?.exports || {};

      const isz = 131072;

      //const icap = sum_in_resize8f(isz);
      const icap = sum_in_resize4f(isz);
      //const iview = new Float64Array(memory?.buffer, sum_in_ptr8f(), isz);
      const iview = new Float32Array(memory?.buffer, sum_in_ptr4f(), isz);

      const lpcnt = 16384;

      const funcs = {
        sum_wasm: sum_std4f,
        sum_simd: sum_simd4f,
        sum_node: () => iview.reduce((state, next) => state + next, 0.0),
      };

      return Object.keys(funcs).map((key) => {
        if ("sum_node" !== key) { // wasm: mem copy may be required
          for (let i = 0; i < isz; i++) {
            iview[i] = i + 0.5;
          }
        }
        const f = funcs[key];
        const started = Date.now();
        let rslt = 0.0;
        for (let i = 0; i < lpcnt; i++) {
          rslt = f();
        }
        const elapsed = Date.now() - started;
        return { name: key, elapsed, rslt };
      });
    })
    .then(console.info)
    .catch(console.warn);
})();
