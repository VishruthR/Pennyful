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
    import type { CategoryDetails, TransactionImport } from "$lib/types";
    import CategoryPill from "$lib/components/CategoryPill.svelte";
    import { formatDate, formatSignedCurrency, isPositiveAmount } from "$lib/utils/format";
    import { categoriesApi } from "$lib/api/categories";
  
    interface Props {
      transaction: TransactionImport;
    }
  
    let { transaction }: Props = $props();

    const category = $derived(await categoriesApi.getCategoryById(transaction.category_id));
  </script>
  
  <div class="flashcard">
    <div class="field">
      <span class="label">Name</span>
      <span class="value wrap">{transaction.name}</span>
    </div>
  
    <div class="row">
      <div class="field">
        <span class="label">Date</span>
        <span class="value">{formatDate(transaction.date)}</span>
      </div>
      <div class="field">
        <span class="label">Amount</span>
        <span class="value {isPositiveAmount(transaction.amount) ? 'positive' : 'negative'}">
          {formatSignedCurrency(transaction.amount)}
        </span>
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
  
    .value {
      font-size: 16px;
      font-weight: 400;
      color: var(--grey-300);
    }
  
    .value.wrap {
      word-wrap: break-word;
    }
  
    .row {
      display: flex;
      justify-content: space-between;
      width: 100%;
      gap: 40px;
    }
  
    .positive {
      color: var(--profit-green);
    }
  
    .negative {
      color: var(--loss-red);
    }
  </style>
