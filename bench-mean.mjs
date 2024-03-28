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

        cntif3u_ge,
        cntif3u_ge_simd_chunk16,

        mean_arithmetic_in_ptr5f,
        mean_arithmetic_in_resize5f,
        mean_arithmetic5f_std_fast,
        mean_arithmetic5f_std_high_precision,
        mean_arithmetic5f_simd_fast,
      } = instance?.exports || {};

      const isz = 16777216;

      const icap = mean_arithmetic_in_resize5f(isz);
      const iview = new Float32Array(
        memory?.buffer,
        mean_arithmetic_in_ptr5f(),
        isz,
      );
      for (let i = 0; i < isz; i++) {
        iview[i] = i + 0.5;
      }

      const lpcnt = 16;

      const funcs = {
        mean_wasm_fast: mean_arithmetic5f_std_fast,
        mean_wasm_high: mean_arithmetic5f_std_high_precision,
        mean_simd_fast: mean_arithmetic5f_simd_fast,
        mean_node: (_) =>
          iview.reduce(
            (state, next) => {
              return state + next;
            },
            0.0,
          ) / iview.length,
      };

      const emulate_memcopy = false;

      return Object.keys(funcs).map((key) => {
        if ("mean_node" !== key && emulate_memcopy) { // wasm: mem copy may be required
          for (let i = 0; i < isz; i++) {
            iview[i] = i + 0.5;
          }
        }
        const f = funcs[key];
        const started = Date.now();
        let rslt = 0;
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
