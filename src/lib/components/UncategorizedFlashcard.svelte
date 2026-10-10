<!-- @component
  Displays a transaction as a flashcard with the account, name, date, amount,
  and category selector.

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
  import Icon from "@iconify/svelte";
  import { formatDate, formatSignedCurrencyChange } from "$lib/utils/format";
  import { getInstitutionIcon } from "$lib/utils/institutionLogos";
  import CategoryCombobox from "$lib/components/CategoryCombobox.svelte";
  import type { FullTransaction } from "$lib/types/full_transaction";
  import type { Category } from "$lib/types/category";

  interface Props {
    transaction: FullTransaction;
    categories: Category[];
    handleCategoryUpdate: (transactionId: number, categoryId: number) => void;
  }

  let { transaction, categories, handleCategoryUpdate }: Props = $props();

  const onCategoryUpdate = (categoryId: number) => {
    handleCategoryUpdate(transaction.transaction.id, categoryId);
  };
</script>

<div class="flashcard">
  <div class="account">
    <Icon
      icon={getInstitutionIcon(transaction.bank_institution_id)}
      width={28}
      height={28}
    />
    <span class="account-name">{transaction.account_name}</span>
  </div>

  <hr class="divider" />

  <span class="name">{transaction.transaction.name}</span>

  <div class="row">
    <div class="date">
      <Icon icon="mdi:calendar-blank-outline" width={22} height={22} />
      <span class="date-value">{formatDate(transaction.transaction.date)}</span>
    </div>
    <div class="amount">
      <span class="amount-label">Amount</span>
      <span
        class="amount-value {transaction.transaction.amount >= 0
          ? 'positive'
          : 'negative'}"
      >
        {formatSignedCurrencyChange(transaction.transaction.amount)}
      </span>
    </div>
  </div>

  <div class="field">
    <span class="label category-label">Category</span>
    <div>
      <CategoryCombobox
        {categories}
        value={transaction.transaction.category_id}
        onSelect={onCategoryUpdate}
      />
    </div>
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

  .account {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--grey-500);
  }

  .account-name {
    font-size: 18px;
    font-weight: 500;
    color: var(--grey-500);
  }

  .divider {
    width: 100%;
    height: 0;
    margin: 0;
    border: none;
    border-top: 1px solid var(--grey-200);
  }

  .name {
    font-size: 20px;
    font-weight: 700;
    color: var(--grey-500);
    word-wrap: break-word;
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    gap: 40px;
  }

  .date {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--grey-300);
  }

  .date-value {
    font-size: 16px;
    font-weight: 500;
    color: var(--grey-500);
  }

  .amount {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .amount-label {
    font-size: 16px;
    color: var(--grey-300);
  }

  .amount-value {
    font-size: 16px;
  }

  .label {
    font-size: 16px;
    font-weight: 700;
    color: var(--grey-500);
  }

  .category-label {
    display: block;
    margin-bottom: 4px;
  }

  .field {
    display: flex;
    flex-direction: column;
    width: 100%;
    gap: 4px;
  }

  .positive {
    color: var(--profit-green);
  }

  .negative {
    color: var(--loss-red);
  }
</style>
