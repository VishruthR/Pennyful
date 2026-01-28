<!-- @component
  Displays a table of transactions with sorting capabilities.
  Each row shows date, account, name, category badge, and amount.

  Example data:
  const transactions = [
    {
      id: 1,
      name: "COSTCO WHSE #0008 11/1 PURCHASE...",
      amount: -10.99,
      date: new Date("2025-11-01"),
      account: { id: 1, name: "BOFA" },
      category: { id: 1, name: "Food", color: "#FEEE8C", icon: "fluent:food-16-filled" }
    }
  ];
-->

<script lang="ts">
  import type { FullTransactionInfo } from "$lib/types";
  import SortArrows, { type SortDirection } from "$lib/components/SortArrows.svelte";
  import CategoryPill from "$lib/components/CategoryPill.svelte";
  import { formatSignedCurrency, isPositiveAmount } from "$lib/utils/format";

  interface Props {
    transactions: FullTransactionInfo[];
    height?: string;
  }

  let { transactions, height }: Props = $props();

  let sortColumn: string = $state("date");
  let sortDirection: SortDirection = $state("desc");

  function toggleSort(column: string) {
    if (sortColumn === column) {
      // Cycle through: null -> desc -> asc -> null
      if (sortDirection === null) {
        sortDirection = "desc";
      } else if (sortDirection === "desc") {
        sortDirection = "asc";
      } else {
        sortDirection = null;
        sortColumn = "";
      }
    } else {
      sortColumn = column;
      sortDirection = "desc";
    }
    // TODO: Implement actual sorting logic
  }

  // TransactionsTable uses a different date format (e.g., "Nov 1, 2025")
  const dateFormatter = new Intl.DateTimeFormat("en-US", { 
    month: "short", day: "numeric", year: "numeric" 
  });

  function formatTableDate(date: Date): string {
    return dateFormatter.format(date);
  }

  let tableHeight = $derived(
    height ? height : '100%'
  )
</script>

<div class="transactions-table-container" style:height={tableHeight}>
  <table class="transactions-table">
    <thead>
      <tr>
        <th class="col-date">
          <button class="sort-button" onclick={() => toggleSort("date")}>
            Date
            <SortArrows column="date" activeColumn={sortColumn} direction={sortDirection} />
          </button>
        </th>
        <th class="col-account">
          <button class="sort-button" onclick={() => toggleSort("account")}>
            Account
            <SortArrows column="account" activeColumn={sortColumn} direction={sortDirection} />
          </button>
        </th>
        <th class="col-name">
          <button class="sort-button" onclick={() => toggleSort("name")}>
            Name
            <SortArrows column="name" activeColumn={sortColumn} direction={sortDirection} />
          </button>
        </th>
        <th class="col-category">Category</th>
        <th class="col-amount">
          <button class="sort-button" onclick={() => toggleSort("amount")}>
            Amount
            <SortArrows column="amount" activeColumn={sortColumn} direction={sortDirection} />
          </button>
        </th>
      </tr>
    </thead>
    <tbody>
      {#each transactions as transaction, index}
        <tr class={index % 2 === 0 ? "row-white" : "row-grey"}>
          <td class="col-date">{formatTableDate(transaction.date)}</td>
          <td class="col-account">
            <span class="ellipsis">{transaction.account.name}</span>
          </td>
          <td class="col-name">
            <span class="ellipsis">{transaction.name}</span>
          </td>
          <td class="col-category">
            <CategoryPill 
              name={transaction.category.name}
              icon={transaction.category.icon}
              color={transaction.category.color}
              textColor={index % 2 === 0 ? 'var(--pure-white)' : 'var(--grey-50)'}
            />
          </td>
          <td class="col-amount {isPositiveAmount(transaction.amount) ? 'positive' : 'negative'}">
            {formatSignedCurrency(transaction.amount)}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .transactions-table-container {
    width: 100%;
    border: 2px solid var(--grey-500);
    border-radius: 10px;
    overflow: auto;
  }

  .transactions-table {
    width: 100%;
    border-collapse: collapse;
    table-layout: fixed;
  }

  thead {
    position: sticky;
    top: 0;
    z-index: 1;
    background-color: var(--pure-white);
  }

  th {
    text-align: left;
    padding: 24px 20px;
    font-weight: bold;
    color: var(--grey-500); 
    box-shadow: inset 0px 0px #000, 0 1px var(--grey-100);
  }

  th.col-amount {
    text-align: right;
  }

  .sort-button {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: none;
    border: none;
    padding: 0;
    font-family: inherit;
    font-size: inherit;
    font-weight: inherit;
    color: inherit;
    cursor: pointer;
  }

  .sort-button:hover {
    opacity: 0.7;
  }

  tbody tr {
    transition: background-color 0.15s ease;
    cursor: pointer;
  }

  tbody tr:hover {
    background-color: var(--blue-50) !important;
  }

  tbody tr:hover td:first-child {
    box-shadow: inset 4px 0 0 0 var(--grey-500);
  }

  .row-white {
    background-color: var(--pure-white);
  }

  .row-grey {
    background-color: var(--grey-50);
  }

  td {
    padding: 16px 20px;
    color: var(--grey-500);
  }

  td.col-amount {
    text-align: right;
    white-space: nowrap;
  }

  .ellipsis {
    display: block;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .col-date {
    width: 12%;
    min-width: 120px;
  }

  .col-account {
    width: 18%;
  }

  .col-name {
    width: 35%;
  }

  .col-category {
    width: 20%;
  }

  .col-amount {
    width: 15%;
    min-width: 100px;
  }

  .positive {
    color: var(--profit-green);
  }

  .negative {
    color: var(--loss-red);
  }
</style>
