<!-- @component
  Displays the manage-categories table: each row shows the category pill, an
  editable budget, the current-month amount spent (with percent of budget), and
  edit/delete actions. The "Uncategorized" fallback row has no actions.

  Shares the visual shell (container, sticky header, alternating rows, hover) with
  TransactionsTable but has its own columns and behavior.
-->

<script lang="ts">
  import type { CategoryOverview } from "$lib/types";
  import Icon from "@iconify/svelte";
  import CategoryPill from "$lib/components/CategoryPill.svelte";
  import BudgetInput from "$lib/components/BudgetInput.svelte";
  import { formatCentsAsDollars } from "$lib/utils/format";

  interface Props {
    categories: CategoryOverview[];
    onBudgetChange: (categoryId: number, amountCents: number) => void;
    onEdit: (category: CategoryOverview) => void;
    onDelete: (category: CategoryOverview) => void;
    height?: string;
  }

  let { categories, onBudgetChange, onEdit, onDelete, height }: Props = $props();

  let tableHeight = $derived(height ? height : "100%");

  const UNCATEGORIZED = "Uncategorized";

  function pillTextColor(color: string): string {
    // No secondary color is stored, so darken the category color for the label.
    return `color-mix(in srgb, ${color} 25%, black)`;
  }

  function hasBudget(category: CategoryOverview): boolean {
    return category.budget_cents !== null && category.budget_cents > 0;
  }

  function spentDisplay(category: CategoryOverview): string {
    const spent = formatCentsAsDollars(category.spent_cents);
    if (hasBudget(category)) {
      const percent = Math.round((category.spent_cents / category.budget_cents!) * 100);
      return `${spent} (${percent}%)`;
    }
    return spent;
  }

  function spentClass(category: CategoryOverview): string {
    if (!hasBudget(category)) return "spent-neutral";
    const ratio = category.spent_cents / category.budget_cents!;
    if (ratio >= 1) return "spent-over";
    if (ratio >= 0.9) return "spent-warning";
    return "spent-under";
  }
</script>

<div class="category-table-container" style:height={tableHeight}>
  <table class="category-table">
    <thead>
      <tr>
        <th class="col-category">Category</th>
        <th class="col-budget">Budget</th>
        <th class="col-spent">Amount Spent</th>
        <th class="col-actions">Actions</th>
      </tr>
    </thead>
    <tbody>
      {#each categories as category, index (category.id)}
        <tr class={index % 2 === 0 ? "row-white" : "row-grey"}>
          <td class="col-category">
            <CategoryPill
              name={category.name}
              icon={category.icon}
              color={category.color}
              textColor={pillTextColor(category.color)}
            />
          </td>
          <td class="col-budget">
            <BudgetInput
              budgetCents={category.budget_cents}
              onCommit={(cents) => onBudgetChange(category.id, cents)}
              slim
            />
          </td>
          <td class="col-spent">
            <span class={spentClass(category)}>{spentDisplay(category)}</span>
          </td>
          <td class="col-actions">
            {#if category.name !== UNCATEGORIZED}
              <div class="actions">
                <button
                  class="icon-button"
                  aria-label={`Edit ${category.name}`}
                  onclick={() => onEdit(category)}
                >
                  <Icon icon="mdi:pencil-outline" width={20} height={20} />
                </button>
                <button
                  class="icon-button"
                  aria-label={`Delete ${category.name}`}
                  onclick={() => onDelete(category)}
                >
                  <Icon icon="mdi:trash-can-outline" width={20} height={20} />
                </button>
              </div>
            {/if}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .category-table-container {
    width: 100%;
    border: 2px solid var(--grey-500);
    border-radius: 10px;
    overflow: auto;
  }

  .category-table {
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
    padding: 24px 20px;
    font-weight: bold;
    color: var(--grey-500);
    box-shadow: inset 0px 0px #000, 0 1px var(--grey-100);
  }

  th.col-category {
    text-align: left;
  }

  th.col-budget,
  th.col-spent {
    text-align: center;
  }

  th.col-actions {
    text-align: right;
  }

  tbody tr {
    transition: background-color 0.15s ease;
  }

  tbody tr:hover {
    background-color: var(--blue-50) !important;
  }

  .row-white {
    background-color: var(--pure-white);
  }

  .row-grey {
    background-color: var(--grey-50);
  }

  td {
    padding: 8px 20px;
    color: var(--grey-500);
  }

  td.col-budget,
  td.col-spent {
    text-align: center;
  }

  .col-category {
    width: 34%;
  }

  .col-budget {
    width: 20%;
  }

  .col-spent {
    width: 24%;
    white-space: nowrap;
  }

  .col-actions {
    width: 22%;
  }

  .actions {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 16px;
  }

  .icon-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    background: none;
    border: none;
    border-radius: 6px;
    color: var(--grey-500);
    cursor: pointer;
    transition: background-color 0.15s ease, color 0.15s ease;
  }

  .icon-button:hover {
    background-color: var(--grey-100);
  }

  .icon-button:focus-visible {
    outline: 2px solid var(--grey-500);
    outline-offset: 1px;
  }

  .spent-under {
    color: var(--profit-green);
  }

  .spent-warning {
    color: var(--warning-yellow);
  }

  .spent-over {
    color: var(--loss-red);
  }

  .spent-neutral {
    color: var(--grey-300);
  }
</style>
