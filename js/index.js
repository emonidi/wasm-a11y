import { getStyles } from "./get-styles";

(async () => {})();

document.addEventListener("DOMContentLoaded", async () => {
  const wasm = await import("../pkg/index");
  const element = document.querySelector("#test");

  const styles = getStyles();
  let string = new String("");
  const result = wasm.check_a11y(element.outerHTML, styles);

  console.log(result);
});

