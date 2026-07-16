import type { Category, CategoryDetails, CategoryOverview } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

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

const getCategoryOverviews = async (): Promise<CategoryOverview[]> => {
    return (await invoke("get_category_overviews")) as CategoryOverview[];
}

const setCategoryBudget = async (categoryId: number, amountCents: number): Promise<void> => {
    await invoke("set_category_budget", { categoryId, amountCents });
}

interface CategoryInput {
    name: string;
    color: string;
    icon: string | null;
    budgetCents: number | null;
}

const createCategory = async (input: CategoryInput): Promise<number> => {
    return (await invoke("create_category", { ...input })) as number;
}

const updateCategory = async (id: number, input: CategoryInput): Promise<void> => {
    await invoke("update_category", { id, ...input });
}

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
