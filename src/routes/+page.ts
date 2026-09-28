import { categoriesApi } from "$lib/api/categories";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ depends }) => {
  depends("home:transactions-table");

  return {
    categories: await categoriesApi.getCategoryOverviews(["Income"]),
  };
};
