<!-- @component
A compact, editable currency input for a category's budget.
Displays a formatted whole-dollar amount (e.g. "$500").
Commits on blur or Enter; Escape cancels.
-->

<script lang="ts">
  interface Props {
    budget: number | null;
    onCommit: (amount: number | null) => void;
    slim?: boolean;
  }

  let { budget = null, onCommit, slim = false }: Props = $props();

  // This `value` is expected to be updated so we want to avoid binding it to `budget`.
  // svelte-ignore state_referenced_locally
  let value = $state<number | null>(budget);

  const formattedBudget = {
    get() {
      console.log("get", value);
      return value !== null ? `$${value}` : "";
    },
    set(displayValue: string) {
      const nullLength = displayValue.startsWith("$") ? 1 : 0;
      if (displayValue.length <= nullLength) {
        value = null;
        return;
      }
      value = parseInt(displayValue.slice(nullLength));
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
  class="budget-input paragraph"
  inputmode="decimal"
  placeholder="$0"
  class:slim
  bind:value={formattedBudget.get, formattedBudget.set}
  onblur={commit}
  onkeydown={handleKeyDown}
/>

<style>
  .budget-input {
    width: 96px;
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
