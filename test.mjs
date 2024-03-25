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
      } = instance?.exports || {};

      const isz = 4;
      const icap = sum_in_resize8f(isz);
      console.info({ icap });
      const iptr = sum_in_ptr8f();
      console.info({ iptr });
      const iview = new Float64Array(memory?.buffer, iptr, isz);
      iview[0] = 3776.0;
      iview[1] = 634.0;
      iview[2] = 599.0;
      iview[3] = 333.0;
      return {
        left: sum_std8f(),
        right: iview.reduce((state, next) => state + next, 0.0),
      };
    })
    .then(console.info)
    .catch(console.warn);
})();
