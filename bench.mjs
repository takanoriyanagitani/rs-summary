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

        cnt_in_resize3u,
        cnt_in_ptr3u,
        cntif3u_ge,
        cntif3u_ge_simd_chunk16,
      } = instance?.exports || {};

      const isz = 16777216;

      const icap = cnt_in_resize3u(isz);
      const iview = new Uint8Array(memory?.buffer, cnt_in_ptr3u(), isz);
      for (let i = 0; i < isz; i++) {
        iview[i] = i;
      }

      const lpcnt = 128;

      const funcs = {
        cnt_wasm: cntif3u_ge,
        cnt_simd: cntif3u_ge_simd_chunk16,
        cnt_node: (lbi = 100) =>
          iview.reduce(
            (state, next) => {
              const isGe = lbi <= next;
              const add = isGe ? 1 : 0;
              return state + add;
            },
            0,
          ),
      };

      const age = 100;

      return Object.keys(funcs).map((key) => {
        if ("cnt_node" !== key) { // wasm: mem copy may be required
          for (let i = 0; i < isz; i++) {
            iview[i] = i;
          }
        }
        const f = funcs[key];
        const started = Date.now();
        let rslt = 0;
        for (let i = 0; i < lpcnt; i++) {
          rslt = f(age);
        }
        const elapsed = Date.now() - started;
        return { name: key, elapsed, rslt };
      });
    })
    .then(console.info)
    .catch(console.warn);
})();
