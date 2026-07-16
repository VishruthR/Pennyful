<!-- @component
  Confirmation modal for deleting a category. 
-->

<script lang="ts">
  import Modal from "$lib/components/Modal.svelte";
  import Button from "$lib/components/Button.svelte";
  import { categoriesApi } from "$lib/api/categories";
  import type { CategoryOverview } from "$lib/types";

  interface Props {
    open?: boolean;
    category?: CategoryOverview;
    onSuccess: () => void;
  }

  let { open = $bindable(false), category, onSuccess }: Props = $props();

  let submitting = $state(false);
  let error = $state<string | null>(null);

  let wasOpen = false;
  $effect(() => {
    if (open && !wasOpen) {
      error = null;
    }
    wasOpen = open;
  });

  async function handleDelete() {
    if (!category) return;
    submitting = true;
    error = null;
    try {
      await categoriesApi.deleteCategory(category.id);
      open = false;
      onSuccess();
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }
</script>

<Modal bind:open title="Delete Category">
  <div class="body">
    <p class="paragraph">
      Deleting <strong>{category?.name}</strong> is permanent and cannot be undone. All
      transactions in this category will be set to <strong>Uncategorized</strong>.
    </p>

    {#if error}
      <p class="error paragraph">{error}</p>
    {/if}

    <div class="actions">
      <Button onclick={() => (open = false)} disabled={submitting}>Cancel</Button>
      <Button color="var(--loss-red)" onclick={handleDelete} disabled={submitting}>Delete</Button>
    </div>
  </div>
</Modal>

<style>
  .body {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .body .paragraph {
    margin: 0;
    color: var(--grey-500);
  }

  .error {
    color: var(--loss-red);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
  }
</style>
