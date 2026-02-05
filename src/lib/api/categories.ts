import type { Category, CategoryDetails } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

/*
Invoke is super fast in Tauri so for now we can just call the backend directly

TODO: In the future switch to using a context provider and make categories a singleton
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

export const categoriesApi = {
    getCategoryDetails,
    getCategoryByName,
    getCategoryById,
}