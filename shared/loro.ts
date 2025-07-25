import * as imports from "loro-crdt/bundler/loro_wasm_bg.js";
import * as wasm from "loro-crdt/bundler/loro_wasm_bg.wasm";

if (wasm.__wbindgen_start) {
  imports.__wbg_set_wasm(wasm);
  wasm.__wbindgen_start();
} else {
  const wkmod = await import("loro-crdt/bundler/loro_wasm_bg.wasm");
  const instance = new WebAssembly.Instance(wkmod.default, {
    "./loro_wasm_bg.js": imports,
  });
  imports.__wbg_set_wasm(instance.exports);
}

export * from "loro-crdt/bundler";
