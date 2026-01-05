<!-- @component
  Displays a list of spending categories with progress bars showing budget usage.
  Each category shows an icon, name, progress bar, and spending/budget amounts.

  Example data:
  const categories = [
    { name: "Restaurants", icon: "fluent:food-28-filled", spending: 555, budget: 600 },
    { name: "Healthcare", icon: "mdi:heart", spending: 84, budget: 400 },
    { name: "Transportation", icon: "bxs:car", spending: 400, budget: 600 },
  ];
-->

<script lang="ts">
  import Icon from "@iconify/svelte";

  interface SpendingCategory {
    name: string;
    icon: string;
    spending: number;
    budget?: number;
  }

  interface Props {
    categories: SpendingCategory[];
    title?: string;
    onSeeAll?: () => void;
  }

  let {
    categories,
    title = "Top categories",
    onSeeAll,
  }: Props = $props();

  function formatAmount(amount: number): string {
    return `$${amount.toLocaleString()}`;
  }

  function getProgress(category: SpendingCategory): number {
    if (!category.budget) return 100;
    return Math.min((category.spending / category.budget) * 100, 100);
  }

  function getProgressColor(category: SpendingCategory): string {
    const progress = getProgress(category);
    if (progress < 35) return "var(--profit-green)";
    if (progress <= 70) return "#F7B500";
    return "var(--loss-red)";
  }
</script>

<div class="top-categories">
  <header class="header">
    <h2 class="title h3">{title}</h2>
    <button class="see-all" onclick={onSeeAll}>
      see all categories <Icon icon="stash:chevron-right" width={16} height={16} />
    </button>
  </header>

  <div class="category-grid">
    <div class="column-labels">
      {#each categories as category}
        <div class="category-label">
          <div class="category-icon">
            <Icon icon={category.icon} width={20} height={20} />
          </div>
          <span class="category-name paragraph">{category.name}</span>
        </div>
      {/each}
    </div>

    <div class="column-progress">
      {#each categories as category}
        <div class="progress-bar-container">
          <div class="progress-bar">
            <div
              class="progress-fill"
              style:width="{getProgress(category)}%"
              style:background-color={getProgressColor(category)}
            ></div>
          </div>
        </div>
      {/each}
    </div>

    <div class="column-amounts">
      {#each categories as category}
        <span class="category-amount paragraph">
          {#if category.budget}
            {formatAmount(category.spending)} / {formatAmount(category.budget)}
          {:else}
            {formatAmount(category.spending)}
          {/if}
        </span>
      {/each}
    </div>
  </div>
</div>

<style>
  .top-categories {
    border: 2px solid var(--grey-300);
    border-radius: 12px;
    padding: 24px;
    width: 100%;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .title {
    color: var(--grey-500);
    margin: 0;
  }

  .see-all {
    display: flex;
    align-items: center;
    gap: 4px;
    background: none;
    border: none;
    color: var(--grey-300);
    font-size: 14px;
    cursor: pointer;
    padding: 0;
    font-family: inherit;
  }

  .see-all:hover {
    text-decoration: underline;
  }

  .category-grid {
    display: grid;
    grid-template-columns: auto 1fr auto;
    gap: 12px;
  }

  .column-labels,
  .column-progress,
  .column-amounts {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .column-labels {
    align-items: flex-start;
  }

  .column-progress {
    justify-content: center;
    width: 100%;
  }

  .column-amounts {
    align-items: flex-end;
  }

  .category-label {
    display: flex;
    align-items: center;
    gap: 12px;
    height: 20px;
  }

  .category-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--grey-300);
  }

  .category-name {
    color: var(--grey-500);
    white-space: nowrap;
  }

  .progress-bar-container {
    display: flex;
    height: 20px;
    width: 100%;
    align-items: center;
  }

  .progress-bar {
    height: 10px;
    background-color: var(--grey-100);
    border-radius: 5px;
    overflow: hidden;
    align-self: center;
    width: 100%;
  }

  .progress-fill {
    height: 100%;
    border-radius: 5px;
    transition: width 0.3s ease;
  }

  .category-amount {
    color: var(--grey-500);
    white-space: nowrap;
    height: 20px;
    display: flex;
    align-items: center;
  }
</style>
