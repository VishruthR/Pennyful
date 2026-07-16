<!-- @component
  Add/Edit category modal. In "add" mode it creates a new category; in "edit" mode
  it edits the given category (fields are prefilled). Reuses the standard Modal shell
  plus the TextInput, IconPicker and ColorPicker form fields.

  On a successful save it closes itself and calls `onSuccess` so the parent can refresh.
-->

<script lang="ts">
  import Modal from "$lib/components/Modal.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import IconPicker from "$lib/components/IconPicker.svelte";
  import ColorPicker from "$lib/components/ColorPicker.svelte";
  import Button from "$lib/components/Button.svelte";
  import { categoriesApi } from "$lib/api/categories";
  import { parseDollarsToCents } from "$lib/utils/format";
  import type { CategoryOverview } from "$lib/types";

  interface Props {
    open?: boolean;
    mode: "add" | "edit";
    category?: CategoryOverview;
    onSuccess: () => void;
  }

  let { open = $bindable(false), mode, category, onSuccess }: Props = $props();

  let name = $state("");
  let budgetText = $state("");
  let iconName = $state<string | null>(null);
  let hex = $state<string | null>(null);
  let submitting = $state(false);
  let error = $state<string | null>(null);

  let wasOpen = false;
  $effect(() => {
    if (open && !wasOpen) {
      if (mode === "edit" && category) {
        name = category.name;
        budgetText =
          category.budget_cents !== null ? String(Math.round(category.budget_cents / 100)) : "";
        iconName = category.icon ?? null;
        hex = category.color;
      } else {
        name = "";
        budgetText = "";
        iconName = null;
        hex = null;
      }
      error = null;
    }
    wasOpen = open;
  });

  const title = $derived(mode === "edit" ? "Edit Category" : "Add Category");
  const submitLabel = $derived(mode === "edit" ? "Save" : "Add Category");

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    const trimmedName = name.trim();
    if (!trimmedName) {
      error = "Please enter a name.";
      return;
    }
    if (!hex) {
      error = "Please choose a color.";
      return;
    }

    const budgetCents = parseDollarsToCents(budgetText);
    const input = { name: trimmedName, color: hex, icon: iconName, budgetCents };

    submitting = true;
    error = null;
    try {
      if (mode === "edit" && category) {
        await categoriesApi.updateCategory(category.id, input);
      } else {
        await categoriesApi.createCategory(input);
      }
      open = false;
      onSuccess();
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }
</script>

<Modal bind:open {title}>
  <form class="form" onsubmit={handleSubmit}>
    <div class="field">
      <label class="paragraph-bold" for="category-name">Name</label>
      <TextInput id="category-name" bind:value={name} placeholder="Category name" />
    </div>

    <div class="field">
      <label class="paragraph-bold" for="category-budget">Budget</label>
      <TextInput
        id="category-budget"
        bind:value={budgetText}
        inputmode="decimal"
        placeholder="$0.00"
      />
    </div>

    <div class="pickers">
      <div class="field">
        <span class="paragraph-bold">Icon</span>
        <IconPicker bind:iconName />
      </div>
      <div class="field">
        <span class="paragraph-bold">Color</span>
        <ColorPicker bind:hex />
      </div>
    </div>

    {#if error}
      <p class="error paragraph">{error}</p>
    {/if}

    <div class="actions">
      <Button type="submit" disabled={submitting}>{submitLabel}</Button>
    </div>
  </form>
</Modal>

<style>
  .form {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .pickers {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .error {
    margin: 0;
    color: var(--loss-red);
  }

  .actions {
    display: flex;
    justify-content: center;
    margin-top: 4px;
  }
</style>
