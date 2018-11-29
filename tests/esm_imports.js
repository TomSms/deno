import { returnsB, returnsC } from "./esm_imports_b.js";

let b = returnsB();
if (b !== "B") {
  throw Error("bad value");
}

let c = returnsC();
if (c !== "C") {
  throw Error("bad value");
}

console.log("ok");
