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
    import type { FullTransactionInfo } from "$lib/types";
    import CategoryPill from "$lib/components/CategoryPill.svelte";
  
    interface Props {
      transaction: FullTransactionInfo;
    }
  
    let { transaction }: Props = $props();
  
    function formatDate(date: Date): string {
      const month = String(date.getMonth() + 1).padStart(2, "0");
      const day = String(date.getDate()).padStart(2, "0");
      const year = date.getFullYear();
      return `${month}/${day}/${year}`;
    }
  
    function formatAmount(amount: number): string {
      const absAmount = Math.abs(amount);
      const formatted = absAmount.toLocaleString("en-US", {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
      });
      return amount >= 0 ? `+$${formatted}` : `-$${formatted}`;
    }
  
    function isPositive(amount: number): boolean {
      return amount >= 0;
    }
  </script>
  
  <div class="flashcard">
    <div class="field">
      <span class="label">Name</span>
      <span class="name-value">{transaction.name}</span>
    </div>
  
    <div class="row">
      <div class="field">
        <span class="label">Date</span>
        <span class="value">{formatDate(transaction.date)}</span>
      </div>
      <div class="field">
        <span class="label">Amount</span>
        <span class="value {isPositive(transaction.amount) ? 'positive' : 'negative'}">
          {formatAmount(transaction.amount)}
        </span>
      </div>
    </div>
  
    <div class="field">
      <span class="label">Category</span>
      <div>
        <CategoryPill
          name={transaction.category.name}
          icon={transaction.category.icon}
          color={transaction.category.color}
          textColor="white"
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
  
    .name-value {
      font-size: 16px;
      font-weight: 400;
      color: var(--grey-300);
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
  