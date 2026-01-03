<!-- @component
  A card wrapper for the SpendingPieChart that includes a title, subtitle, and border.
-->

<script lang="ts">
  import SpendingPieChart from "./SpendingPieChart.svelte";

  interface SpendingCategory {
    name: string;
    color: string;
    icon: string;
    iconColor: string;
    amount: number;
  }

  interface Props {
    categories: SpendingCategory[];
    title?: string;
    subtitle?: string;
    size?: number;
    strokeWidth?: number;
    currency?: string;
  }

  let {
    categories,
    title = "Spending Breakdown",
    subtitle = "Each color corresponds to a category. Hover over a color to see more details on your spending within that category.",
    size = 280,
    strokeWidth = 24,
    currency = "$",
  }: Props = $props();
</script>

<div class="spending-breakdown">
  <h2 class="title">{title}</h2>
  <p class="subtitle">{subtitle}</p>
  <div class="chart-wrapper">
    <SpendingPieChart {categories} {size} {strokeWidth} {currency} />
  </div>
</div>

<style>
  .spending-breakdown {
    border: 2px solid #535353;
    border-radius: 12px;
    padding: 1.5rem;
    display: inline-block;
  }

  .title {
    color: #404040;
    font-size: 1.5rem;
    font-weight: 700;
    margin: 0 0 0.5rem 0;
    text-align: left;
  }

  .subtitle {
    color: #535353;
    font-size: 0.95rem;
    font-weight: 400;
    margin: 0 0 1.5rem 0;
    text-align: left;
    line-height: 1.4;
    max-width: 320px;
  }

  .chart-wrapper {
    display: flex;
    justify-content: center;
  }
</style>

