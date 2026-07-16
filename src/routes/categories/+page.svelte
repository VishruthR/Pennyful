<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "@iconify/svelte";
  import Button from "$lib/components/Button.svelte";
  import CategoryTable from "$lib/components/CategoryTable.svelte";
  import CategoryFormModal from "$lib/components/CategoryFormModal.svelte";
  import DeleteCategoryModal from "$lib/components/DeleteCategoryModal.svelte";
  import { categoriesApi } from "$lib/api/categories";
  import type { CategoryOverview } from "$lib/types";

  let categories = $state<CategoryOverview[]>([]);

  let formOpen = $state(false);
  let formMode = $state<"add" | "edit">("add");
  let activeCategory = $state<CategoryOverview | undefined>(undefined);

  let deleteOpen = $state(false);
  let categoryToDelete = $state<CategoryOverview | undefined>(undefined);

  async function loadCategories() {
    categories = await categoriesApi.getCategoryOverviews();
  }

  onMount(loadCategories);

  async function handleBudgetChange(categoryId: number, amountCents: number) {
    await categoriesApi.setCategoryBudget(categoryId, amountCents);
    categories = categories.map((category) =>
      category.id === categoryId ? { ...category, budget_cents: amountCents } : category
    );
  }

  function handleAdd() {
    formMode = "add";
    activeCategory = undefined;
    formOpen = true;
  }

  function handleEdit(category: CategoryOverview) {
    formMode = "edit";
    activeCategory = category;
    formOpen = true;
  }

  function handleDelete(category: CategoryOverview) {
    categoryToDelete = category;
    deleteOpen = true;
  }
</script>

<main class="page">
  <header class="page-header">
    <div class="page-heading">
      <h2 class="h2">Manage Categories</h2>
      <p class="paragraph">
        Understand your spending by category. You can also add, edit, or delete any
        existing categories.
      </p>
    </div>
    <Button onclick={handleAdd}>
      <Icon icon="mdi:plus" width={20} height={20} />
      Add Category
    </Button>
  </header>

  <CategoryTable
    {categories}
    onBudgetChange={handleBudgetChange}
    onEdit={handleEdit}
    onDelete={handleDelete}
  />
</main>

<CategoryFormModal
  bind:open={formOpen}
  mode={formMode}
  category={activeCategory}
  onSuccess={loadCategories}
/>

<DeleteCategoryModal
  bind:open={deleteOpen}
  category={categoryToDelete}
  onSuccess={loadCategories}
/>

<style>
  .page {
    padding: 32px;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .page-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 24px;
  }

  .page-heading {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
</style>
