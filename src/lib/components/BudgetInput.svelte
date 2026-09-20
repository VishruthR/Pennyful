<!-- @component
A compact, editable currency input for a category's budget.
Displays a formatted whole-dollar amount (e.g. "$500").
Commits on blur or Enter; Escape cancels.
-->

<script lang="ts">
  import { formatDollars } from "$lib/utils/format";
  import { parseNumericString } from "$lib/utils/parse";

  interface Props {
    id?: string;
    budget: number | null;
    onCommit: (amount: number | null) => void;
    slim?: boolean;
    maxWidth?: string;
  }

  let { id, budget = null, onCommit, slim = false, maxWidth }: Props = $props();

  let value = $derived<number | null>(budget);

  // TODO: Support budgets with cents values
  const formattedBudget = {
    get() {
      return value !== null ? formatDollars(value) : "";
    },
    set(displayValue: string) {
      const nullLength = displayValue.startsWith("$") ? 1 : 0;
      if (displayValue.length <= nullLength) {
        value = null;
        return;
      }
      value = parseNumericString(displayValue.slice(nullLength));
      if (Number.isNaN(value)) {
        value = null;
      }
    },
  };

  const commit = () => {
    onCommit(value);
  };

  const handleKeyDown = (event: KeyboardEvent) => {
    const target = event.currentTarget as HTMLInputElement;
    if (event.key === "Enter") {
      target.blur();
    } else if (event.key === "Escape") {
      value = budget;
      target.blur();
    }
  };
</script>

<input
  {id}
  class="budget-input paragraph"
  inputmode="decimal"
  placeholder="--"
  class:slim
  style:max-width={maxWidth}
  bind:value={formattedBudget.get, formattedBudget.set}
  onblur={commit}
  onkeydown={handleKeyDown}
/>

<style>
  .budget-input {
    width: 100%;
    padding: 10px 12px;
    text-align: center;
    border: 1.5px solid var(--grey-100);
    border-radius: 10px;
    background-color: var(--pure-white);
    color: var(--grey-500);
    font-family: var(--font-family);
    transition: border-color 0.15s ease;
  }

  .slim {
    padding: 4px 12px;
  }

  .budget-input::placeholder {
    color: var(--grey-200);
  }

  .budget-input:hover {
    border-color: var(--grey-200);
  }

  .budget-input:focus {
    outline: none;
    border-color: var(--grey-300);
  }
</style>
