/**
 * 
 * @returns a text string of all stylesheet found in the page 
 * 
 */
export function getStyles() {
  let text = [];
  const stylesheets = document.styleSheets;
  for (let sheet of stylesheets) {
    const rules = sheet.cssRules;
    for (let rule of rules) {
      text.push(rule.cssText);
    }
  }
  return text.join("\n");
}
