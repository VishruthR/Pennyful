<!-- @component
  A card wrapper for the SpendingPieChart that includes a title, subtitle, and border.
-->

<script lang="ts">
  import SpendingPieChart from "./SpendingPieChart.svelte";
  import { invoke } from '@tauri-apps/api/core';
  import type { CategoryDetails, Category } from "$lib/types.ts"

  export interface SpendingCategory {
    name: string;
    amount: number;
  }

  interface Props {
    categories: SpendingCategory[];
    title?: string;
    subtitle?: string;
    size?: number;
    strokeWidth?: number;
  }

  let {
    categories,
    title = "Spending Breakdown",
    subtitle = "Each color corresponds to a category. Hover over a color to see more details on your spending within that category.",
    size = 280,
    strokeWidth = 24,
  }: Props = $props();

  const fetchCategoryDetails = async () => {
    // TODO: This function doesn't work on first load, only on refresh
    let categoryDetails = new Map(Object.entries(await invoke('get_category_details'))) as CategoryDetails
    console.log("Fetched", categoryDetails);
    return categoryDetails;
  }
</script>

<div class="spending-breakdown">
  <h2 class="title h2">{title}</h2>
  <p class="subtitle paragraph">{subtitle}</p>
  <div class="chart-wrapper">
    {#await fetchCategoryDetails()}
      <p>Loading...</p>
    {:then categoryDetails}
        <SpendingPieChart {categories} {categoryDetails} {size} {strokeWidth} />
    {:catch error}
      <p>Error: {error}</p>
    {/await}
    
  </div>
</div>

<style>
  .spending-breakdown {
    border: 2px solid var(--grey-300);
    border-radius: 12px;
    padding: 24px;
    display: inline-block;
  }

  .title {
    color: var(--grey-500);
    margin: 0 0 8px 0;
    text-align: left;
  }

  .subtitle {
    color: var(--grey-300);
    margin: 0 0 24px 0;
    text-align: left;
    max-width: 320px;
  }

  .chart-wrapper {
    display: flex;
    justify-content: center;
  }
</style>
