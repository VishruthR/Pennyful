/**
 * Formats a Date object to MM/DD/YYYY string format.
 */
export function formatDate(date: Date): string {
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  const year = date.getFullYear();
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
 * Returns true if the amount is non-negative.
 */
export function isPositiveAmount(amount: number): boolean {
  return amount >= 0;
}
