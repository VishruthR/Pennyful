<script lang="ts">
  import MonthByMonthSpendingChart from "$lib/components/MonthByMonthSpendingChart.svelte";
  import TransactionsTable from "$lib/components/TransactionsTable.svelte";
  import TopCategories from "$lib/components/TopCategories.svelte";
  import type { PageProps } from "./$types";
  import { invalidate } from "$app/navigation";

  let { data }: PageProps = $props();

  const handleTransactionsLoad = async () => {
    await invalidate("home:transactions-table");
  };
</script>

<main class="container">
  <div class="stats-section">
    <MonthByMonthSpendingChart />
    <TopCategories categories={data.categories} />
  </div>
  <TransactionsTable onTransactionsLoad={handleTransactionsLoad} />
</main>

<style>
  .container {
    min-height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 32px;
    gap: 48px;
  }

  .stats-section {
    display: flex;
    width: 100%;
    gap: 25px;
  }
</style>
