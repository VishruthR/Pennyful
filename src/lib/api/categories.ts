import type { Category, CategoryDetails, CategoryOverview } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

/*
Invoke is super fast in Tauri so for now we can just call the backend directly

TODO: In the future switch to using a context provider and make categories a singleton
TODO: Figure out if you like having categories be indexed by name or id
*/
const getCategoryDetails = async (): Promise<CategoryDetails> => {
    return (await invoke("get_category_details")) as CategoryDetails;
}

const getCategoryByName = async (name: string): Promise<Category | undefined> => {
    const categories = (await invoke("get_category_details")) as CategoryDetails;
    return categories[name];
}

const getCategoryById = async (id: number): Promise<Category | undefined> => {
    const categories = (await invoke("get_category_details")) as CategoryDetails;
    return Object.values(categories).find(category => category.id === id);
}

/*
Returns every category with its budget and current-month spend, for the
manage-categories table. Amounts are raw integer cents.
*/
const getCategoryOverviews = async (): Promise<CategoryOverview[]> => {
    return (await invoke("get_category_overviews")) as CategoryOverview[];
}

/*
Inserts or updates the budget for a category. `amountCents` is a raw integer.
*/
const setCategoryBudget = async (categoryId: number, amountCents: number): Promise<void> => {
    await invoke("set_category_budget", { categoryId, amountCents });
}

interface CategoryInput {
    name: string;
    color: string;
    icon: string | null;
    budgetCents: number | null;
}

/*
Creates a category (optionally with a budget) and returns its new id.
*/
const createCategory = async (input: CategoryInput): Promise<number> => {
    return (await invoke("create_category", { ...input })) as number;
}

/*
Updates a category's name/color/icon and, when provided, its budget.
*/
const updateCategory = async (id: number, input: CategoryInput): Promise<void> => {
    await invoke("update_category", { id, ...input });
}

/*
Deletes a category. Its transactions are reassigned to "Uncategorized".
*/
const deleteCategory = async (id: number): Promise<void> => {
    await invoke("delete_category", { id });
}

export const categoriesApi = {
    getCategoryDetails,
    getCategoryByName,
    getCategoryById,
    getCategoryOverviews,
    setCategoryBudget,
    createCategory,
    updateCategory,
    deleteCategory,
}

export type { CategoryInput };
