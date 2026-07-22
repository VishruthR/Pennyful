<!-- @component
  A slim-styled dropdown that shows the selected category as a CategoryPill and
  lets the user reassign it. Wraps the generic Combobox + CategoryPill.
-->

<script lang="ts">
  import Combobox from "$lib/components/Combobox.svelte";
  import CategoryPill from "$lib/components/CategoryPill.svelte";
  import type { Category, DropdownOption } from "$lib/types";

  interface Props {
    categories: Category[];
    value: string | null;
    onSelect: (categoryId: number) => void;
  }

  let { categories, value, onSelect }: Props = $props();

  const options = $derived<DropdownOption[]>(categories.map((c) => ({ value: String(c.id) })));
  const byId = $derived(new Map(categories.map((c) => [String(c.id), c])));
</script>

{#snippet categoryItem(option: DropdownOption)}
  {@const category = byId.get(option.value)}
  {#if category}
    <CategoryPill
      name={category.name}
      icon={category.icon ?? undefined}
      textColor={category.color}
    />
  {/if}
{/snippet}

<Combobox
  variant="slim"
  {options}
  {value}
  item={categoryItem}
  onSelect={(v) => {
    if (v !== null) onSelect(Number(v));
  }}
/>
