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

        max_in_ptr6i,
        max_in_resize6i,

        max6i,
        max6i_simd,
      } = instance?.exports || {};

      const isz = 16777216;

      const icap = max_in_resize6i(isz);
      const iview = new BigInt64Array(
        memory?.buffer,
        max_in_ptr6i(),
        isz,
      );
      for (let i = 0n; i < isz; i++) {
        iview[i] = i;
      }

      const lpcnt = 128;

      const alt = -1n;

      const funcs = {
        max_wasm: max6i,
        max_simd: max6i_simd,
        max_node: (_) =>
          iview.reduce(
            (state, next) => state < next ? next : state,
            alt,
          ),
      };

      const emulate_memcopy = false;

      return Object.keys(funcs).map((key) => {
        if ("max_node" !== key && emulate_memcopy) { // wasm: mem copy may be required
          for (let i = 0n; i < isz; i++) {
            iview[i] = i;
          }
        }
        const f = funcs[key];
        const started = Date.now();
        let rslt = 0;
        for (let i = 0; i < lpcnt; i++) {
          rslt = f(alt);
        }
        const elapsed = Date.now() - started;
        return { name: key, elapsed, rslt };
      });
    })
    .then(console.info)
    .catch(console.warn);
})();
