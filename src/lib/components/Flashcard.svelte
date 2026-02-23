<!-- @component
  Displays a transaction as a flashcard with name, date, amount, and category.

  Example data:
  const transaction = {
    id: 1,
    name: "COSTCO WHSE #0008",
    amount: -190.10,
    date: new Date("2025-11-10"),
    account: { id: 1, name: "BOFA" },
    category: { id: 1, name: "Housing", color: "#A78BFA", icon: "mdi:home" }
  };
-->

<script lang="ts">
    import type { TransactionImport } from "$lib/types";
    import CategoryPill from "$lib/components/CategoryPill.svelte";
    import { categoriesApi } from "$lib/api/categories";
    import Input from "$lib/components/Input.svelte";
    import DatePicker from "$lib/components/DatePicker.svelte";

    interface Props {
      transaction: TransactionImport;
    }

    let { transaction = $bindable() }: Props = $props();

    // TODO: Handle errors in case category doesn't exist
    const category = $derived(await categoriesApi.getCategoryById(transaction.category_id));

    let amountStr = $state(String(transaction.amount));

    $effect(() => {
      const parsed = parseFloat(amountStr);
      if (!isNaN(parsed)) transaction.amount = parsed;
    });
  </script>

  <div class="flashcard">
    <div class="field">
      <span class="label">Name</span>
      <Input bind:value={transaction.name} multiline maxlength={100} />
    </div>

    <div class="row">
      <div class="field">
        <span class="label">Date</span>
        <DatePicker bind:value={transaction.date} />
      </div>
      <div class="field">
        <span class="label">Amount</span>
        <Input bind:value={amountStr} />
      </div>
    </div>

    <div class="field">
      <span class="label">Category</span>
      {#if category}
      <div>
        <CategoryPill
          name={category.name}
          icon={category.icon}
          color={category.color}
          textColor="white"
        />
      </div>
      {/if}
    </div>
  </div>
  
  <style>
    .flashcard {
      width: 100%;
      height: 100%;
      overflow-y: auto;
      background-color: var(--pure-white);
      border: 2px solid var(--grey-300);
      border-radius: 12px;
      padding: 24px;
      display: flex;
      flex-direction: column;
      gap: 20px;
    }
  
    .field {
      display: flex;
      flex-direction: column;
      width: 100%;
      gap: 4px;
    }
  
    .label {
      font-size: 16px;
      font-weight: 700;
      color: var(--grey-500);
    }
  
    .row {
      display: flex;
      justify-content: space-between;
      width: 100%;
      gap: 40px;
    }
  
</style>
