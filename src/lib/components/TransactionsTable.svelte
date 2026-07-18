<!-- @component
  Displays a table of transactions with sorting capabilities.
  Each row shows date, account, name, category badge, and amount.

  Fetches data itself
-->

<script lang="ts">
  import SortArrows, { type SortDirection } from "$lib/components/SortArrows.svelte";
  import CategoryPill from "$lib/components/CategoryPill.svelte";
  import { formatSignedCurrencyChange, isPositiveAmount } from "$lib/utils/format";
  import { transactionsApi } from "$lib/api/transactions";
  import type { TransactionWithAccount, PaginedSortedTransactionsResponse } from "$lib/types";
  import Icon from "@iconify/svelte";

  interface Props {
    height?: string;
  }

  let { height }: Props = $props();

  let sortColumn: string = $state("date");
  let sortDirection: SortDirection = $state("Desc");
  let page: number = $state(1);
  let pageSize: number = $state(25);
  let paginatedResponse: PaginedSortedTransactionsResponse | null = $state.raw(null); 
  
  let transactions: TransactionWithAccount[] = $derived.by(() => {
    if (paginatedResponse !== null) {
      return paginatedResponse.transactions
    } else {
      return [];
    }
  })

  const DEFAULT_SORT_COLUMN = "date";
  const DEFAULT_SORT_DIRECTION: SortDirection = "Desc";

  function toggleSort(column: string) {
    if (sortColumn === column) {
      // Cycle through: Desc -> Asc -> default (newest first)
      if (sortDirection === "Desc") {
        sortDirection = "Asc";
      } else {
        sortColumn = DEFAULT_SORT_COLUMN;
        sortDirection = DEFAULT_SORT_DIRECTION;
      }
    } else {
      sortColumn = column;
      sortDirection = "Desc";
    }
    page = 1;
  }

  const handleNextPage = () => {
    if (paginatedResponse === null || page >= paginatedResponse.num_pages) return;
    page++;
  };
  const handlePrevPage = () => {
    if (page <= 1) return;
    page--;
  };

  let requestId = 0;
  $effect(() => {
    const currPage = page;
    const currPageSize = pageSize;
    const currSortColumn = sortColumn;
    const currSortDirection = sortDirection;

    const fetchTransactions = async () => {
      const currentRequest = ++requestId;
      try {
        const response = await transactionsApi.getPaginatedSortedTransactions(
          currPage,
          currPageSize,
          currSortColumn,
          currSortDirection
        );
        // If a user flips between pages rapidly this could cause multiple requests
        // to be in flight. To maintain a consistent state we must ignore older requests
        if (currentRequest !== requestId) {
          console.log("[WARN] Outdated request getting dropped", response);
          return;
        }
        paginatedResponse = response;
      } catch(e) {
        let error = e as string;
        console.error(error);
      }
    }

    fetchTransactions();
  })

  const getShowingRange = () => {
    if (paginatedResponse === null ) { return [0, 0]; }

    let rangeStart = (page - 1) * pageSize + 1;
    let rangeEnd = Math.min(page * pageSize, paginatedResponse.num_transactions); 

    return [rangeStart, rangeEnd];
  }

  const dateFormatter = new Intl.DateTimeFormat("en-US", { 
    month: "short", day: "numeric", year: "numeric" 
  });
  function formatTableDate(date: string | Date): string {
    // The backend serializes dates as "YYYY-MM-DD". Passing that straight to
    // `new Date(...)` parses it as UTC midnight, which renders the previous day
    // in negative-offset timezones. Build the date from its parts so it stays local.
    const [year, month, day] = String(date).split("-").map(Number);
    return dateFormatter.format(new Date(year, month - 1, day));
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
    <tfoot>
      <tr class="footer">
        {#if paginatedResponse !== null}
          <td colspan="2" class="col-showing">
            <p class="showing-text">
              Showing {getShowingRange()[0]} - {getShowingRange()[1]} out of {paginatedResponse.num_transactions}
            </p>
          </td>
          <td colspan="3" class="foot-controls">
            {#if paginatedResponse.prev_page !== null}
              <Icon icon="material-symbols:chevron-left" class="page-controls" width={24} onclick={handlePrevPage} />
            {/if}
            <p class="page-count">{page} of {paginatedResponse?.num_pages}</p>
            {#if paginatedResponse.next_page !== null}
              <Icon icon="material-symbols:chevron-right" class="page-controls" width={24} onclick={handleNextPage} />
            {/if}
          </td>
        {/if}
      </tr>
    </tfoot>
    <tbody>
      {#each transactions as transaction, index}
        <tr class={index % 2 === 0 ? "row-white" : "row-grey"}>
          <td class="col-date">{formatTableDate(transaction.transaction.date)}</td>
          <td class="col-account">
            <span class="ellipsis">{transaction.account_name}</span>
          </td>
          <td class="col-name">
            <span class="ellipsis">{transaction.transaction.name}</span>
          </td>
          <td class="col-category">
            <CategoryPill 
              name={transaction.category_name}
              icon={transaction.category_icon ?? undefined}
              color={transaction.category_color}
              textColor={index % 2 === 0 ? 'var(--pure-white)' : 'var(--grey-50)'}
            />
          </td>
          <td class="col-amount {isPositiveAmount(transaction.transaction.amount) ? 'positive' : 'negative'}">
            {formatSignedCurrencyChange(transaction.transaction.amount)}
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

  tfoot {
    width: 100%;
    border-top: 1px solid var(--grey-100);
  }

  :global(.page-controls) {
    cursor: pointer;
    padding: 4px;
    border-radius: 6px;
    box-sizing: content-box;
    transition: background-color 0.15s ease;
    user-select: none;
  }

  :global(.page-controls:hover) {
    background-color: var(--grey-100);
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

  .col-showing {
    padding: 12px 20px;
  }

  .showing-text {
    width: fit-content;
    margin: 0px;
    font-size: 14px;
    color: var(--grey-300);
  }

  .foot-controls {
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .page-count {
    width: fit-content;
    margin: 0px;
    padding: 0px 8px;
  }

  .positive {
    color: var(--profit-green);
  }

  .negative {
    color: var(--loss-red);
  }
</style>
