import { categoriesApi } from "$lib/api/categories";
import { transactionsApi } from "$lib/api/transactions";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ depends }) => {
  depends("home:transactions-table");

  const loadMonthlyStats = async () => {
    // We represent months by the 1st of each month. The date carries no semantic meaning in this component.
    const today = new Date();
    const lastTwelveMonths = [...Array(13).keys()].map(
      (offset) => new Date(today.getFullYear(), today.getMonth() - offset, 1),
    );

    const promises = lastTwelveMonths.map((month) => {
      const endOfMonth = new Date(month.getFullYear(), month.getMonth() + 1, 0);
      return transactionsApi.getSpendingAndIncomeByDateRange(
        month.toISOString().slice(0, 10),
        endOfMonth.toISOString().slice(0, 10),
      );
    });
    const spendingAndIncome = await Promise.all(promises);

    const monthlyStats = spendingAndIncome.map(([spending, income], idx) => {
      return {
        date: lastTwelveMonths[idx],
        income: spending,
        spending: income,
      };
    });

    return monthlyStats;
  };

  return {
    categories: await categoriesApi.getFullCategories(["Income"]),
    monthlyStats: await loadMonthlyStats(),
  };
};
