/** Parses a numeric string into a number.
 *
 * Supports strings of the form 4,000.00
 * TODO: Support different locales (e.g. ones that use commas to mark the decimal)
 */
export const parseNumericString = (numericString: string): number => {
  return parseFloat(numericString.replaceAll(",", ""));
};
