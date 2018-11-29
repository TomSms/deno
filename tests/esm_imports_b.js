export { returnsC } from "./subdir/esm_imports_c.js";

console.log("Hello from esm_imports_b.js");

export function returnsB() {
  return "B";
}
