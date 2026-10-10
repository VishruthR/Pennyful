<!-- @component
  A slim-styled dropdown that shows the selected category as a CategoryPill and
  lets the user reassign it. Wraps the generic Combobox + CategoryPill.
-->

<script lang="ts">
  import Combobox from "$lib/components/Combobox.svelte";
  import CategoryPill from "$lib/components/CategoryPill.svelte";
  import type { Category } from "$lib/types/category";
  import type { DropdownOption } from "$lib/types/dropdown_option";

  interface Props {
    categories: Category[];
    value: number | null;
    onSelect: (categoryId: number) => void;
  }

  let { categories, value, onSelect }: Props = $props();

  // bits-ui Combobox only works with Strings
  const options = $derived<DropdownOption[]>(
    categories.map((c) => ({ value: String(c.id) })),
  );
  const byId = $derived(new Map(categories.map((c) => [String(c.id), c])));
</script>

{#snippet categoryItem(option: DropdownOption)}
  {@const category = byId.get(String(option.value))}
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
  value={value !== null ? String(value) : null}
  item={categoryItem}
  onSelect={(v) => {
    if (v !== null) onSelect(Number(v));
  }}
/>
