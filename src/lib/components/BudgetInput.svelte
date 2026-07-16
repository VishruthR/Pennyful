<!-- @component
  A compact, editable currency input for a category's budget.
  Displays a formatted whole-dollar amount (e.g. "$500") when idle and the raw
  editable value while focused. Commits on blur or Enter; Escape cancels.
-->

<script lang="ts">
  import { formatCentsAsDollars, parseDollarsToCents } from "$lib/utils/format";

  interface Props {
    budgetCents: number | null;
    onCommit: (amountCents: number) => void;
    slim?: boolean;
  }

  let { budgetCents, onCommit, slim = false }: Props = $props();

  let editing = $state(false);
  let text = $state("");

  const display = $derived(budgetCents === null ? "" : formatCentsAsDollars(budgetCents));

  function startEdit() {
    editing = true;
    text = budgetCents === null ? "" : String(Math.round(budgetCents / 100));
  }

  function commit() {
    editing = false;
    const cents = parseDollarsToCents(text);
    if (cents !== null && cents !== budgetCents) {
      onCommit(cents);
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    const target = event.currentTarget as HTMLInputElement;
    if (event.key === "Enter") {
      target.blur();
    } else if (event.key === "Escape") {
      editing = false;
      target.blur();
    }
  }
</script>

<input
  class="budget-input paragraph"
  inputmode="decimal"
  placeholder="$0"
  class:slim={slim}
  value={editing ? text : display}
  oninput={(e) => (text = e.currentTarget.value)}
  onfocus={startEdit}
  onblur={commit}
  onkeydown={handleKeydown}
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
