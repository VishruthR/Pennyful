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

/**
 * Formats an integer number of cents as a whole-dollar currency string, e.g. 50000 -> "$500".
 * Rounds to the nearest dollar; cents are not shown.
 */
export function formatCentsAsDollars(cents: number): string {
  const dollars = Math.round(cents / 100);
  const sign = dollars < 0 ? "-" : "";
  return `${sign}$${Math.abs(dollars).toLocaleString("en-US")}`;
}

/**
 * Parses a user-entered dollar string (e.g. "$1,200" or "500") into integer cents.
 * Returns null when the input is empty or not a valid number.
 */
export function parseDollarsToCents(input: string): number | null {
  const cleaned = input.replace(/[$,\s]/g, "");
  if (cleaned === "") return null;
  const dollars = Number(cleaned);
  if (!Number.isFinite(dollars)) return null;
  return Math.round(dollars * 100);
}
