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
  import type { TransactionImport, Category } from "$lib/types";
  import { categoriesApi } from "$lib/api/categories";
  import Input from "$lib/components/Input.svelte";
  import DatePicker from "$lib/components/DatePicker.svelte";
  import Icon from "@iconify/svelte";
  import Combobox from "./Combobox.svelte";

  interface Props {
    transaction: TransactionImport;
  }

  let { transaction = $bindable() }: Props = $props();

  // TODO: Handle errors in case category doesn't exist
  const categories = $derived(await categoriesApi.getCategoryDetails());

  let selectedId = $state(String(transaction.category_id));
  let amountStr = $state(String(transaction.amount));

  $effect(() => {
    const parsed = parseFloat(amountStr);
    if (!isNaN(parsed)) transaction.amount = parsed;
  });
</script>

{#snippet categoryOption(category: Category)}
  <span class="option-wrapper">
    <Icon icon={category.icon} color={category.color} width={20} height={20} />
    <p class="paragraph option-text" style:color={category.color}>{category.name}</p>
  </span> 
{/snippet}

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
    <Combobox 
      options={Object.values(categories).map(category => { 
        return { value: category.id.toString(), data: category }
      })}
      optionRenderer={categoryOption}
      placeholder="Uncategorized"
      bind:value={selectedId}
    />
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

  .option-wrapper {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .option-text {
    margin: 4px;
    font-weight: bold;
  }
</style>
