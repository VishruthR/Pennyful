/**
 * Formats a date string of form YYYY-MM-DD
 * to MM/DD/YYYY format
 */
export function formatDate(date: string): string {
  // Simple guard
  if (date.length !== "YYYY-MM-DD".length) {
    return "";
  }

  const dateParts = date.split("-");

  const month = dateParts[1];
  const year = dateParts[0];
  const day = dateParts[2];
  return `${month}/${day}/${year}`;
}

/**
 * Formats a number as currency with sign prefix.
 * Positive amounts show as "+$X.XX", negative as "-$X.XX".
 */
export function formatSignedCurrencyChange(change: number): string {
  const absAmount = Math.abs(change);
  const formatted = absAmount.toLocaleString("en-US", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
  return change >= 0 ? `+$${formatted}` : `-$${formatted}`;
}

/**
 * Formats a number as currency with sign prefix.
 * Positive amounts show as "$X.XX", negative as "-$X.XX".
 */
export function formatSignedCurrencyAmount(amount: number): string {
  const absAmount = Math.abs(amount);
  const formatted = absAmount.toLocaleString("en-US", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
  return amount >= 0 ? `$${formatted}` : `-$${formatted}`;
}

/**
 * Formats an integer `amount` to a dollar currency string, e.g. 500 --> "$500".
 * Rounds to the nearest dollar; cents are not shown.
 */
export function formatDollars(amount: number): string {
  const sign = amount < 0 ? "-" : "";
  return `${sign}$${Math.abs(amount).toLocaleString("en-US")}`;
}
